use crate::utils::*;
use eyre::Result;
use itertools::Itertools;
use std::{
    collections::{BTreeMap, VecDeque},
    fs,
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
    let original: Vec<isize> = input.trim().lines().flat_map(str::parse::<isize>).collect();
    let (mut values, labels, zero_id) = relabel(&original);
    mix(&mut values, &labels);
    Ok(extract(values, labels, zero_id))
}

fn part_two_inner(input: &str) -> Result<isize> {
    let original: Vec<isize> = input
        .trim()
        .lines()
        .flat_map(str::parse::<isize>)
        .map(|v| v * 811589153)
        .collect();
    let (mut values, labels, zero_id) = relabel(&original);
    for _ in 0..10 {
        mix(&mut values, &labels);
    }
    Ok(extract(values, labels, zero_id))
}

fn extract(values: VecDeque<usize>, labels: BTreeMap<usize, isize>, zero_id: usize) -> isize {
    let (idx, _) = values.iter().find_position(|v| **v == zero_id).unwrap();
    labels[&values[(idx + 1000) % values.len()]]
        + labels[&values[(idx + 2000) % values.len()]]
        + labels[&values[(idx + 3000) % values.len()]]
}

fn mix(values: &mut VecDeque<usize>, labels: &BTreeMap<usize, isize>) {
    for id in 0..values.len() {
        let (idx, _) = values.iter().find_position(|v| **v == id).unwrap();
        values.remove(idx);
        let value = labels[&id];
        let new_idx = (idx as isize + value).rem_euclid(values.len() as isize) as usize;
        values.insert(new_idx, id);
    }
}

fn relabel(original: &Vec<isize>) -> (VecDeque<usize>, BTreeMap<usize, isize>, usize) {
    let mut id_to_value = BTreeMap::new();
    let mut zero_id = 0;
    for (idx, value) in original.iter().enumerate() {
        if *value == 0 {
            zero_id = idx;
        }
        id_to_value.insert(idx, *value);
    }
    let values: VecDeque<usize> = (0..original.len()).collect();
    (values, id_to_value, zero_id)
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
        assert_eq!(part_two().unwrap(), 6704537992933);
    }
}
