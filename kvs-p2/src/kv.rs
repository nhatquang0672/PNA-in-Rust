use std::{collections::{BTreeMap, HashMap}, path::{PathBuf, Path}, io::{BufReader, BufWriter}, fs::{File, OpenOptions, self}, ffi::OsStr};
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;
use crate::error::{KVSError, Result};


#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set {key: String, value: String },
    Remove { key: String },
}
impl Command {
    fn set(key: String, value: String) -> Command {
        Command::Set {key, value}
    }
    fn remove(key: String) -> Command {
        Command::Remove {key}
    }
}

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
    path: PathBuf,
    cur_gen: u64,
    readers: HashMap<u64, BufReader<File>>,
    writer: BufWriter<File>,
    // index: BTreeMap<String, String>,
}

impl KvStore {

    /// Open the KvStore at a given path. Return the KvStore.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        panic!("unimplemented")
    }

    /// Set the value of a string key to a string
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        panic!("unimplemented")
    }

    /// Get the string value of the a string key. If the key does not exist, return None. 
    pub fn get(&self, key: String) -> Result<Option<String>> {
        panic!("unimplemented")
    }
    
    /// Remove a given key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        panic!("unimplemented")
    }


    
}
