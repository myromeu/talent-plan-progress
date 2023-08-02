mod error;

use anyhow::anyhow;
/// KvStore is key-value store built on top HashMap from std
pub use error::Result;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::PathBuf;

/// KvStore stores key-value pairs in HashMap structure
///
/// # Examples
///
/// ```rust
/// use kvs::KvStore;
///
/// let mut store = KvStore::new();
/// store.set("foo".to_owned(), "bar".to_owned());
/// assert_eq!(store.get("foo".to_owned()), Ok(Some(String::from("bar"))));
/// ```
#[derive(Debug)]
pub struct KvStore {
    index: HashMap<String, u64>,
    log_file_reader: BufReader<File>,
    log_file_writer: BufWriter<File>,
    writer_pos: u64,
}

impl KvStore {
    fn write_log(&mut self, command: &Command) -> Result<()> {
        let buf = serde_json::to_vec(command)?;
        let mut len = self.log_file_writer.write(&buf)? as u64;
        len += self.log_file_writer.write(&[b'\n'])? as u64;
        self.log_file_writer.flush()?;
        self.writer_pos += len;
        Ok(())
    }

    /// Sets key to point to value
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let set_cmd = Command::Set {
            key: key.clone(),
            value,
        };
        let pos = self.writer_pos;
        self.write_log(&set_cmd)?;
        *self.index.entry(key).or_default() = pos;
        Ok(())
    }

    /// Retrives value by key
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(pos) = self.index.get(&key) {
            self.log_file_reader.seek(SeekFrom::Start(*pos))?;
            let mut line = String::new();
            let _ = self.log_file_reader.read_line(&mut line);
            let cmd: Command = serde_json::from_str(line.as_str())?;
            match cmd {
                Command::Set { value, .. } => Ok(Some(value)),
                Command::Remove { .. } => {
                    unreachable!("we dont store keys in index for removed items")
                }
            }
        } else {
            Ok(None)
        }
    }

    /// Remove values from store by value
    pub fn remove(&mut self, key: String) -> Result<()> {
        let offset = self.index.remove(&key);
        match offset {
            Some(_) => {
                let rm_cmd = Command::Remove { key };
                self.write_log(&rm_cmd)?;
            }
            None => {
                return Err(anyhow!("Key not found"));
            }
        }
        Ok(())
    }

    /// Open store at given path
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut path: PathBuf = path.into();
        path.push("file.log");
        let index = read_index(&path)?;
        let log_file = File::options().write(true).create(true).open(&path)?;
        let mut log_file_writer = BufWriter::new(log_file);
        let log_file_reader = BufReader::new(File::open(path)?);
        let writer_pos = log_file_writer.seek(SeekFrom::End(0))?;
        Ok(KvStore {
            index,
            log_file_reader,
            log_file_writer,
            writer_pos,
        })
    }
}

fn read_index(path: &PathBuf) -> Result<HashMap<String, u64>> {
    let mut index = HashMap::new();
    if path.exists() {
        let mut log_file = BufReader::new(File::open(path)?);
        let mut entry_buf = String::new();
        let mut pos = 0;

        while let Ok(len) = log_file.read_line(&mut entry_buf) {
            if len == 0 {
                break; // file reached EOF
            }
            let cmd = serde_json::from_str(&entry_buf.as_str())?;
            entry_buf.clear();
            match cmd {
                Command::Set { key, .. } => {
                    *index.entry(key).or_default() = pos as u64;
                }
                Command::Remove { ref key } => {
                    index.remove(key);
                }
            }
            pos += len;
        }
    }
    Ok(index)
}

#[derive(Debug, Deserialize, Serialize)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}
