mod args;
pub use args::{Args, SubCommand};

mod output;
pub use output::ReadAtAdapter;
pub use output::{output_file, output_info, output_partitions, output_tree, FileCfg, TreeLimits};

pub use {bootsector, ext4, gpt_partition_type, humansize, positioned_io, qcow};

use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};

pub fn main(args: Args) {
    let mut file = BufReader::new(File::open(&args.qcow).unwrap());
    let qcow = qcow::load(&mut file)
        .expect("Failed to parse qcow")
        .unwrap_qcow2();
    let mut reader = qcow.reader(&mut file);

    macro_rules! get_superblock {
        ($superblock:ident) => {
            let partitons = bootsector::list_partitions(&mut reader, &Default::default()).unwrap();
            reader.seek(SeekFrom::Start(0)).unwrap();

            let partition_reader = bootsector::open_partition(&mut reader, &partitons[0]).unwrap();
            let mut reader = ReadAtAdapter::new(partition_reader);
            let $superblock = ext4::SuperBlock::new(&mut reader).unwrap();
        };
    }

    #[cfg(target_family = "unix")]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    match args.command {
        SubCommand::Info => output_info(&qcow),
        SubCommand::Partitions => output_partitions(&mut reader),
        SubCommand::Tree {
            dir,
            file_limit,
            depth_limit,
            no_file_limit,
            no_depth_limit,
        } => {
            get_superblock!(superblock);
            output_tree(
                &superblock,
                &dir,
                TreeLimits {
                    files: if no_file_limit {
                        usize::MAX
                    } else {
                        file_limit
                    },
                    depth: if no_depth_limit {
                        usize::MAX
                    } else {
                        depth_limit
                    },
                },
            )
        }
        SubCommand::GetFile {
            file,
            no_page,
            force_fancy,
            language,
        } => {
            get_superblock!(superblock);
            output_file(
                &superblock,
                &file,
                FileCfg {
                    no_page,
                    force_fancy,
                    language,
                },
            )
        }
    }
}
