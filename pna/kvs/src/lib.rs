/// KvStore is key-value store built on top HashMap from std

use std::collections::HashMap;


/// KvStore stores key-value pairs in HashMap structure
///
/// # Examples
///
/// ```rust
/// use kvs::KvStore;
///
/// let store = KvStore::new();
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
    pub fn set(&mut self, key: String, value: String) {
        self.inner.insert(key, value);
    }

    /// Retrives value by key
    pub fn get(&self, key: String) -> Option<String> {
        self.inner.get(&key).map(String::to_string)
    }

    /// Remove values from store by value
    pub fn remove(&mut self, key: String) {
        self.inner.remove(&key);
    }
}
