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
        let path = path.into();
        let mut readers: HashMap<u64, BufReader<File>> = HashMap::new();
        let mut gen_list = sorted_gen_list(&path)?;
        for &gen in gen_list.iter() {
            let tmp = BufReader::new(File::open(log_path(&path, gen))?);
            readers.insert(gen, tmp);
        }

        let cur_gen = gen_list.last().unwrap_or(&0)+1;
        let mut writer: BufWriter<File> = new_log_file(&path, cur_gen, &mut readers)?;

        Ok(KvStore{
            path,
            cur_gen,
            readers,
            writer,
        })
    }

    /// Set the value of a string key to a string
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd: Command = Command::Set { key, value };
        let cmd_str = serde_json::to_string(&cmd)?;
        serde_json::to_writer(&mut self.writer, &cmd_str);
        
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

fn sorted_gen_list(path: &Path) -> Result<Vec<u64>> {

    let gen_list = fs::read_dir(path)?.flat_map(|res| res.map(|d| d.path()));
    for v in gen_list {
    }
        // .filter(|e|)
    ;
    //  let b = gen_list.next().map(|d| -> Result<_> {Ok(d?.file_name())});
    //  let t = b.map(|e| e?.as_os_str());
     let c = gen_list.next();
     match c {
         Some(val) => {},
         None => {},
     }

    let v = vec![1,2,3];
    // for i in v.iter();

    panic!("unimplemented")
}

/// Returns sorted generation numbers in the given directory.
fn sorted_gen_list_v2(path: &Path) -> Result<Vec<u64>> {
    let mut gen_list: Vec<u64> = fs::read_dir(&path)?
        .flat_map(|res| -> Result<_> { Ok(res?.path()) })
        .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))
        .flat_map(|path| {
            path.file_name()
                .and_then(OsStr::to_str)
                .map(|s| s.trim_end_matches(".log"))
                .map(str::parse::<u64>)
        })
        .flatten()
        .collect();
    gen_list.sort_unstable();
    Ok(gen_list)
}

fn log_path(dir: &Path, gen: u64) -> PathBuf {
    dir.join(format!("{}.log", gen))
}

fn new_log_file(dir: &Path, gen: u64, readers: &mut HashMap<u64, BufReader<File>>) -> Result<BufWriter<File>> {
    let path = log_path(dir, gen);
    let writer = BufWriter::new(OpenOptions::new().write(true).read(true).append(true).open(&path)?);
    readers.insert(gen, BufReader::new(File::open(&path)?));
    Ok((writer))
}