use failure::Error;
// This is a new error type that you've created. It represents the ways a
// toolchain could be invalid.
//
// The custom derive for Fail derives an impl of both Fail and Display.
// We don't do any other magic like creating new types.
#[derive(Debug, Fail)]
enum KVSError {
    #[fail(display = "invalid command: {}", name)]
    InvalidCommand {
        name: String,
    },
    #[fail(display = "unknown toolchain version: {}", version)]
    UnknownToolchainVersion {
        version: String,
    }
}

pub fn main() {
    println!("abcde");
}

pub type Result<T> = Result<T, KVSError>;