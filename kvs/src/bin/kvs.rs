use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Set the value of a string key to a string")]
    Set {
        #[clap(help = "The key string")]
        key: Option<String>,
        #[clap(help = "The value assigned to key")]
        val: Option<String>,
    },

    #[clap(about = "Get the string value of a given string key")]
    Get {
        #[clap(help = "The key string")]
        key: Option<String>
    },

    #[clap(about = "Remove a given key")]
    Rm {
        #[clap(help = "The key string")]
        key: Option<String>
    }
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    match args.command {
        Commands::Set{key: _key, val: _val} => Err("unimplemented".to_owned()),
        Commands::Get{key: _key} => Err("unimplemented".to_owned()),
        Commands::Rm{key: _key} => Err("unimplemented".to_owned()),
    }
}