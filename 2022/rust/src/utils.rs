use eyre::Result;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub fn problem_input_path(day: u8, part: Option<u8>) -> PathBuf {
    let path_str = format!(
        "../problems/day{:02}/input{}",
        day,
        part.as_ref().map_or(String::new(), u8::to_string)
    );
    path_str.into()
}

pub fn _read_file_split_whitespace<T: std::str::FromStr>(file: &Path) -> Result<Vec<T>>
where
    <T as FromStr>::Err: 'static + Error + Send + Sync,
{
    let content = fs::read_to_string(file)?;
    let parsed: Result<Vec<T>, _> = content.split_whitespace().map(str::parse::<T>).collect();
    parsed.map_err(|e| e.into())
}

pub fn _read_file_split_on<T: std::str::FromStr>(file: &Path, pattern: &str) -> Result<Vec<T>>
where
    <T as FromStr>::Err: 'static + Error + Send + Sync,
{
    let content = fs::read_to_string(file)?;
    let content = content.trim();
    let parsed: Result<Vec<T>, _> = content.split(pattern).map(str::parse::<T>).collect();
    parsed.map_err(|e| e.into())
}

// #[derive(Default)]
// pub struct CharHasher(u64);

// impl Hasher for CharHasher {
//     fn write(&mut self, bytes: &[u8]) {
//         for byte in bytes.iter() {
//             self.0 = self.0.wrapping_shl(8) + (*byte as u64);
//         }
//     }
//     fn finish(&self) -> u64 {
//         self.0
//     }
// }

// pub type CharHasherBuilder = BuildHasherDefault<CharHasher>;

#[derive(Debug, Default)]
pub struct Compartment {
    pub items: HashMap<char, u32>,
}

impl From<&str> for Compartment {
    fn from(value: &str) -> Self {
        let mut items = HashMap::new();
        for c in value.chars() {
            *(items.entry(c).or_default()) += 1;
        }
        Compartment { items }
    }
}
