use crate::utils::*;
use eyre::{ContextCompat, Report, Result};
use itertools::Itertools;
use std::{fs, mem, str::FromStr};

pub fn part_one() -> Result<usize> {
    let input_path = problem_input_path(11, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_one_inner(&content)?;
    println!("{}", result);
    Ok(result)
}

pub fn part_two() -> Result<usize> {
    let input_path = problem_input_path(11, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_two_inner(&content)?;
    println!("{}", result);
    Ok(result)
}

fn part_one_inner(input: &str) -> Result<usize> {
    monkey_business::<20, true>(input)
}

fn part_two_inner(input: &str) -> Result<usize> {
    monkey_business::<10000, false>(input)
}

fn monkey_business<const ROUNDS: usize, const DIMINISH: bool>(input: &str) -> Result<usize> {
    let mut barrel: Barrel = str::parse(input)?;
    for _ in 0..ROUNDS {
        barrel.simulate_round::<DIMINISH>();
    }
    Ok(barrel
        .monkeys
        .into_iter()
        .map(|m| m.inspection_count)
        .sorted()
        .rev()
        .take(2)
        .product())
}

enum Argument {
    Literal(usize),
    Old,
}

impl FromStr for Argument {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match str::parse(s) {
            Ok(lit) => Ok(Argument::Literal(lit)),
            Err(_) => {
                if s == "old" {
                    Ok(Argument::Old)
                } else {
                    Err(Report::msg(format!("unknown argument [{}]", s)))
                }
            }
        }
    }
}

enum Operation {
    Add(Argument),
    Multiply(Argument),
}

// TODO: would be sliiiightly cleaner as (Op, Arg)
impl Operation {
    fn apply(&self, worry: usize) -> usize {
        match self {
            Operation::Add(arg) => match arg {
                Argument::Literal(lit) => worry + lit,
                Argument::Old => worry + worry,
            },
            Operation::Multiply(arg) => match arg {
                Argument::Literal(lit) => worry * lit,
                Argument::Old => worry * worry,
            },
        }
    }
}

impl FromStr for Operation {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let last_two: Vec<&str> = s.split_whitespace().rev().take(2).collect();
        let [arg, op]: [&str; 2] = last_two.try_into().unwrap();
        let arg = str::parse(arg)?;
        match op {
            "+" => Ok(Operation::Add(arg)),
            "*" => Ok(Operation::Multiply(arg)),
            _ => Err(Report::msg(format!("unknown op [{}] from [{}]", op, s))),
        }
    }
}

struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    result: (usize, usize),
    inspection_count: usize,
}

impl FromStr for Monkey {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        // skip monkey number line
        lines.next().unwrap();
        let item_line = lines.next().unwrap();
        let (_, items) = item_line.split_once(':').wrap_err_with(|| {
            Report::msg(format!(
                "Failed to split on : for item line [{}]",
                item_line
            ))
        })?;
        let items: Result<Vec<usize>, _> = items
            .split(",")
            .into_iter()
            .map(|item| str::parse(item.trim()))
            .collect();
        let items = items?;
        let operation = str::parse(lines.next().unwrap())?;
        let test = str::parse(lines.next().unwrap().split_whitespace().last().unwrap())?;
        let true_result = str::parse(lines.next().unwrap().split_whitespace().last().unwrap())?;
        let false_result = str::parse(lines.next().unwrap().split_whitespace().last().unwrap())?;
        Ok(Monkey {
            items,
            operation,
            test,
            result: (true_result, false_result),
            inspection_count: 0,
        })
    }
}

struct Barrel {
    monkeys: Vec<Monkey>,
}

impl FromStr for Barrel {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys: Result<Vec<Monkey>, _> = s.trim().split("\n\n").map(str::parse).collect();
        let monkeys = monkeys?;
        Ok(Barrel { monkeys })
    }
}

impl Barrel {
    fn simulate_round<const DIMINISH: bool>(&mut self) {
        let p: usize = self.monkeys.iter().map(|m| m.test).product();
        for monkey_idx in 0..self.monkeys.len() {
            let items = mem::replace(&mut self.monkeys[monkey_idx].items, Vec::new());
            for item in items {
                self.monkeys[monkey_idx].inspection_count += 1;
                let worry = self.monkeys[monkey_idx].operation.apply(item);
                let worry = if DIMINISH { worry / 3 } else { worry % p };
                let result = worry % self.monkeys[monkey_idx].test == 0;
                let target = if result {
                    self.monkeys[monkey_idx].result.0
                } else {
                    self.monkeys[monkey_idx].result.1
                };
                self.monkeys[target].items.push(worry);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one_inner(TEST_INPUT).unwrap(), 10605);
        assert_eq!(part_one().unwrap(), 99852);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two_inner(TEST_INPUT).unwrap(), 2713310158);
        assert_eq!(part_two().unwrap(), 25935263541);
    }
}
