use crate::*;

fn reverse<T>(func: impl Fn([u8; 8]) -> T) -> impl Fn([u8; 8]) -> T {
    move |mut bytes| {
        bytes.reverse();
        func(bytes)
    }
}

/// Bitmask of incompatible features. An implementation must
/// fail to open an image if an unknown bit is set.
#[bitfield(bits = 64)]
#[derive(BinRead, Debug)]
#[br(map = reverse(Self::from_bytes))]
pub struct IncompatibleFeatures {
    /// Dirty bit.  If this bit is set then refcounts may be inconsistent, make sure to scan L1/L2
    /// tables to repair refcounts before accessing the image.
    pub dirty: bool,

    /// Corrupt bit.  If this bit is set then any data structure may be corrupt and the image must
    /// not be written to (unless for regaining consistency).
    pub corrupt: bool,

    /// External data file bit.  If this bit is set, an external data file is used. Guest clusters
    /// are then stored in the external data file. For such images, clusters in the external data
    /// file are not refcounted. The offset field in the Standard Cluster Descriptor must match the
    /// guest offset and neither compressed clusters nor internal snapshots are supported.
    ///
    /// An External Data File Name header extension may be present if this bit is set.
    pub external_data_file: bool,

    /// Compression type bit.  If this bit is set, a non-default compression is used for compressed
    /// clusters. The compression_type field must be present and not zero.
    pub has_compression_type: bool,

    /// Extended L2 Entries.  If this bit is set then L2 table entries use an extended format that
    /// allows subcluster-based allocation. See the Extended L2 Entries section for more details.
    pub extended_l2: bool,

    #[skip] __: B59,
}

/// Bitmask of compatible features. An implementation can
/// safely ignore any unknown bits that are set.
#[bitfield(bits = 64)]
#[derive(BinRead, Debug)]
#[br(map = reverse(Self::from_bytes))]
pub struct CompatibleFeatures {
    /// Lazy refcounts bit.  If this bit is set then lazy refcount updates can be used.  This means
    /// marking the image file dirty and postponing refcount metadata updates.
    pub lazy_refcount: bool,

    #[skip] __: B63,
}

/// Bitmask of auto-clear features. An implementation may only
/// write to an image with unknown auto-clear features if it
/// clears the respective bits from this field first.
#[bitfield(bits = 64)]
#[derive(BinRead, Debug)]
#[br(map = reverse(Self::from_bytes))]
pub struct AutoClearFeatures {
    /// Bitmaps extension bit
    ///
    /// This bit indicates consistency for the bitmaps
    /// extension data.
    ///
    /// It is an error if this bit is set without the
    /// bitmaps extension present.
    ///
    /// If the bitmaps extension is present but this
    /// bit is unset, the bitmaps extension data must be
    /// considered inconsistent.
    pub bitmap_extension: bool,

    /// Raw external data bit
    ///
    /// If this bit is set, the external data file can
    /// be read as a consistent standalone raw image
    /// without looking at the qcow2 metadata.

    /// Setting this bit has a performance impact for
    /// some operations on the image (e.g. writing
    /// zeros requires writing to the data file instead
    /// of only setting the zero flag in the L2 table
    /// entry) and conflicts with backing files.

    /// This bit may only be set if the External Data
    /// File bit (incompatible feature bit 1) is also
    /// set.
    pub raw_external_data: bool,
    #[skip] __: B62,
}
