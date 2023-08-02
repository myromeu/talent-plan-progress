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

    /// Test some functionality
    Foo,
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
        Commands::Foo => {
            let mut store = kvs::KvStore::open(cli.path.as_str())?;

            store.set("key1".to_owned(), "value1".to_owned())?;
            store.set("key2".to_owned(), "value2".to_owned())?;

            //assert_eq!(store.get("key1".to_owned())?, Some("value1".to_owned()));
            //assert_eq!(store.get("key2".to_owned())?, Some("value2".to_owned()));

            println!("index before drop: {:?}", store);
            std::thread::sleep(std::time::Duration::from_secs(10));

            // Open from disk again and check persistent data.
            drop(store);
            Ok(())
        }
    }
}
