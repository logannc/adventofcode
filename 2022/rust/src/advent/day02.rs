use crate::utils::*;
use eyre::{Report, Result, WrapErr};
use std::{fs, str::FromStr};

// TODO: impl From<(Shape, Shape)> for Outcome instead of using PartialOrd which should have been Ord anyway

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            other => Err(Report::msg(other.to_string())),
        }
    }
}

impl From<Outcome> for u32 {
    fn from(value: Outcome) -> Self {
        match value {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            other => Err(Report::msg(other.to_string())),
        }
    }
}

impl From<Shape> for u32 {
    fn from(value: Shape) -> Self {
        match value {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (&Shape::Rock, &Shape::Rock) => Some(std::cmp::Ordering::Equal),
            (&Shape::Rock, &Shape::Paper) => Some(std::cmp::Ordering::Less),
            (&Shape::Rock, &Shape::Scissors) => Some(std::cmp::Ordering::Greater),
            (&Shape::Paper, &Shape::Paper) => Some(std::cmp::Ordering::Equal),
            (&Shape::Paper, &Shape::Scissors) => Some(std::cmp::Ordering::Less),
            (&Shape::Scissors, &Shape::Scissors) => Some(std::cmp::Ordering::Equal),
            (a, b) => Some(PartialOrd::partial_cmp(b, a).unwrap().reverse()),
        }
    }
}

type Round = (Shape, Shape);
type Strategy = Vec<Round>;

pub fn part_one() -> Result<u32> {
    let input_path = problem_input_path(2, Some(1));
    let content = fs::read_to_string(input_path)?;
    let strategy = parse_strategy_one(&content)?;
    let result = part_one_inner(strategy);
    println!("{result}");
    Ok(result)
}

pub fn part_two() -> Result<u32> {
    let input_path = problem_input_path(2, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_two_inner(&content)?;
    println!("{result}");
    Ok(result)
}

fn parse_strategy_one(input: &str) -> Result<Strategy> {
    let mut strategy = Vec::new();
    for [opponent, mine] in input.split_whitespace().array_chunks() {
        strategy.push((
            str::parse(opponent).wrap_err_with(|| format!("failed to parse [{opponent}]"))?,
            str::parse(mine).wrap_err_with(|| format!("failed to parse [{mine}]"))?,
        ));
    }
    Ok(strategy)
}

fn resolve_round(opponent: Shape, mine: Shape) -> u32 {
    let move_score: u32 = mine.into();
    let result_score: u32 = match PartialOrd::partial_cmp(&mine, &opponent).unwrap() {
        std::cmp::Ordering::Greater => 6,
        std::cmp::Ordering::Equal => 3,
        std::cmp::Ordering::Less => 0,
    };
    move_score + result_score
}

fn part_one_inner(strategy: Strategy) -> u32 {
    strategy
        .into_iter()
        .map(|(opponent, mine)| resolve_round(opponent, mine))
        .sum()
}

fn shape_from_outcome(opponent: Shape, outcome: Outcome) -> Shape {
    match (opponent, outcome) {
        (any, Outcome::Draw) => any,
        (Shape::Rock, Outcome::Win) => Shape::Paper,
        (Shape::Rock, Outcome::Lose) => Shape::Scissors,
        (Shape::Paper, Outcome::Win) => Shape::Scissors,
        (Shape::Paper, Outcome::Lose) => Shape::Rock,
        (Shape::Scissors, Outcome::Win) => Shape::Rock,
        (Shape::Scissors, Outcome::Lose) => Shape::Paper,
    }
}

fn part_two_inner(input: &str) -> Result<u32> {
    let mut score = 0;
    for [opponent, mine] in input.split_whitespace().array_chunks() {
        let opponent_shape: Shape =
            str::parse(opponent).wrap_err_with(|| format!("failed to parse [{opponent}]"))?;
        let desired_outcome: Outcome =
            str::parse(mine).wrap_err_with(|| format!("failed to parse [{mine}]"))?;
        let desired_shape = shape_from_outcome(opponent_shape, desired_outcome);
        let outcome_score: u32 = desired_outcome.into();
        let shape_score: u32 = desired_shape.into();
        score += outcome_score + shape_score;
    }
    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
A Y
B X
C Z"#;

    #[test]
    fn part_one_works() {
        let strategy = parse_strategy_one(TEST_INPUT).unwrap();
        assert_eq!(part_one_inner(strategy), 15);
        assert_eq!(part_one().unwrap(), 10941);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two_inner(TEST_INPUT).unwrap(), 12);
        assert_eq!(part_two().unwrap(), 13071);
    }
}
