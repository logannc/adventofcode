use crate::utils::*;
use eyre::{Context, ContextCompat, Report, Result};
use itertools::Itertools;
use std::{fs, str::FromStr};

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl FromStr for Stacks {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().rev();
        let stack_count = lines.next().unwrap().split_whitespace().count();
        let mut stacks = Vec::with_capacity(stack_count);
        for _ in 0..stack_count {
            stacks.push(Vec::new());
        }
        for line in lines {
            for (i, mut chars) in line.chars().chunks(4).into_iter().enumerate() {
                if let Some(c) = chars.find(|c| c.is_alphabetic()) {
                    stacks[i].push(c)
                }
            }
        }
        Ok(Stacks { stacks })
    }
}

impl Stacks {
    fn apply_command(&mut self, command: &Command) -> Result<()> {
        let [from, to] = self
            .stacks
            .get_many_mut([command.from, command.to])
            .wrap_err_with(|| Report::msg("failed to get the from/to vectors".to_string()))?;
        for _ in 0..command.amt {
            let tmp = from.pop().wrap_err_with(|| Report::msg(String::new()))?;
            to.push(tmp);
        }
        Ok(())
    }

    fn apply_command_multiple(&mut self, command: &Command) -> Result<()> {
        let [from, to] = self
            .stacks
            .get_many_mut([command.from, command.to])
            .wrap_err_with(|| Report::msg("failed to get the from/to vectors".to_string()))?;
        let moving = from.split_off(from.len() - command.amt);
        to.extend(moving);
        Ok(())
    }
}

#[derive(Debug)]
struct Command {
    from: usize,
    to: usize,
    amt: usize,
}

impl FromStr for Command {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command_parts: Vec<usize> = s
            .split_whitespace()
            .map(str::parse::<usize>)
            .filter(|v| v.is_ok())
            .map(Result::unwrap)
            .collect();
        let [amt, from, to]: [usize; 3] = command_parts
            .try_into()
            .map_err(|o| Report::msg(format!("incorrect number of parts [{o:?}]")))?;
        Ok(Command {
            from: from - 1,
            to: to - 1,
            amt,
        })
    }
}

#[derive(Debug)]
struct Commands {
    commands: Vec<Command>,
}

impl FromStr for Commands {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let commands: Result<Vec<Command>, _> = s.lines().map(str::parse).collect();
        Ok(Commands {
            commands: commands?,
        })
    }
}

pub fn part_one() -> Result<String> {
    let input_path = problem_input_path(5, Some(1));
    let content = fs::read_to_string(input_path)?;
    let (stacks, commands) = parse_input(&content)?;
    let result = part_one_inner(stacks, commands)?;
    println!("{result}");
    Ok(result)
}

pub fn part_two() -> Result<String> {
    let input_path = problem_input_path(5, Some(1));
    let content = fs::read_to_string(input_path)?;
    let (stacks, commands) = parse_input(&content)?;
    let result = part_two_inner(stacks, commands)?;
    println!("{result}");
    Ok(result)
}

fn part_one_inner(mut stacks: Stacks, commands: Commands) -> Result<String> {
    for command in commands.commands {
        stacks.apply_command(&command)?;
    }
    Ok(stacks
        .stacks
        .into_iter()
        .map(|mut v| v.pop().unwrap())
        .collect())
}

fn part_two_inner(mut stacks: Stacks, commands: Commands) -> Result<String> {
    for command in commands.commands {
        stacks.apply_command_multiple(&command)?;
    }
    Ok(stacks
        .stacks
        .into_iter()
        .map(|mut v| v.pop().unwrap())
        .collect())
}

fn parse_input(input: &str) -> Result<(Stacks, Commands)> {
    let (stack_input, command_input) = input.split_once("\n\n").wrap_err_with(|| {
        Report::msg("input didn't split by double newline correctly".to_string())
    })?;
    let stacks = str::parse(stack_input)?;
    let commands = str::parse(command_input)?;
    Ok((stacks, commands))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn part_one_works() {
        let (stacks, commands) = parse_input(TEST_INPUT).unwrap();
        assert_eq!(part_one_inner(stacks, commands).unwrap(), "CMZ");
        assert_eq!(part_one().unwrap(), "DHBJQJCCW");
    }

    #[test]
    fn part_two_works() {
        let (stacks, commands) = parse_input(TEST_INPUT).unwrap();
        assert_eq!(part_two_inner(stacks, commands).unwrap(), "MCD");
        assert_eq!(part_two().unwrap(), "WJVRLSJJT");
    }
}
