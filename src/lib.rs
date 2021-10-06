//! A library for working with qcow (version 1, 2, and 3) files.
//!
//! ## Example
//!
//! ```rust
//! # const PATH: &str = "/home/jamcleod/.panda/bionic-server-cloudimg-amd64-noaslr-nokaslr.qcow2";
//! # use std::fs::File;
//! # use std::io::{Read, BufReader};
//! // open qcow
//! let qcow = qcow::open(PATH).unwrap();
//!
//! // print out list of snapshots in the qcow
//! for snapshot in qcow.snapshots() {
//!     println!(
//!         "Snapshot {:?}: {:?} (size = {})",
//!         snapshot.unique_id,
//!         snapshot.name,
//!         snapshot.vm_state_size
//!     );
//! }
//!
//! // create a reader for accessing the virtual hard disk
//! let mut file = BufReader::new(File::open(PATH)?);
//! let qcow2 = qcow.unwrap_qcow2();
//! let mut reader = qcow2.reader(&mut file);
//!
//! // read the first 10 bytes of the virtual hard disk
//! let mut buf = [0; 10];
//! reader.read_exact(&mut buf)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Important types/functions
//!
//! * Retrieving a qcow - [`open`] (from path), [`load`] (from reader), [`load_from_memory`] (from
//! slice)
//! * Converting to qcow2 - [`DynamicQcow::unwrap_qcow2`]
//! * Reading from a virtual hard disk - [`Qcow2::reader`] (returns [`Reader`], which implements
//! [`Read`](std::io::Read) + [`Seek`](std::io::Seek))
//!
//! ## Features
//!
//! * Parse qcow files
//! * Full qcow version 1 support
//!   * Support for parsing the header and some associated data
//! * Full qcow version 2-3 support
//!   * Header parsing, including extra version 3 header data
//!   * Header extension parsing, allowing you to use addition data they provide
//!   * Lookup table (L1 and L2) parsing, only loading L2 tables on demand
//!   * Snapshot parsing, including snapshot L1 lookup tables
//!   * Support for reading the contents of the virtual disk
//!     * Includes compression support (for both zlib and zstd)
//!     * Cluster lookup caching, backtracking on cache miss
//!     * Allows arbitrary seeking within the guest
#![warn(missing_docs)]
use binread::{
    derive_binread,
    io::{Read, Seek, SeekFrom},
    until_exclusive, BinRead, BinReaderExt, BinResult, ReadOptions,
};
use modular_bitfield::prelude::*;

use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::Path;

mod methods;
mod reader;
pub use reader::*;

mod error;
pub use error::Error;

mod header;
pub use header::*;

/// Module for types pertaining to QCOW header extensions
pub mod header_ext;

pub mod levels;
use levels::*;

/// Module containing structs specific to the legacy QCOW v1 format
pub mod v1;
use v1::Qcow1Header;

mod snapshots;
pub use snapshots::*;

mod features;
pub use features::*;

mod dynamic_qcow;
pub use dynamic_qcow::DynamicQcow;

/// Parsed representation of a qcow2 file.
///
/// Can be aquired by using one of:
///
/// * [`open`]
/// * [`load`]
/// * [`load_from_memory`]
///
/// and then using [`DynamicQcow::unwrap_qcow2`].
///
/// ## Example
///
/// ```rust
/// # const PATH: &str = "/home/jamcleod/.panda/bionic-server-cloudimg-amd64-noaslr-nokaslr.qcow2";
/// let qcow = qcow::open(PATH).unwrap();
/// ```
#[derive(BinRead, Debug)]
#[br(big)]
pub struct Qcow2 {
    /// Header of the qcow as parsed from the file, contains top-level data about the qcow
    pub header: QcowHeader,

    /// List of snapshots present within this qcow
    #[br(seek_before = SeekFrom::Start(header.snapshots_offset), count = header.nb_snapshots)]
    pub snapshots: Vec<Snapshot>,

    /// Active table of [`L1Entry`]s used for handling lookups of contents
    #[br(seek_before = SeekFrom::Start(header.l1_table_offset), count = header.l1_size)]
    pub l1_table: Vec<L1Entry>,
}

/// Parsed representation of a v1 qcow file (legacy)
#[derive(BinRead, Debug)]
#[br(big)]
pub struct Qcow1 {
    /// Header of the qcow as parsed from the file
    pub header: Qcow1Header,
}

#[derive(BinRead, Debug)]
#[br(big, magic = b"QFI\xfb")]
struct QcowVersion(u32);

/// Open a qcow or qcow2 file from a path
///
/// ## Example
///
/// ```rust
/// # const PATH: &str = "/home/jamcleod/.panda/bionic-server-cloudimg-amd64-noaslr-nokaslr.qcow2";
/// let qcow = qcow::open(PATH)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn open(path: impl AsRef<Path>) -> Result<DynamicQcow, Error> {
    let path = path.as_ref();
    let mut file = BufReader::new(File::open(path).map_err(Error::FileNotFound)?);

    load(&mut file)
}

/// Read a qcow or qcow2 file from a reader
///
/// **Note**: unlike [`open`] this does not buffer your I/O. Any buffering should be handled via a
/// wrapper such as [`BufReader`] in order to ensure good performance where applicable.
///
/// ## Example
///
/// ```rust
/// # const PATH: &str = "/home/jamcleod/.panda/bionic-server-cloudimg-amd64-noaslr-nokaslr.qcow2";
/// # use std::fs::File;
/// # use std::io::BufReader;
/// let mut file = BufReader::new(File::open(PATH)?);
/// let qcow = qcow::load(&mut file)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn load(reader: &mut (impl Read + Seek)) -> Result<DynamicQcow, Error> {
    let QcowVersion(version) = reader.read_be()?;

    reader.seek(SeekFrom::Start(0)).unwrap();
    if version == 1 {
        reader.read_be().map(DynamicQcow::Qcow1)
    } else {
        reader.read_be().map(DynamicQcow::Qcow2)
    }
    .map_err(Error::from)
}

/// Read a qcow or qcow2 file from a slice
pub fn load_from_memory(bytes: &[u8]) -> Result<DynamicQcow, Error> {
    load(&mut Cursor::new(bytes))
}
