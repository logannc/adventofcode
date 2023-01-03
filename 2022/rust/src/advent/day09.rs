use crate::utils::*;
use derive_more::{Add, Sub};
use eyre::{ContextCompat, Report, Result};
use itertools::Itertools;
use std::{collections::BTreeMap, fs, str::FromStr};

pub fn part_one() -> Result<usize> {
    let input_path = problem_input_path(9, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_one_inner(&content)?;
    println!("{result}");
    Ok(result)
}

pub fn part_two() -> Result<usize> {
    let input_path = problem_input_path(9, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_two_inner(&content)?;
    println!("{result}");
    Ok(result)
}

fn part_one_inner(input: &str) -> Result<usize> {
    let path: RopePath = str::parse(input)?;
    Ok(path.simulate::<2>()?.len())
}

fn part_two_inner(input: &str) -> Result<usize> {
    let path: RopePath = str::parse(input)?;
    Ok(path.simulate::<10>()?.len())
}

#[derive(Copy, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Command {
    dir: Direction,
    length: usize,
}

impl FromStr for Command {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, length) = s
            .split_once(' ')
            .wrap_err_with(|| Report::msg(format!("failed to split [{s}]")))?;
        let length = str::parse(length)?;
        match direction {
            "R" => Ok(Command {
                dir: Direction::Right,
                length,
            }),
            "L" => Ok(Command {
                dir: Direction::Left,
                length,
            }),
            "U" => Ok(Command {
                dir: Direction::Up,
                length,
            }),
            "D" => Ok(Command {
                dir: Direction::Down,
                length,
            }),
            other => Err(Report::msg(format!("unknown direction [{other}]"))),
        }
    }
}

struct RopePath {
    commands: Vec<Command>,
}

impl FromStr for RopePath {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let commands: Result<Vec<Command>, _> = s.trim().lines().map(str::parse).collect();
        let commands = commands?;
        Ok(RopePath { commands })
    }
}

#[derive(Add, Sub, Debug, Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl From<Direction> for Point {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Right => Point { x: 1, y: 0 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Up => Point { x: 0, y: 1 },
            Direction::Down => Point { x: 0, y: -1 },
        }
    }
}

impl RopePath {
    fn simulate<const N: usize>(&self) -> Result<BTreeMap<Point, usize>> {
        let mut knots: [Point; N] = [Default::default(); N];
        let mut visits = BTreeMap::new();
        let last = knots.last().unwrap();
        *visits.entry(*last).or_default() += 1;
        for command in self.commands.iter() {
            let vector: Point = command.dir.into();
            for _ in 0..command.length {
                let head = knots.first_mut().unwrap();
                *head = *head + vector;
                for (left, right) in (0..N).into_iter().tuple_windows() {
                    let [left, right] = knots.get_many_mut([left, right])?;
                    move_tail(left, right);
                }
                let last = knots.last().unwrap();
                *visits.entry(*last).or_default() += 1;
            }
        }
        Ok(visits)
    }
}

fn move_tail(head: &mut Point, tail: &mut Point) {
    let Point { x, y } = *head - *tail;
    match (x, y) {
        (0, y) if y.abs() > 1 => {
            *tail = *tail
                + Point {
                    x: 0,
                    y: y.signum(),
                }
        }
        (x, 0) if x.abs() > 1 => {
            *tail = *tail
                + Point {
                    x: x.signum(),
                    y: 0,
                }
        }
        (x, y) if x.abs() > 1 || y.abs() > 1 => {
            *tail = *tail
                + Point {
                    x: x.signum(),
                    y: y.signum(),
                }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

    const TEST_INPUT_TWO: &str = r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one_inner(TEST_INPUT).unwrap(), 13);
        assert_eq!(part_one().unwrap(), 5878);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two_inner(TEST_INPUT).unwrap(), 1);
        assert_eq!(part_two_inner(TEST_INPUT_TWO).unwrap(), 36);
        assert_eq!(part_two().unwrap(), 2405);
    }
}
