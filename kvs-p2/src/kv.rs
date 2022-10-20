use std::{collections::{BTreeMap, HashMap}, path::{PathBuf, Path}, io::{BufReader, BufWriter, Write, BufRead, Read, Seek, SeekFrom}, fs::{File, OpenOptions, self}, ffi::OsStr, any::type_name, ops::Range};
use predicates::ord::le;
use serde::{Serialize, Deserialize};
use serde_json::Deserializer;
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
    index: BTreeMap<String, CommandPos>,
    readers: HashMap<u64, BufReaderWithPos<File>>,
    writer: BufWriterWithPos<File>,
}

impl KvStore {
    
    /// Open the KvStore at a given path. Return the KvStore.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let cur_path = path.into();
        fs::create_dir_all(&cur_path);
        
        let mut index = BTreeMap::new();
        let mut readers: HashMap<u64, BufReaderWithPos<File>> = HashMap::new();
        let mut sorted_gen_list: Vec<u64> = sorted_gen_list(&cur_path)?;
        for &gen in &sorted_gen_list {
            let mut reader = BufReaderWithPos::new(File::open(log_path(&cur_path, gen))?)?;
            load(gen, &mut index, &mut reader);
            readers.insert(gen, reader);
        }

        let cur_gen = sorted_gen_list.last().unwrap_or(&0)+0;
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
        let pos = self.writer.pos; 
        let cmd = Command::set(key.to_owned(), value.to_owned());
        serde_json::to_writer(&mut self.writer, &cmd);
        self.writer.flush()?;
        self.index.insert(key, (self.cur_gen, pos..self.writer.pos).into());
        Ok(())
    }

    /// Get the string value of a string key. If the key does not exist, return None. Return an error if the value is not read successfully.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(cmd_pos) = self.index.get(&key) {
            let reader = self.readers.get_mut(&cmd_pos.gen).expect("Can not find log reader");
            reader.seek(SeekFrom::Start(cmd_pos.pos));
            let cmd_reader = reader.take(cmd_pos.len);
            if let Command::Set { key, value } = serde_json::from_reader(cmd_reader)? {
                Ok(Some(value))
            } else {
                Err(KVSError::InvalidCommand)
            }
        } else {
            Ok(None)
        }
    }
    
    /// Remove a given key. Return an error if the key does not exist or is not removed successfully.
    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.index.contains_key(&key) {
            self.index.remove(&key);
            serde_json::to_writer(&mut self.writer, &Command::remove(key));
            self.writer.flush()?;
            Ok(())
        } else {
            Err(KVSError::KeyNotFound)
        }       
    }
    
}

fn sorted_gen_list(path: &Path) -> Result<Vec<u64>> {
    let mut gen_list: Vec<u64> = fs::read_dir(path)?
            .flat_map(|res| -> Result<_> {Ok(res?.path())})
            .filter(|res| res.is_file() && res.extension() == Some("log".as_ref()))
            .flat_map(|e| e.file_name()
                                .and_then(OsStr::to_str)
                                .map(|f| f.trim_end_matches(".log"))
                                .map(str::parse::<u64>))
            .flatten()
            .collect()
            ;
    gen_list.sort_unstable();
    Ok(gen_list)
}

fn log_path(dir: &Path, gen: u64) -> PathBuf {
    dir.join(format!("{}.log", gen))
}

fn new_log_file(path: &Path, gen: u64, readers: &mut HashMap<u64, BufReaderWithPos<File>>) -> Result<BufWriterWithPos<File>> {
    let path = log_path(path, gen);
    let writer = BufWriterWithPos::new(OpenOptions::new()
                                                                .create(true)
                                                                .append(true)
                                                                .open(&path)?)?;
    readers.insert(gen, BufReaderWithPos::new(File::open(&path)?)?);
    Ok(writer)
}

// fn load_v1(index: &mut BTreeMap<String, CommandPos>, readers: &mut HashMap<u64, BufReaderWithPos<File>>) -> Result<()> {
//     for (gen, gen_reader) in readers.iter_mut() {
//         for line in gen_reader.lines() {
//             let tmp: Command = serde_json::from_str(line?.as_str())?;
//             match tmp {
//                 Command::Set { key, value } => {
//                     index.insert(key, value);
//                 }                   
//                 Command::Remove { key } => {
//                     index.remove(&key);
//                 }
//             }
//         }
//     }
//     Ok(())
// }

fn load(gen: u64, index: &mut BTreeMap<String, CommandPos>, reader: &mut BufReaderWithPos<File>) -> Result<()> {
    let mut pos = reader.seek(SeekFrom::Start(0))?;
    let mut stream = Deserializer::from_reader(reader).into_iter::<Command>();
    while let Some(cmd) = stream.next() {
        let new_pos = stream.byte_offset() as u64;
        match cmd? {
            Command::Set { key, ..} => {
                index.insert(key, (gen, pos..new_pos).into());
            },
            Command::Remove { key } => {
                index.remove(&key);
            },
        }
        pos = new_pos;
    }
    Ok(())
}

struct CommandPos {
    gen: u64,
    pos: u64,
    len: u64,
}

impl From<(u64, Range<u64>)> for CommandPos {
    fn from((gen, range): (u64, Range<u64>)) -> Self {
        CommandPos { gen, pos: range.start, len: range.end-range.start}
    }
}
struct BufReaderWithPos <R: Read + Seek> {
    reader: BufReader<R>,
    pos: u64,
} 

impl <R: Read+Seek> BufReaderWithPos<R> {
    fn new(mut inner: R) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos { reader: BufReader::new(inner) , pos})
    }
}
impl <R: Read+Seek> Read for BufReaderWithPos<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl <R: Read+Seek> Seek for BufReaderWithPos<R> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}
struct BufWriterWithPos<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64
} 

impl <W: Write+Seek> BufWriterWithPos<W> {
    fn new(mut inner: W) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufWriterWithPos { writer: BufWriter::new(inner), pos})
    }
}

impl <W: Write+Seek> Write for BufWriterWithPos<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl <W: Write+Seek> Seek for BufWriterWithPos<W> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.writer.seek(pos)?;
        Ok(self.pos)
    }
}