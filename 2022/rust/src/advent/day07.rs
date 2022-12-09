use crate::utils::*;
use eyre::{ContextCompat, Report, Result};
use std::{
    fs,
    iter::{once, Peekable},
    str::{FromStr, Lines},
};

trait FileSystemSize {
    fn size(&self) -> usize;
}

#[derive(Debug)]
struct Directory {
    _name: String,
    contents: Vec<FileSystemEntry>,
}

impl Directory {
    fn named(name: String) -> Directory {
        Directory {
            _name: name,
            contents: Vec::new(),
        }
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &FileSystemEntry> + 'a> {
        Box::new(
            self.contents.iter().chain(
                self.contents
                    .iter()
                    .map(|fse| match fse {
                        FileSystemEntry::Directory(d) => d.iter(),
                        FileSystemEntry::File(_) => Box::new(once(fse)),
                    })
                    .flatten(),
            ),
        )
    }
}

impl FileSystemSize for Directory {
    fn size(&self) -> usize {
        self.contents.iter().map(FileSystemSize::size).sum()
    }
}

#[derive(Debug)]
struct File {
    _name: String,
    size: usize,
}

impl FileSystemSize for File {
    fn size(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
enum FileSystemEntry {
    Directory(Directory),
    File(File),
}

impl FileSystemSize for FileSystemEntry {
    fn size(&self) -> usize {
        match self {
            FileSystemEntry::Directory(d) => d.size(),
            FileSystemEntry::File(f) => f.size(),
        }
    }
}

struct CDLine(String);

impl CDLine {
    const PREFIX: &str = "$ cd ";
}

impl FromStr for CDLine {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix(CDLine::PREFIX) {
            Some(name) => Ok(CDLine(name.into())),
            None => Err(Report::msg(format!(
                "Failed to strip [{}] off [{}]",
                CDLine::PREFIX,
                s
            ))),
        }
    }
}

enum LSLine {
    Dir(String),
    File(usize, String),
}

impl LSLine {
    const DIR_PREFIX: &str = "dir ";
}

impl FromStr for LSLine {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with(LSLine::DIR_PREFIX) {
            Ok(LSLine::Dir(
                s.strip_prefix(LSLine::DIR_PREFIX)
                    .expect("ensured with starts_with")
                    .into(),
            ))
        } else {
            let (size, name) = s.split_once(" ").wrap_err_with(|| {
                Report::msg(format!(
                    "expected 'number filename', but failed to split on the space for [{}]",
                    s
                ))
            })?;
            Ok(LSLine::File(str::parse(size)?, name.into()))
        }
    }
}

fn parse_inner<'a>(
    mut d: Directory,
    mut input: Peekable<Lines<'a>>,
) -> Result<(Directory, Peekable<Lines<'a>>)> {
    while let Some(line) = input.peek() {
        if line.starts_with(CDLine::PREFIX) {
            if line.ends_with("..") {
                // manually consume since we don't need a whole function for it.
                input
                    .next()
                    .expect("we *just* peeked, so we know there is a line.");
                return Ok((d, input));
            } else {
                let subd: Directory;
                (subd, input) = parse_cd(input)?;
                d.contents.push(FileSystemEntry::Directory(subd));
            }
        } else {
            (d, input) = parse_ls(d, input)?;
        }
    }
    Ok((d, input))
}

fn parse_ls<'a>(
    mut d: Directory,
    mut input: Peekable<Lines<'a>>,
) -> Result<(Directory, Peekable<Lines<'a>>)> {
    input
        .next()
        .expect("callers guarantee the first line is $ ls");
    while let Some(line) = input.next_if(|line| !line.starts_with("$")) {
        let ls_line: LSLine = str::parse(line)?;
        match ls_line {
            LSLine::File(size, name) => d.contents.push(FileSystemEntry::File(File {
                _name: name.into(),
                size,
            })),
            // We ignore dirs because we add them after we cd into them
            // we assume the order of the vec for the directory contents doesn't matter. if it does we can alphabetize it later.
            LSLine::Dir(_) => {}
        }
    }
    Ok((d, input))
}

fn parse_cd<'a>(mut input: Peekable<Lines<'a>>) -> Result<(Directory, Peekable<Lines<'a>>)> {
    let cd_line: CDLine = str::parse(
        input
            .next()
            .expect("callers guarantee the first line is `$ cd foo`"),
    )?;
    let d = Directory::named(cd_line.0.into());
    let (d, input) = parse_inner(d, input)?;
    Ok((d, input))
}

pub fn part_one() -> Result<usize> {
    let input_path = problem_input_path(7, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_one_inner(&content)?;
    println!("{}", result);
    Ok(result)
}

pub fn part_two() -> Result<usize> {
    let input_path = problem_input_path(7, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_two_inner(&content)?;
    println!("{}", result);
    Ok(result)
}

fn part_one_inner(input: &str) -> Result<usize> {
    let (root, _) = parse_cd(input.lines().peekable())?;
    Ok(root
        .iter()
        .filter(|fse| match fse {
            FileSystemEntry::Directory(d) => d.size() <= 100000,
            FileSystemEntry::File(_) => false,
        })
        .map(|fse| fse.size())
        .sum())
}

fn part_two_inner(input: &str) -> Result<usize> {
    const TOTAL_SPACE: usize = 70000000;
    const REQUIRED_SPACE: usize = 30000000;
    let (root, _) = parse_cd(input.lines().peekable())?;
    let used_space = TOTAL_SPACE - root.size();
    let minimum_size = REQUIRED_SPACE - used_space;
    root.iter()
        .filter(|fse| match fse {
            FileSystemEntry::Directory(d) => d.size() >= minimum_size,
            FileSystemEntry::File(_) => false,
        })
        .map(|fse| fse.size())
        .min()
        .wrap_err_with(|| Report::msg("it was empty?"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one_inner(TEST_INPUT).unwrap(), 95437);
        assert_eq!(part_one().unwrap(), 1232307);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two_inner(TEST_INPUT).unwrap(), 24933642);
        assert_eq!(part_two().unwrap(), 7268994);
    }
}
