use std::{collections::{BTreeMap, HashMap}, path::{PathBuf, Path}, io::{BufReader, BufWriter, Write, BufRead}, fs::{File, OpenOptions, self}, ffi::OsStr, any::type_name};
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
    index: HashMap<String, String>,
    readers: HashMap<u64, BufReader<File>>,
    writer: BufWriter<File>,
}

impl KvStore {
    
    /// Open the KvStore at a given path. Return the KvStore.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let cur_path = path.into();
        fs::create_dir_all(&cur_path);
        
        let mut index = HashMap::new();
        let mut readers: HashMap<u64, BufReader<File>> = HashMap::new();
        let mut sorted_gen_list: Vec<u64> = sorted_gen_list(&cur_path)?;
        for &gen in &sorted_gen_list {
            readers.insert(gen, BufReader::new(File::open(log_path(&cur_path, gen))?));
        }

        load(&mut index, &mut readers);
        let cur_gen = sorted_gen_list.last().unwrap_or(&0)+1;
        let writer = new_log_file(&cur_path, cur_gen, &mut readers)?;
        Ok(KvStore {
            cur_path,
            cur_gen,
            index,
            readers,
            writer,
        })
    }

    /// Set the value of a string key to a string. Return an error if the value is not written successfully.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {    
        self.index.insert(key.clone(), value.clone());
        let cmd = Command::set(key, value);
        serde_json::to_writer(&mut self.writer, &cmd);
        self.writer.write_all(b"\n")?;
        self.writer.flush()?;
        Ok(())
    }

    /// Get the string value of a string key. If the key does not exist, return None. Return an error if the value is not read successfully.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        if self.index.contains_key(&key) {
            Ok(self.index.get(&key).map(|s| s.to_string()))
        } else {
            Err(KVSError::KeyNotFound)
        }
    }
    
    /// Remove a given key. Return an error if the key does not exist or is not removed successfully.
    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.index.contains_key(&key) {
            self.index.remove(&key);
            serde_json::to_writer(&mut self.writer, &Command::remove(key));
            self.writer.write_all(b"\n")?;
            Ok(())
        } else {
            Err(KVSError::KeyNotFound)
        }       
    }
    
}

fn sorted_gen_list(path: &Path) -> Result<Vec<u64>> {
    println!("Get sorted gen list");
    let mut gen_list: Vec<u64> = fs::read_dir(path)?
            .flat_map(|res| -> Result<_> {Ok(res?.path())})
            .filter(|res| res.is_file() && res.extension() == Some(".log".as_ref()))
            .flat_map(|e| e.file_name()
                                .and_then(OsStr::to_str)
                                .map(|f| f.trim_end_matches(".log"))
                                .map(str::parse::<u64>))
            .flatten()
            .collect()
            ;
    gen_list.sort_unstable();
    println!("{:?}", gen_list);
    Ok(gen_list)
}

fn log_path(dir: &Path, gen: u64) -> PathBuf {
    dir.join(format!("{}.log", gen))
}

fn new_log_file(path: &Path, gen: u64, readers: &mut HashMap<u64, BufReader<File>>) -> Result<BufWriter<File>> {
    let path = log_path(path, gen);
    let writer = BufWriter::new(OpenOptions::new()
                                                                .create(true)
                                                                .append(true)
                                                                .open(&path)?);
    readers.insert(gen, BufReader::new(File::open(&path)?));
    Ok(writer)
}

fn load(index: &mut HashMap<String, String>, readers: &mut HashMap<u64, BufReader<File>>) -> Result<()> {
    println!("jump into load function");
    for (gen, gen_reader) in readers.iter_mut() {
        for line in gen_reader.lines() {
            let tmp: Command = serde_json::from_str(line?.as_str())?;
            println!("abcdee");
            println!("{:?}", tmp);
            match tmp {
                Command::Set { key, value } => {
                    index.insert(key, value);
                }                   
                Command::Remove { key } => {
                    index.remove(&key);
                }
            }
        }
    }
    Ok(())
}