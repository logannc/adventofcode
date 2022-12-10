use crate::utils::*;
use eyre::{Report, Result};
use itertools::Itertools;
use std::{fmt::Display, fs, str::FromStr};

pub fn part_one() -> Result<i32> {
    let input_path = problem_input_path(10, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_one_inner(&content)?;
    println!("{}", result);
    Ok(result)
}

pub fn part_two() -> Result<String> {
    let input_path = problem_input_path(10, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_two_inner(&content)?;
    println!("{}", result);
    Ok(result)
}

#[derive(Debug, Clone)]
struct Screen {
    state: [[bool; 40]; 6],
}

impl Default for Screen {
    fn default() -> Self {
        let state = [[false; 40]; 6];
        Screen { state }
    }
}

impl From<Screen> for String {
    fn from(value: Screen) -> Self {
        value
            .state
            .into_iter()
            .map(|row| row.into_iter().map(|b| if b { '#' } else { '.' }).join(""))
            .join("\n")
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.clone().into();
        f.write_str(&s)?;
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Noop,
    AddX(i32),
}

impl FromStr for Op {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        Ok(match parts.len() {
            1 => Op::Noop,
            2 => Op::AddX(str::parse(parts[1])?),
            _ => panic!("shouldn't be any this size"),
        })
    }
}

struct CPU {
    clock: u32,
    x: i32,
    signal_strengths: Vec<i32>,
    screen: Screen,
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            clock: 0,
            x: 1,
            signal_strengths: Vec::new(),
            screen: Default::default(),
        }
    }
}

impl CPU {
    fn apply(&mut self, op: Op) {
        // TODO: this could be cleaned up and abstracted
        // factor out the containment check and drawing
        // record the clock length in the instruction so that we can loop over the cycle length
        // separate the signal strengths or provide some other inspection mechanism
        match op {
            Op::Noop => {
                if ((self.x - 1)..=(self.x + 1)).contains(&(self.clock as i32 % 40)) {
                    let (row, col) = (self.clock / 40, self.clock % 40);
                    self.screen.state[row as usize][col as usize] = true;
                }
                self.clock += 1;
                if self.clock % 40 == 20 {
                    self.signal_strengths.push(self.clock as i32 * self.x);
                }
            }
            Op::AddX(arg) => {
                if ((self.x - 1)..=(self.x + 1)).contains(&(self.clock as i32 % 40)) {
                    let (row, col) = (self.clock / 40, self.clock % 40);
                    self.screen.state[row as usize][col as usize] = true;
                }
                self.clock += 1;
                if self.clock % 40 == 20 {
                    self.signal_strengths.push(self.clock as i32 * self.x);
                }
                if ((self.x - 1)..=(self.x + 1)).contains(&(self.clock as i32 % 40)) {
                    let (row, col) = (self.clock / 40, self.clock % 40);
                    self.screen.state[row as usize][col as usize] = true;
                }
                self.clock += 1;
                if self.clock % 40 == 20 {
                    self.signal_strengths.push(self.clock as i32 * self.x);
                }
                self.x += arg;
            }
        }
    }
}

fn parse_instructions(input: &str) -> Result<Vec<Op>> {
    let ops: Result<Vec<Op>, _> = input.trim().lines().map(str::parse).collect();
    Ok(ops?)
}

fn part_one_inner(input: &str) -> Result<i32> {
    let ops = parse_instructions(input)?;
    let mut cpu = CPU::default();
    for op in ops {
        cpu.apply(op);
    }
    Ok(cpu.signal_strengths.iter().sum())
}

fn part_two_inner(input: &str) -> Result<String> {
    let ops = parse_instructions(input)?;
    let mut cpu = CPU::default();
    for op in ops {
        cpu.apply(op);
    }
    Ok(cpu.screen.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#;

    const TEST_INPUT_RESULT: &str = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#;

    const PART_TWO_RESULT: &str = r#"
###...##..#....###..###..####..##..#..#.
#..#.#..#.#....#..#.#..#....#.#..#.#..#.
#..#.#....#....#..#.###....#..#..#.#..#.
###..#.##.#....###..#..#..#...####.#..#.
#.#..#..#.#....#.#..#..#.#....#..#.#..#.
#..#..###.####.#..#.###..####.#..#..##..
"#;
    #[test]
    fn part_one_works() {
        assert_eq!(part_one_inner(TEST_INPUT).unwrap(), 13140);
        assert_eq!(part_one().unwrap(), 14420);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(
            part_two_inner(TEST_INPUT).unwrap(),
            TEST_INPUT_RESULT.trim()
        );
        assert_eq!(part_two().unwrap(), PART_TWO_RESULT.trim());
    }
}
