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
    #[fail(display = "Invalid command")]
    InvalidCommand,
    /// IO Error
    #[fail(display = "IO Error")]
    IOError(#[cause] io::Error),
    /// Key Not Found
    #[fail(display = "Key not found")]
    KeyNotFound,
/// Serialize/Deserialize Error
    #[fail(display = "Serde Error")]
    SerdeError(serde_json::Error),

    /// Umimplemented 
    #[fail(display = "Unimmplemented")]
    Unimplemented,
}

impl From<io::Error> for KVSError {
    fn from(error: io::Error) -> Self {
        KVSError::IOError(error)
    }
}

impl From<serde_json::Error> for KVSError {
    fn from(error: serde_json::Error) -> Self {
        KVSError::SerdeError(error)
    }
}

/// Global result type
pub type Result<T> = std::result::Result<T, KVSError>;