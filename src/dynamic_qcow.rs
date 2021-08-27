use crate::{Qcow1, Qcow2, Snapshot};

/// An enum representing a qcow of any version
#[derive(Debug)]
pub enum DynamicQcow {
    /// Legacy version 1 qcow
    Qcow1(Qcow1),

    /// A qcow of version >= 2
    Qcow2(Qcow2),
}

impl DynamicQcow {
    /// Get the list of snapshots present within the qcow
    pub fn snapshots(&self) -> &[Snapshot] {
        #[allow(unreachable_patterns)]
        match self {
            Self::Qcow2(qcow) => &qcow.snapshots,
            _ => &[],
        }
    }

    /// Get the version of the qcow file
    pub fn version(&self) -> u32 {
        match self {
            Self::Qcow2(qcow) => qcow.header.version,
            Self::Qcow1(_) => 1,
        }
    }

    /// Get the size of a cluster in bytes from the qcow
    pub fn cluster_size(&self) -> u64 {
        match self {
            Self::Qcow2(qcow) => qcow.cluster_size(),
            Self::Qcow1(qcow) => qcow.cluster_size(),
        }
    }

    /// Gets the string representing the backing file of this qcow, if any.
    pub fn backing_file(&self) -> Option<String> {
        match self {
            Self::Qcow2(qcow) => qcow.header.backing_file.clone(),
            Self::Qcow1(qcow) => qcow.header.backing_file.clone(),
        }
    }

    /// Unwrap the qcow into a version 1 qcow, panicking if the qcow is not version 1.
    #[track_caller]
    pub fn unwrap_qcow1(self) -> Qcow1 {
        match self {
            Self::Qcow1(qcow) => qcow,
            _ => panic!("Expected a version 1 qcow"),
        }
    }

    /// Unwrap the qcow as a qcow2, representing a version 2 or higher qcow, panicking if
    /// the qcow is not version 2+.
    #[track_caller]
    pub fn unwrap_qcow2(self) -> Qcow2 {
        #[allow(unreachable_patterns)]
        match self {
            Self::Qcow2(qcow) => qcow,
            _ => panic!("Expected a version 2+ qcow"),
        }
    }
}
