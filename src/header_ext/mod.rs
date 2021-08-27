use crate::*;

mod feature_name;
pub use feature_name::*;

/// An extension to the header allowing additional data to be included.
#[derive_binread]
#[derive(Debug)]
pub enum HeaderExt {
    /// This header extension marks the end of the header extension area
    #[br(magic = 0_u32)]
    End,

    /// A table of names for features provided by incompatible/compatible features
    #[br(magic = 0x6803f857_u32)]
    FeatureNameTable (
        #[br(temp)]
        u32,

        #[br(align_after = 8, count = self_0 / 0x30)]
        Vec<FeatureName>,
    ),

    /// A string describing the name of the backing file format
    #[br(magic = 0xe2792aca_u32)]
    BackingFileFormat (
        #[br(temp)]
        u32,

        #[br(temp, count = self_0)]
        Vec<u8>,

        #[br(calc = {
            self_1.retain(|&x| x != 0);
            String::from_utf8_lossy(&self_1).into_owned()
        })]
        String,
    ),

    /// Path to external data file in the form of a string
    #[br(magic = 0x44415441_u32)]
    ExternalDataPath (
        #[br(temp)]
        u32,

        #[br(temp, count = self_0)]
        Vec<u8>,

        #[br(calc = {
            self_1.retain(|&x| x != 0);
            String::from_utf8_lossy(&self_1).into_owned()
        })]
        String,
    ),

    /// A feature for which this crate does not implement a parser for
    Unparsed {
        /// The type of the header extension
        kind: HeaderExtKind,

        #[br(temp)]
        data_len: u32,

        /// The data corresponding to this feature
        #[br(align_after = 8, count = data_len)]
        data: Vec<u8>
    },
}

impl HeaderExt {
    pub(crate) fn is_end(&self) -> bool {
        matches!(self, Self::End)
    }
}

/// The type of data provided by the given header extension
///
/// Header extension type:
///
/// * 0x00000000 - End of the header extension area
/// * 0xe2792aca - Backing file format name string
/// * 0x6803f857 - Feature name table
/// * 0x23852875 - Bitmaps extension
/// * 0x0537be77 - Full disk encryption header pointer
/// * 0x44415441 - External data file name string
/// * other      - Unknown header extension, can be safely ignored
#[derive_binread]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HeaderExtKind {
    /// This header extension marks the end of the header extension area
    #[br(magic = 0_u32)]
    End,

    /// A table of names for features provided by incompatible/compatible features
    #[br(magic = 0x6803f857_u32)]
    FeatureNameTable,

    /// A string describing the name of the backing file format
    #[br(magic = 0xe2792aca_u32)]
    BackingFileFormat,

    /// Bitmaps extension
    #[br(magic = 0x23852875_u32)]
    BitmapsExtension,

    /// Extension is a pointer to the full disk encryption header
    #[br(magic = 0x0537be77_u32)]
    FullDiskEncryption,

    /// Path to external data file in the form of a string
    #[br(magic = 0x44415441_u32)]
    ExternalDataPath,

    /// A type of header extension unrecognized by this crate, possibly from the future!
    Other(u32),
}
