use std::collections::HashMap;
use std::{error::Error, fmt::Display, str::FromStr};

use crate::utils::*;

#[derive(Debug)]
struct ParseError {
    _bad_string: Option<String>,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");
        let x = if let Some(s) = parts.next() {
            if let Ok(num) = s.parse() {
                num
            } else {
                return Err(ParseError {
                    _bad_string: Some(s.into()),
                });
            }
        } else {
            return Err(ParseError { _bad_string: None });
        };
        let y = if let Some(s) = parts.next() {
            if let Ok(num) = s.parse() {
                num
            } else {
                return Err(ParseError {
                    _bad_string: Some(s.into()),
                });
            }
        } else {
            return Err(ParseError { _bad_string: None });
        };
        Ok(Self { x, y })
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");
        Ok(Line {
            start: parts.next().unwrap().parse()?,
            end: parts.next().unwrap().parse()?,
        })
    }
}

struct PointIterator {
    end: Point,
    cursor: Point,
    finished: bool,
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else {
            let result = self.cursor;
            let x_diff = self.end.x - self.cursor.x;
            let y_diff = self.end.y - self.cursor.y;
            let next = Point {
                x: self.cursor.x + x_diff.signum(),
                y: self.cursor.y + y_diff.signum(),
            };
            if result == self.end {
                self.finished = true;
            } else {
                self.cursor = next;
            }
            Some(result)
        }
    }
}

impl Line {
    fn iter(&self) -> PointIterator {
        PointIterator {
            end: self.end,
            cursor: self.start,
            finished: false,
        }
    }
    fn is_nondiagonal(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}

pub fn part_one() {
    let ip = problem_input_path(5, Some(1));
    let data: Vec<Line> = read_file_split_on(&ip, "\n").unwrap();
    let mut counts = HashMap::new();
    for line in data.into_iter().filter(Line::is_nondiagonal) {
        for point in line.iter() {
            let count = counts.entry(point).or_insert(0);
            *count += 1;
        }
    }
    println!(
        "number of overlaps: {}",
        counts.values().filter(|&&v| v > 1).count()
    );
}

pub fn part_two() {
    let ip = problem_input_path(5, Some(1));
    let data: Vec<Line> = read_file_split_on(&ip, "\n").unwrap();
    let mut counts = HashMap::new();
    for line in data.into_iter() {
        for point in line.iter() {
            let count = counts.entry(point).or_insert(0);
            *count += 1;
        }
    }
    println!(
        "number of overlaps: {}",
        counts.values().filter(|&&v| v > 1).count()
    );
}

#[test]
fn example() {
    let test = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;
    let data: Result<Vec<Line>, _> = test.split("\n").map(str::parse::<Line>).collect();
    let data = data.unwrap();
    let mut counts = HashMap::new();
    for line in data.iter().filter(|l| l.is_nondiagonal()) {
        for point in line.iter() {
            let count = counts.entry(point).or_insert(0);
            *count += 1;
        }
    }
    assert_eq!(counts.values().filter(|&&v| v > 1).count(), 5);
    
    let mut counts = HashMap::new();
    for line in data.iter() {
        for point in line.iter() {
            let count = counts.entry(point).or_insert(0);
            *count += 1;
        }
    }
    assert_eq!(counts.values().filter(|&&v| v > 1).count(), 12);
}
