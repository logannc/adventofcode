use crate::utils::errors::Error;
use crate::utils::files::{problem_input_path, read_file_split_whitespace};

fn fuel_cost(weight: f32) -> u32 {
    ((weight / 3.0) as u32).checked_sub(2).unwrap_or(0)
}

fn iterative_cost(weight: f32) -> u32 {
    let mut cost = fuel_cost(weight);
    let mut additional_cost = fuel_cost(cost as f32);
    while additional_cost > 0 {
        cost += additional_cost;
        additional_cost = fuel_cost(additional_cost as f32);
    }
    cost
}

pub fn part_one() -> Result<u32, Error> {
    let input_path = problem_input_path(1, None);
    let numbers = read_file_split_whitespace(&input_path)?;
    let sum = numbers.into_iter().map(fuel_cost).sum();
    Ok(sum)
}

pub fn part_two() -> Result<u32, Error> {
    let input_path = problem_input_path(1, None);
    let numbers = read_file_split_whitespace(&input_path)?;
    let sum = numbers.into_iter().map(iterative_cost).sum();
    Ok(sum)
}
