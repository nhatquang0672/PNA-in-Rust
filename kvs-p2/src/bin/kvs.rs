use std::env::current_dir;

use clap::{Parser, Subcommand};
use kvs::{Result, KVSError};
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args{
    #[clap(subcommand)]
    commands: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Set the value of a string key to a string. Print an error and return a non-zero exit code on failure.
    Set {
        /// The string key
        key: Option<String>,
        ///The value associated to key
        value: Option<String>,
    },
    /// Get the string value of a given string key. Print an error and return a non-zero exit code on failure.
    Get {
        /// The string key
        key: Option<String>,
    },
    /// Remove a given key. Print an error and return a non-zero exit code on failure.
    Rm {
        /// The string key
        key: Option<String>,
    }

}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.commands {
        Command::Get { key: _key } => {
            let mut kv = kvs::KvStore::open(current_dir()?)?;
            let res = kv.get(_key.unwrap());
            match res {
                Ok(value) => println!("{}", value.unwrap()),
                Err(KVSError::KeyNotFound) => println!("{}", KVSError::KeyNotFound),
                _ => unreachable!(),
            }
        },
        Command::Set { key: _key, value: _value } => {
            let mut kv = kvs::KvStore::open(current_dir()?)?;
            kv.set(_key.unwrap(), _value.unwrap());
        }, 
        Command::Rm { key: _key } => {},
    }
    Ok(())
}