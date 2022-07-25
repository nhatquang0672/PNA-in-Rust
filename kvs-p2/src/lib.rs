#![deny(missing_docs)]
//! This crate provides a KvStore structure
//! that is capable of storing key-value pairs in memory
mod kv;
pub use kv::KvStore;