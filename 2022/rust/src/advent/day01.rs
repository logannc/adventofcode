use crate::utils::*;
use eyre::{Result, WrapErr};
use itertools::Itertools;
use std::fs;

type Calories = u32;
type Inventory = Vec<Calories>;
type Caravan = Vec<Inventory>;

pub fn part_one() -> Result<u32> {
    let input_path = problem_input_path(1, Some(1));
    let content = fs::read_to_string(input_path)?;
    let caravan = parse_caravan(&content)?;
    let result = part_one_inner(caravan);
    println!("{}", result);
    Ok(result)
}

pub fn part_two() -> Result<u32> {
    let input_path = problem_input_path(1, Some(1));
    let content = fs::read_to_string(input_path)?;
    let caravan = parse_caravan(&content)?;
    let result = part_two_inner(caravan);
    println!("{}", result);
    Ok(result)
}

fn parse_caravan(input: &str) -> Result<Caravan> {
    let caravan: Result<Caravan, _> = input
        .trim()
        .split("\n\n")
        .map(|group| {
            group
                .trim()
                .split("\n")
                .map(|s| {
                    s.parse::<Calories>()
                        .wrap_err_with(|| format!("Failed to parse [{}]", s))
                })
                .collect()
        })
        .collect();
    caravan.map_err(|e| e.into())
}

fn part_one_inner(caravan: Caravan) -> u32 {
    caravan
        .into_iter()
        .map(|inventory| inventory.into_iter().sum())
        .max()
        .unwrap()
}

fn part_two_inner(caravan: Caravan) -> u32 {
    caravan
        .into_iter()
        .map(|inventory| inventory.into_iter().sum::<u32>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn part_one_works() {
        let caravan = parse_caravan(TEST_INPUT).unwrap();
        assert_eq!(part_one_inner(caravan), 24000);
        assert_eq!(part_one().unwrap(), 69883);
    }

    #[test]
    fn part_two_works() {
        let caravan = parse_caravan(TEST_INPUT).unwrap();
        assert_eq!(part_two_inner(caravan), 45000);
        assert_eq!(part_two().unwrap(), 207576);
    }
}
