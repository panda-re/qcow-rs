use thiserror::Error;

/// An error encountered by the qcow crate from either a parsing failure or an I/O error.
#[derive(Error, Debug)]
pub enum Error {
    /// Error opening the qcow file
    #[error("The qcow file could not successfully be opened")]
    FileNotFound(std::io::Error),

    /// Error that occurs while parsing the qcow
    #[error("The qcow file failed to parse")]
    ParseError(#[from] binread::Error),
}
