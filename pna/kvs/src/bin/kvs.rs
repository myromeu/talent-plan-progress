use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get value by key
    Get { key: String },

    /// Set key with associated value
    Set { key: String, value: String },

    /// Remove value by key
    Rm { key: String },
}

fn main() {
    let _cli = Cli::parse();

    panic!("unimplemented")
}
