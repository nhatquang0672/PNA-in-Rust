use failure::{Fail};
use std::io;
// This is a new error type that you've created. It represents the ways a
// toolchain could be invalid.
//
// The custom derive for Fail derives an impl of both Fail and Display.
// We don't do any other magic like creating new types.
#[derive(Debug, Fail)]
/// Error
pub enum KVSError {
    /// Invalid Command
    #[fail(display = "invalid command")]
    InvalidCommand,
    /// IO Error
    #[fail(display = "IO Error")]
    IOError(io::Error),
}


impl From<io::Error> for KVSError {
    fn from(error: io::Error) -> Self {
        KVSError::IOError(error)
    }
}

/// Global result type
pub type Result<T> = std::result::Result<T, KVSError>;