use clap::{Parser, Subcommand};
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

fn main() -> Result<(), String> {
    let args = Args::parse();
    match args.commands {
        Command::Get { key: _key } => Err("unimplemented".to_owned()),
        Command::Set { key: _key, value: _value } => Err("unimplemented".to_owned()),
        Command::Rm { key: _key } => Err("unimplemented".to_owned()),
    }
}