use crate::utils::*;
use eyre::{Report, Result};
use itertools::Itertools;
use rayon::prelude::*;
use std::{
    collections::{BTreeMap, VecDeque},
    fs,
    ops::{Add, Mul, Sub},
    str::FromStr,
};

pub fn part_one() -> Result<isize> {
    let input_path = problem_input_path(20, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_one_inner(&content)?;
    println!("{result}");
    Ok(result)
}

pub fn part_two() -> Result<isize> {
    let input_path = problem_input_path(20, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_two_inner(&content)?;
    println!("{result}");
    Ok(result)
}

fn part_one_inner(input: &str) -> Result<isize> {
    let original_value_order: Vec<isize> =
        input.trim().lines().flat_map(str::parse::<isize>).collect();
    let mut id_to_value = BTreeMap::new();
    let mut zero_id = 0;
    for (idx, value) in original_value_order.iter().enumerate() {
        if *value == 0 {
            zero_id = idx;
        }
        id_to_value.insert(idx, *value);
    }
    let mut values: VecDeque<usize> = (0..original_value_order.len()).collect();
    for id in 0..original_value_order.len() {
        let (idx, _) = values.iter().find_position(|v| **v == id).unwrap();
        values.remove(idx);
        let value = id_to_value[&id];
        let new_idx = (idx as isize + value).rem_euclid(values.len() as isize) as usize;
        values.insert(new_idx, id);
    }
    let (idx, _) = values.iter().find_position(|v| **v == zero_id).unwrap();
    Ok(id_to_value[&values[(idx + 1000) % values.len()]]
        + id_to_value[&values[(idx + 2000) % values.len()]]
        + id_to_value[&values[(idx + 3000) % values.len()]])
}

fn part_two_inner(input: &str) -> Result<isize> {
    let original_value_order: Vec<isize> = input
        .trim()
        .lines()
        .flat_map(str::parse::<isize>)
        .map(|v| v * 811589153)
        .collect();
    let mut id_to_value = BTreeMap::new();
    let mut zero_id = 0;
    for (idx, value) in original_value_order.iter().enumerate() {
        if *value == 0 {
            zero_id = idx;
        }
        id_to_value.insert(idx, *value);
    }
    let mut values: VecDeque<usize> = (0..original_value_order.len()).collect();
    for _ in 0..10 {
        for id in 0..original_value_order.len() {
            let (idx, _) = values.iter().find_position(|v| **v == id).unwrap();
            values.remove(idx);
            let value = id_to_value[&id];
            let new_idx = (idx as isize + value).rem_euclid(values.len() as isize) as usize;
            values.insert(new_idx, id);
        }
    }
    let (idx, _) = values.iter().find_position(|v| **v == zero_id).unwrap();
    Ok(id_to_value[&values[(idx + 1000) % values.len()]]
        + id_to_value[&values[(idx + 2000) % values.len()]]
        + id_to_value[&values[(idx + 3000) % values.len()]])
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
1
2
-3
3
-2
0
4
"#;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one_inner(TEST_INPUT).unwrap(), 3);
        assert_eq!(part_one().unwrap(), 4066);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two_inner(TEST_INPUT).unwrap(), 1623178306);
        assert_eq!(part_two().unwrap(), 0);
    }
}
