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
    /// Create the new instance KvStore
    pub fn new() -> KvStore {
        KvStore {
            data: HashMap::new()
        }
    }
    /// Set the value of a string key to a string
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Get the string value of the a string key. If the key does not exist, return None. 
    pub fn get(&self, key: String) -> Option<String> {
        self.data.get(&key).cloned()
    }
    
    /// Remove a given key.
    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
    }
    
}