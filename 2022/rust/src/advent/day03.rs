use crate::utils::*;
use eyre::Result;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Default)]
struct Compartment {
    items: HashMap<char, u32>,
}

impl From<&str> for Compartment {
    fn from(value: &str) -> Self {
        let mut items = HashMap::new();
        for c in value.chars() {
            *(items.entry(c).or_default()) += 1;
        }
        Compartment { items }
    }
}

struct Rucksack {
    left: Compartment,
    right: Compartment,
}

impl Rucksack {
    fn shared_item(&self) -> char {
        let left_set: HashSet<char> = self.left.items.keys().copied().collect();
        let right_set: HashSet<char> = self.right.items.keys().copied().collect();
        left_set.intersection(&right_set).next().unwrap().clone()
    }

    fn item_set(&self) -> HashSet<char> {
        let left_set: HashSet<char> = self.left.items.keys().copied().collect();
        let right_set: HashSet<char> = self.right.items.keys().copied().collect();
        left_set.union(&right_set).copied().collect()
    }
}

trait Priority {
    fn priority(self) -> u32;
}

impl Priority for char {
    fn priority(self) -> u32 {
        if self.is_alphabetic() {
            if self.is_uppercase() {
                (self as u32) - 38
            } else {
                (self as u32) - 96
            }
        } else {
            panic!("expected only alphabetic characters! found {}", self)
        }
    }
}

pub fn part_one() -> Result<()> {
    let input_path = problem_input_path(3, Some(1));
    let content = fs::read_to_string(input_path)?;
    let rucksacks = parse_rucksacks(&content);
    let result = part_one_inner(rucksacks);
    println!("{}", result);
    Ok(())
}

pub fn part_two() -> Result<()> {
    let input_path = problem_input_path(3, Some(1));
    let content = fs::read_to_string(input_path)?;
    let rucksacks = parse_rucksacks(&content);
    let result = part_two_inner(rucksacks);
    println!("{}", result);
    Ok(())
}

fn parse_rucksacks(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|line| {
            let line = line.trim();
            let length = line.len();
            let (left, right) = line.split_at(length / 2);
            let left_compartment: Compartment = left.into();
            let right_compartment: Compartment = right.into();
            Rucksack {
                left: left_compartment,
                right: right_compartment,
            }
        })
        .collect()
}

fn part_one_inner(rucksacks: Vec<Rucksack>) -> u32 {
    rucksacks
        .iter()
        .map(Rucksack::shared_item)
        .map(Priority::priority)
        .sum()
}

fn part_two_inner(rucksacks: Vec<Rucksack>) -> u32 {
    rucksacks
        .chunks(3)
        .map(|group| {
            group
                .into_iter()
                .map(Rucksack::item_set)
                .reduce(|a, b| a.intersection(&b).copied().collect::<HashSet<char>>())
                .unwrap()
                .into_iter()
                .next()
                .unwrap()
                .clone()
                .priority()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn part_one_works() {
        let rucksacks = parse_rucksacks(TEST_INPUT);
        assert_eq!(part_one_inner(rucksacks), 157);
    }

    #[test]
    fn part_two_works() {
        let rucksacks = parse_rucksacks(TEST_INPUT);
        assert_eq!(part_two_inner(rucksacks), 70);
    }
}
