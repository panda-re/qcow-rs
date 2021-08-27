use super::*;

/// An enum representing the different types of features
#[derive_binread]
#[derive(Debug)]
#[br(repr(u8))]
pub enum FeatureKind {
    /// Feature belongs to the backwards incompatible feature set
    IncompatibleFeature = 0,

    /// Feature belongs to the backwards compatible feature set
    CompatibleFeatures = 1,

    /// Feature belogns to the autocleared features
    AutoClearFeatures = 2,
}

/// A struct representing a feature/name pair
#[derive_binread]
#[derive(Debug)]
pub struct FeatureName {
    /// The type of feature being named
    pub kind: FeatureKind,

    /// The bit number within the feature being named, with 0 being the least significant bit
    pub bit_number: u8,

    #[br(temp, count = 0x2e)]
    feature_name_bytes: Vec<u8>,

    /// The name of the feature pointed to by the feature kind and the bit number
    #[br(calc = {
        feature_name_bytes.retain(|&x| x != 0);
        String::from_utf8_lossy(&feature_name_bytes).into_owned()
    })]
    pub feature_name: String,
}
