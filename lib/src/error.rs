use thiserror::Error;

/// An error that occured during decompression.
#[derive(Error, Debug)]
pub enum DecompressError {
    #[error("Failed to read compressed data")]
    ReadFailure(#[from] std::io::Error),
    #[error("Corrupt input: invalid length")]
    InvalidLength,
    #[error("Corrupt input: invalid offset")]
    InvalidOffset,
}
