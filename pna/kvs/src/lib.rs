mod error;

use anyhow::anyhow;
/// KvStore is key-value store built on top HashMap from std
pub use error::Result;
use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use std::fs::File;
use std::io::{Seek, BufWriter, SeekFrom, Write, BufReader, BufRead, Read};
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
        let _ = self.log_file_writer.write(buf.len().to_string().as_bytes())?;
        let _ = self.log_file_writer.write(&[b'-'])?;
        let len = self.log_file_writer.write(&buf)? as u64;
        self.log_file_writer.flush()?;
        self.writer_pos += len;
        Ok(())
    }

    /// Sets key to point to value
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let set_cmd = Command::Set { key: key.clone(), value };
        let pos = self.writer_pos;
        self.write_log(&set_cmd)?;
        *self.index.entry(key).or_default() = pos;
        Ok(())
    }

    /// Retrives value by key
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(pos) = self.index.get(&key) {
            self.log_file_reader.seek(SeekFrom::Start(*pos))?;
            let mut buf = vec![];
            self.log_file_reader.read_until(b'}', &mut buf)?;
            let s = unsafe { String::from_utf8_unchecked(buf) };
            self.log_file_reader.seek(SeekFrom::Start(*pos))?;
            let cmd: Command = serde_json::from_reader(&mut self.log_file_reader)?;
            match cmd {
                Command::Set { value, .. } => Ok(Some(value)),
                Command::Remove { .. } => unreachable!("we dont store keys in index for removed items"),
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
            },
            None => {
                return Err(anyhow!("Key not found"));
            },
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
        Ok(KvStore { index, log_file_reader, log_file_writer, writer_pos })
    }

}

fn read_index(path: &PathBuf) -> Result<HashMap<String, u64>> {
    let mut index = HashMap::new();
    if path.exists() {
        let mut log_file = BufReader::new(File::open(path)?);
        //let mut deser = serde_json::Deserializer::from_reader(log_file).into_iter::<Command>();
        let mut entry_length_buf = Vec::new();
        let mut pos = log_file.read_until(b'-', &mut entry_length_buf)?;
        if pos == 0 {
            // file is empty
            return Ok(index);
        }
        entry_length_buf.pop();
        let mut entry_length: usize = String::from_utf8(entry_length_buf.clone())?.parse()?;
        let mut entry_buf = vec![0u8; entry_length];
        while let Ok(_) = log_file.read_exact(&mut entry_buf) {
            let cmd = serde_json::from_slice(&entry_buf)?;
            match cmd {
                Command::Set { key, .. } => {
                    *index.entry(key).or_default() = pos as u64;
                },
                Command::Remove { ref key } => {
                    index.remove(key);
                },
            }
            pos += entry_buf.len();
            entry_length_buf.clear();
            let l = log_file.read_until(b'-', &mut entry_length_buf)?;
            entry_length_buf.pop();
            if l == 0 {
                break;
            }
            pos += l;
            entry_length = String::from_utf8(entry_length_buf.clone())?.parse()?;
            entry_buf = vec![0u8; entry_length];
        }
    }
    Ok(index)
}

#[derive(Debug, Deserialize, Serialize)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}
