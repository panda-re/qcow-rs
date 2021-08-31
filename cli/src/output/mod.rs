mod tree;
mod file;
mod info;
mod partitions;

pub use {
    info::output_info,
    tree::{output_tree, TreeLimits},
    file::{output_file, FileCfg},
    partitions::output_partitions,
};

use std::io::{Read, Seek, SeekFrom};
use std::sync::Mutex;

use bootsector::Attributes;
use ext4::{DirEntry, Enhanced, Inode};
use syntect::parsing::{SyntaxDefinition, SyntaxSet};
use gpt_partition_type::{PartitionDescription, PartitionTypeGuid};
use humansize::{file_size_opts as opts, FileSize};
use owo_colors::OwoColorize;

use tabled::{
    style::Style, Alignment, Disable, Format, Full, Head, Header, Indent, Modify, Row,
    Table, Tabled,
};

pub struct ReadAtAdapter<R>
where
    R: Read + Seek,
{
    inner: Mutex<R>,
}

impl<R: Read + Seek> ReadAtAdapter<R> {
    pub fn new(reader: R) -> Self {
        Self {
            inner: Mutex::new(reader),
        }
    }

    pub fn into_inner(self) -> R {
        let Self { inner } = self;

        inner.into_inner().unwrap()
    }
}

use positioned_io::ReadAt;

impl<R: Read + Seek> ReadAt for ReadAtAdapter<R> {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut reader = self.inner.lock().unwrap();
        reader.seek(SeekFrom::Start(pos))?;

        reader.read(buf)
    }
}
