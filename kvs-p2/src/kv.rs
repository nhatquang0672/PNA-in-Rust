use std::{collections::{BTreeMap, HashMap}, path::{PathBuf, Path}, io::{BufReader, BufWriter}, fs::{File, OpenOptions, self}, ffi::OsStr};
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;
use crate::error::{KVSError, Result};


#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set {key: String, value: String},
    Remove {key: String},
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
    cur_path: PathBuf,
    cur_gen: u64,
    readers: HashMap<u64, BufReader<File>>,
    writer: BufWriter<File>,
    
}

impl KvStore {

    /// Open the KvStore at a given path. Return the KvStore.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let cur_path = path.into();
        
        let mut readers: HashMap<u64, BufReader<File>> = HashMap::new();
        let sorted_gen_list: Vec<u64> = sorted_gen_list(&cur_path)?;
        for &gen in &sorted_gen_list {
            readers.insert(gen, BufReader::new(File::open(log_path(&cur_path, gen))?));
        }

        let cur_gen = sorted_gen_list.last().unwrap_or(&0)+1;
        let writer = new_log_file(&cur_path, cur_gen, &readers);
        Ok(KvStore {
            cur_path,
            cur_gen,
            readers,
            writer,
        })
    }

    /// Set the value of a string key to a string. Return an error if the value is not written successfully.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {    
        panic!("unimplemented")
    }

    /// Get the string value of a string key. If the key does not exist, return None. Return an error if the value is not read successfully.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        panic!("unimplemented")
    }
    
    /// Remove a given key. Return an error if the key does not exist or is not removed successfully.
    pub fn remove(&mut self, key: String) -> Result<()> {
        panic!("unimplemented")
    }
    
}

fn sorted_gen_list(path: &Path) -> Result<Vec<u64>> {
    panic!("unimplemented")
}

fn log_path(dir: &Path, gen: u64) -> PathBuf {
    dir.join(format!("{}.log", gen))
}

fn new_log_file(path: &Path, gen: u64, readers: &HashMap<u64, BufReader<File>>) -> BufWriter<File> {
    panic!("unimplemented")
}