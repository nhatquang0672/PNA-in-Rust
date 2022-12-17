use std::collections::HashMap;

/// Data Structure handling the storage and retrieval
/// of key-value data
///
/// ```
/// use kvs::KvStore;
///
/// let mut store = KvStore::new();
/// store.set(String::from("key"), String::from("value"));
/// assert_eq!(Some(String::from("value")), store.get(String::from("key")));
///
pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    /// Create new KvStore instance.
    pub fn new() -> KvStore {
        KvStore {
            data: HashMap::new(),
        }
    }

    /// Set key-value pair KvStore.
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Get a copy of owned values associated with key
    /// return None if key not found.
    pub fn get(&self, key: String) -> Option<String> {
        self.data.get(&key).cloned()
    }

    /// Remove key-value pair from the key.
    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
    }
}
