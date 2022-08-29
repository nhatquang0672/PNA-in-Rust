#![deny(missing_docs)]
//! This crate provides a KvStore structure
//! that is capable of storing key-value pairs in memory
mod kv;
mod error;
pub use kv::KvStore;
pub use error::{KVSError, Result};
