use crate::*;
use crate::header_ext::HeaderExt;

/// Top-level header of Qcow format
#[derive_binread]
#[derive(Debug)]
#[br(magic = b"QFI\xfb")]
pub struct QcowHeader {
    /// Version of the QCOW format. Only version 2 or 3 is supported, future formats will throw
    /// a parsing error upon being read.
    #[br(assert(version == 2 || version == 3))]
    pub version: u32,

    /// Offset into the image file at which the backing file name
    /// is stored (NB: The string is not null terminated). 0 if the
    /// image doesn't have a backing file.
    /// 
    /// **Note**: backing files are incompatible with raw external data
    /// files (auto-clear feature bit 1).
    #[br(temp)]
    backing_file_offset: u64,

    /// Length of the backing file name in bytes. Must not be
    /// longer than 1023 bytes. Undefined if the image doesn't have
    /// a backing file.
    #[br(temp)]
    backing_file_size: u32,

    #[br(
        restore_position, temp, parse_with = read_string, count = backing_file_size,
        args(backing_file_offset)
    )]
    backing_file_offset: FileString,

    /// Backing file represented as a string, if any.
    #[br(calc = backing_file_offset.0.clone())]
    pub backing_file: Option<String>,

    /// Number of bits that are used for addressing an offset
    /// within a cluster (1 << cluster_bits is the cluster size).
    /// Must not be less than 9 (i.e. 512 byte clusters).
    ///
    /// **Note**: qemu as of today has an implementation limit of 2 MB
    /// as the maximum cluster size and won't be able to open images
    /// with larger cluster sizes.
    /// 
    /// **Note**: if the image has Extended L2 Entries then cluster_bits
    /// must be at least 14 (i.e. 16384 byte clusters).
    pub cluster_bits: u32,

    /// Virtual disk size in bytes.
    ///
    /// **Note**: qemu has an implementation limit of 32 MB as
    /// the maximum L1 table size.  With a 2 MB cluster
    /// size, it is unable to populate a virtual cluster
    /// beyond 2 EB (61 bits); with a 512 byte cluster
    /// size, it is unable to populate a virtual size
    /// larger than 128 GB (37 bits).  Meanwhile, L1/L2
    /// table layouts limit an image to no more than 64 PB
    /// (56 bits) of populated clusters, and an image may
    /// hit other limits first (such as a file system's
    /// maximum size).
    pub size: u64,

    /// Encryption method to use for contents
    pub crypt_method: EncryptionMethod,

    /// Number of entries in the active L1 table
    pub l1_size: u32,

    /// Offset into the image file at which the active L1 table
    /// starts. Must be aligned to a cluster boundary.
    pub l1_table_offset: u64,

    /// Offset into the image file at which the refcount table
    /// starts. Must be aligned to a cluster boundary.
    pub refcount_table_offset: u64,

    /// Number of clusters that the refcount table occupies
    pub refcount_table_clusters: u32,

    /// Number of snapshots contained in the image
    pub(crate) nb_snapshots: u32,

    /// Offset into the image file at which the snapshot table
    /// starts. Must be aligned to a cluster boundary.
    pub(crate) snapshots_offset: u64,

    /// Part of header only present in qcow version 3, otherwise set to `None`
    #[br(align_after = 8, if(version == 3))]
    pub v3_header: Option<Version3Header>,

    /// Extentions to the header format
    #[br(parse_with = until_exclusive(|ext: &HeaderExt| ext.is_end()))]
    pub extensions: Vec<HeaderExt>,
}

/// Part of header only present in Qcow version 3
#[derive_binread]
#[derive(Debug)]
pub struct Version3Header {
    /// Bitmask of incompatible features. An implementation must fail to open an image if an
    /// unknown bit is set.
    pub incompatible_features: IncompatibleFeatures,

    /// Bitmask of compatible features. An implementation can safely ignore any unknown bits
    /// that are set.
    pub compatible_features: CompatibleFeatures,

    /// Bitmask of auto-clear features. An implementation may only write to an image with unknown
    /// auto-clear features if it clears the respective bits from this field first.
    pub autoclear_features: AutoClearFeatures,

    /// Describes the width of a reference count block entry (width
    /// in bits: refcount_bits = 1 << refcount_order). For version 2
    /// images, the order is always assumed to be 4
    /// (i.e. refcount_bits = 16).
    /// This value may not exceed 6 (i.e. refcount_bits = 64).
    pub refcount_order: u32,

    #[br(temp)]
    header_len: u32,

    /// Defines the compression method used for compressed clusters.
    ///
    /// All compressed clusters in an image use the same compression
    /// type.
    ///
    /// If the incompatible bit "Compression type" is set: the field
    /// must be present and non-zero (which means non-zlib
    /// compression type). Otherwise, this field must not be present
    /// or must be zero (which means zlib).
    #[br(if(header_len > 104 && incompatible_features.has_compression_type()))]
    pub compression_type: CompressionType,
}

/// Encryption method (if any) to use for image contents.
#[derive(BinRead, Debug, Clone, Copy, PartialEq, Eq)]
#[br(repr(u32))]
pub enum EncryptionMethod {
    /// No encryption is being used. This is the default.
    None = 0,

    /// Cluster contents are AES encrypted
    Aes = 1,

    /// Uses LUKS from drive encryption
    Luks = 2,
}

/// Compression type used for compressed clusters.
#[derive(BinRead, Debug, Clone, Copy, PartialEq, Eq)]
#[br(repr(u8))]
pub enum CompressionType {
    /// Uses flate/zlib compression for any clusters which are compressed
    Zlib = 0,

    /// Uses zstandard compression for any clusters which are compressed
    Zstd = 1,
}

impl Default for CompressionType {
    fn default() -> Self {
        Self::Zlib
    }
}

#[derive(BinRead)]
#[br(import(_offset: u64,))]
pub(crate) struct FileString(#[br(ignore)] pub(crate) Option<String>);

pub(crate) fn read_string<R>(mut reader: &mut R, ro: &ReadOptions, (offset,): (u64,)) -> BinResult<FileString>
    where R: Read + Seek,
{
    if offset == 0 {
        Ok(FileString(None))
    } else {
        reader.seek(binread::io::SeekFrom::Start(offset))?;

        let data: Vec<u8> = BinRead::read_options(&mut reader, ro, ())?;
        Ok(FileString(Some(String::from_utf8_lossy(&data).into_owned())))
    }
}
