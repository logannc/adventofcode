use crate::utils::errors::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub fn problem_input_path<'a>(day: u8, part: Option<u8>) -> PathBuf {
    let path_str = format!(
        "../advent_problems/day{:02}/input{}",
        day,
        part.as_ref().map_or(String::new(), u8::to_string)
    );
    path_str.into()
}

pub fn read_file_split_whitespace<T: std::str::FromStr>(file: &Path) -> Result<Vec<T>, Error>
where
    Error: std::convert::From<<T as std::str::FromStr>::Err>,
{
    let content = fs::read_to_string(file)?;
    let parsed: Result<Vec<T>, _> = content.split_whitespace().map(str::parse::<T>).collect();
    parsed.map_err(|e| e.into())
}
