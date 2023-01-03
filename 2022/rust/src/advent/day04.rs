use crate::utils::*;
use eyre::{Report, Result};
use std::{fs, str::FromStr};

struct Assignment {
    start: u32,
    end: u32,
}

impl FromStr for Assignment {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once('-')
            .ok_or_else(|| Report::msg(format!("Failed to split [{s}]")))?;
        let start = str::parse(start)?;
        let end = str::parse(end)?;
        Ok(Assignment { start, end })
    }
}

impl Assignment {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn overlap(&self, other: &Self) -> bool {
        (self.start <= other.start && other.start <= self.end)
            || (other.start <= self.start && self.start <= other.end)
    }
}

struct Pair {
    left: Assignment,
    right: Assignment,
}

impl FromStr for Pair {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(',')
            .ok_or_else(|| Report::msg(format!("Failed to split [{s}]")))?;
        let left: Assignment = str::parse(left)?;
        let right: Assignment = str::parse(right)?;
        Ok(Pair { left, right })
    }
}

impl Pair {
    fn contains(self) -> bool {
        self.left.contains(&self.right) || self.right.contains(&self.left)
    }

    fn overlap(self) -> bool {
        self.left.overlap(&self.right)
    }
}

pub fn part_one() -> Result<u32> {
    let input_path = problem_input_path(4, Some(1));
    let content = fs::read_to_string(input_path)?;
    let assignments = parse_assignments(&content)?;
    let result = part_one_inner(assignments);
    println!("{result}");
    Ok(result)
}

pub fn part_two() -> Result<u32> {
    let input_path = problem_input_path(4, Some(1));
    let content = fs::read_to_string(input_path)?;
    let assignments = parse_assignments(&content)?;
    let result = part_two_inner(assignments);
    println!("{result}");
    Ok(result)
}

fn parse_assignments(input: &str) -> Result<Vec<Pair>> {
    input.trim().lines().map(str::parse).collect()
}

fn part_one_inner(assignments: Vec<Pair>) -> u32 {
    assignments
        .into_iter()
        .map(Pair::contains)
        .filter(|b| *b)
        .count() as u32
}

fn part_two_inner(assignments: Vec<Pair>) -> u32 {
    assignments
        .into_iter()
        .map(Pair::overlap)
        .filter(|b| *b)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn part_one_works() {
        let assignments = parse_assignments(TEST_INPUT).unwrap();
        assert_eq!(part_one_inner(assignments), 2);
        assert_eq!(part_one().unwrap(), 498);
    }

    #[test]
    fn part_two_works() {
        let assignments = parse_assignments(TEST_INPUT).unwrap();
        assert_eq!(part_two_inner(assignments), 4);
        assert_eq!(part_two().unwrap(), 859);
    }
}
