use thiserror::Error;

/// An error that occured during decompression.
#[derive(Error, Debug)]
pub enum DecompressError {
    #[error("Compressed data stream ended prematurely")]
    TruncatedInput,
    #[error("Compressed data is invalid")]
    InvalidInput,
    #[error("Failed to read compressed data")]
    ReadFailure(#[from] std::io::Error),
}
