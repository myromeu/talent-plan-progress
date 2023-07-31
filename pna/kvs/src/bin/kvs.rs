use std::process::exit;

use clap::{Parser, Subcommand};
use kvs::Result;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value_t = String::from("./"))]
    path: String,
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get { key } => {
            let mut kvs = kvs::KvStore::open(cli.path.as_str())?;
            match kvs.get(key.to_string()) {
                Ok(val) => {
                    match val {
                        Some(val) => println!("{val}"),
                        None => println!("Key not found"),
                    };
                    exit(0)
                }
                Err(err) => {
                    println!("{err}");
                    exit(1)
                }
            }
        }
        Commands::Set { key, value } => {
            let mut kvs = kvs::KvStore::open(cli.path.as_str())?;
            kvs.set(key.to_string(), value.to_string())
        }
        Commands::Rm { key } => {
            let mut kvs = kvs::KvStore::open(cli.path.as_str())?;
            match kvs.remove(key.to_string()) {
                Ok(_) => exit(0),
                Err(err) => {
                    println!("{err}");
                    exit(1)
                }
            }
        }
    }
}
