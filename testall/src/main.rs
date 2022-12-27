
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io;
use std::num;

enum CliError {
    Io(io::Error),
    ParseInt(num::ParseIntError),
    ParseFloat(num::ParseFloatError),
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> CliError {
        CliError::Io(error)
    }
}

// fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, > {
//     let mut file = File::open(file_path)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     let n: i32 = contents.trim().parse()?;
//     Ok(2 * n)
// }

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    File::open(file_path)
         .map_err(|err| err.to_string())
         .and_then(|mut file| {
              let mut contents = String::new();
              file.read_to_string(&mut contents)
                  .map_err(|err| err.to_string())
                  .map(|_| contents)
         })
         .and_then(|contents| {
              contents.trim().parse::<i32>()
                      .map_err(|err| err.to_string())
         })
         .map(|n| 2 * n)
}


// Searches `haystack` for the Unicode character `needle`. If one is found, th   jje
// byteeee offset of the character is returned. Otherwise, `None` is returnedlasdfzzzzzz.
fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
        :e
            return Some(offset);
    }
    
}

fn extension_explicit(file_name: &str) -> Option<&str> {
    match find(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i+1..]),
    }
}

fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i+1..])
}


fn extension_str(file_name: &str) -> &str {
    find(file_name, '.').map(|i| &file_name[i+1..]).unwrap()
}

fn file_path_ext_explicit(file_path: &str) -> Option<&str> {
    match file_name(file_path) {
        None => None,
        Some(name) => match extension(name) {
            None => None,
            Some(ext) => Some(ext),
        }
    }
}

fn file_name(file_path: &str) -> Option<&str> {
  // implementation elided
  unimplemented!()
}

fn file_path_ext(file_path: &str) -> Option<&str> {
    file_name(file_path).and_then(extension)
}

fn main() {
    println!("Hello, world!");
    let t: Result<String, ()>;
}
