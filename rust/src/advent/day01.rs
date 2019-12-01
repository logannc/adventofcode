
// use crate::utils::files::

use std::fs;
use crate::utils::errors::Error;

fn fuel_cost(weight: f32) -> u32 {
    ((weight / 3.0) as u32).checked_sub(2).unwrap_or(0)
}

fn recursive_cost(weight: f32) -> u32 {
    let mut cost = fuel_cost(weight);
    let mut additional_cost = fuel_cost(cost as f32);
    while additional_cost > 0 {
        cost += additional_cost;
        additional_cost = fuel_cost(additional_cost as f32);
    }
    cost
}

pub fn part_one() -> Result<u32, Error> {
    let content = fs::read_to_string("../advent_problems/day01/input")?;
    let numbers: Result<Vec<f32>, _> = content.split_whitespace().map(str::parse::<f32>).collect();
    let sum = numbers?.into_iter().map(fuel_cost).sum();
    Ok(sum)
}

pub fn part_two() -> Result<u32, Error> {
    let content = fs::read_to_string("../advent_problems/day01/input")?;
    let numbers: Result<Vec<f32>, _> = content.split_whitespace().map(str::parse::<f32>).collect();
    let sum = numbers?.into_iter().map(recursive_cost).sum();
    Ok(sum)
}