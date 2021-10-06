use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "A utility for querying information about qcow files")]
pub struct Args {
    pub qcow: PathBuf,

    #[structopt(subcommand)]
    pub command: SubCommand,
}

#[derive(StructOpt)]
pub enum SubCommand {
    #[structopt(about = "Output info about the given qcow")]
    Info,

    #[structopt(about = "Display a tree listing of the contents of the qcow")]
    Tree {
        #[structopt(default_value = "/")]
        dir: String,

        #[structopt(
            long,
            help = "The number of files to show per directory",
            default_value = "10"
        )]
        file_limit: usize,

        #[structopt(long, help = "Don't limit the number of files to show per directory")]
        no_file_limit: bool,

        #[structopt(
            short,
            long,
            help = "The maximum number of times to recurse directories",
            default_value = "16"
        )]
        depth_limit: usize,

        #[structopt(long, help = "Don't limit the number of directories to recurse")]
        no_depth_limit: bool,
    },

    #[structopt(about = "Display a list of partitions in the qcow image")]
    Partitions,

    #[structopt(about = "Output a file within the qcow to stdout")]
    GetFile {
        #[structopt(help = "Path of the file to output")]
        file: String,

        #[structopt(long, help = "Disable using less as a pager")]
        no_page: bool,

        #[structopt(
            short = "-ff",
            long,
            help = "Force fancy output even while piping to another program"
        )]
        force_fancy: bool,

        #[structopt(short, long, help = "Language to syntax highlight as")]
        language: Option<String>,
    },
}
