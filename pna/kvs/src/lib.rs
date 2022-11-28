/// KvStore is key-value store built on top HashMap from std
use std::collections::HashMap;

/// KvStore stores key-value pairs in HashMap structure
///
/// # Examples
///
/// ```rust
/// use kvs::KvStore;
///
/// let mut store = KvStore::new();
/// store.set("foo", "bar");
/// assert_eq!(store.get("foo"), Some(String::from("bar")));
/// ```
#[derive(Debug)]
pub struct KvStore {
    inner: HashMap<String, String>,
}

impl KvStore {
    /// Creates empty store
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Sets key to point to value
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.inner.insert(key.into(), value.into());
    }

    /// Retrives value by key
    pub fn get(&self, key: impl Into<String>) -> Option<String> {
        self.inner.get(&key.into()).map(String::to_string)
    }

    /// Remove values from store by value
    pub fn remove(&mut self, key: String) {
        self.inner.remove(&key);
    }
}
