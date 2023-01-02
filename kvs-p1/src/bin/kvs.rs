use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands 
}

#[derive(Subcommand)]
enum Commands {}



fn main() {
    kvs::KvStore;
    println!("Hello, world!")
}