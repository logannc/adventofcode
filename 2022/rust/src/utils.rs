use eyre::Result;
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
