use core::num::dec2flt::parse;
use std::{collections::{BTreeMap, HashMap}, path::PathBuf, io::{BufReader, BufWriter}, fs::File};
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
    current_gen: u64,
    readers: HashMap<u64, BufReader<File>>,
    writer: BufWriter<File>,
    // index: BTreeMap<String, String>,
}

impl KvStore {

    /// Open the KvStore at a given path. Return the KvStore.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        let cur_dir = WalkDir::new(path).sort_by_file_name();
        let readers: HashMap<u64, BufReader<File>>= HashMap::new();
        let writer : BufWriter<File>;
        let lastiter = cur_dir.into_iter().last().
        
        for iter in cur_dir.into_iter() {
            
        }
        Ok(KvStore{
            path,
            current_gen: 0,
            readers,
            writer,
        })
    }

    /// Set the value of a string key to a string
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd: Command = Command::Set { key, value };
        let cmd_str = serde_json::to_string(&cmd)?;
        serde_json::to_writer(self.writer, &cmd_str);
        self.writer.
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

