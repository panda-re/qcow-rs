use binread::derive_binread;
use crate::EncryptionMethod;
use crate::header::{read_string, FileString};

/// Header for qcow version 1 format
#[derive_binread]
#[derive(Debug)]
#[br(magic = b"QFI\xfb")]
pub struct Qcow1Header {
    /// Version of the QCOW format. Must be 1.
    #[br(assert(version == 1))]
    pub version: u32,

    #[br(temp)]
    backing_file_offset: u64,

    #[br(temp)]
    backing_file_size: u32,

    #[br(
        restore_position, temp, parse_with = read_string, count = backing_file_size,
        args(backing_file_offset)
    )]
    backing_file_offset: FileString,

    /// A string representing the backing file, if any.
    #[br(calc = backing_file_offset.0.clone())]
    pub backing_file: Option<String>,

    /// Modification time of the image
    pub mtime: u32,

    /// Size of the virtual hard disk
    pub size: u64,

    /// Number of bits used to represent the offset within the cluster.
    ///
    /// The cluster size can be retrivied from (1 << cluster_bits)
    pub cluster_bits: u8,

    /// Number of bits used to index into the L2 lookup table
    pub l2_bits: u8,

    #[br(temp)]
    padding: [u8; 2],

    /// Encryption method used to encrypt the contents of clusters
    pub crypt_method: EncryptionMethod,

    /// Offset of L1 table used to lookup L2 table offsets
    pub l1_table_offset: u64,
}
