use crate::*;

impl Qcow2 {
    /// Get the size of a cluster in bytes from the qcow
    pub fn cluster_size(&self) -> u64 {
        self.header.cluster_size()
    }
}

impl QcowHeader {
    /// Get the size of a cluster in bytes from the qcow
    pub fn cluster_size(&self) -> u64 {
        1 << self.cluster_bits
    }
}

impl Qcow1 {
    /// Get the size of a cluster in bytes from the qcow
    pub fn cluster_size(&self) -> u64 {
        self.header.cluster_size()
    }
}

impl v1::Qcow1Header {
    /// Get the size of a cluster in bytes from the qcow
    pub fn cluster_size(&self) -> u64 {
        1 << self.cluster_bits
    }
}
