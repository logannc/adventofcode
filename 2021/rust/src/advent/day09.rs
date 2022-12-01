use std::collections::BTreeSet;

use crate::utils::*;

struct Neighbors {
    items: Vec<(i32, i32)>,
}

impl Neighbors {
    fn iter(row: i32, row_max: i32, col: i32, col_max: i32) -> Neighbors {
        let mut items = Vec::new();
        if row - 1 >= 0 {
            items.push((row - 1, col));
        }
        if col - 1 >= 0 {
            items.push((row, col - 1));
        }
        if row + 1 < row_max {
            items.push((row + 1, col));
        }
        if col + 1 < col_max {
            items.push((row, col + 1));
        }
        Neighbors { items }
    }
}

impl Iterator for Neighbors {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() {
            None
        } else {
            self.items.pop()
        }
    }
}

fn risk_level(floor: Vec<Vec<i32>>) -> i32 {
    let rows = floor.len();
    let columns = floor.get(0).unwrap().len();
    let mut low_points = Vec::new();
    for row_idx in 0..rows {
        for col_idx in 0..columns {
            let value = floor[row_idx][col_idx];
            let mut pass = true;
            for (nrow, ncol) in
                Neighbors::iter(row_idx as i32, rows as i32, col_idx as i32, columns as i32)
            {
                if value >= floor[nrow as usize][ncol as usize] {
                    pass = false;
                }
            }
            if pass {
                low_points.push(value + 1);
            }
        }
    }
    low_points.iter().sum()
}

pub fn part_one() {
    let ip = problem_input_path(9, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    let floor: Vec<Vec<i32>> = lines
        .into_iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    println!("{}", risk_level(floor));
}

// TODO: why is this so slow? need to profile...
pub fn part_two() {
    let start = std::time::Instant::now();
    let ip = problem_input_path(9, Some(1));
    println!("{:?}", std::time::Instant::now() - start);
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    let floor: Vec<Vec<i32>> = lines
        .into_iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let rows = floor.len();
    let columns = floor.get(0).unwrap().len();
    let mut basins = basins(floor);
    println!("{:?}", std::time::Instant::now() - start);
    basins.sort_by(|a, b| a.len().cmp(&b.len()));
    let end = std::time::Instant::now();
    let duration = end - start;
    println!("{:?} for ({}, {})", duration, rows, columns);
}

fn collect_basin(
    floor: &Vec<Vec<i32>>,
    point: (usize, usize),
    mut accum: BTreeSet<(usize, usize)>,
) -> BTreeSet<(usize, usize)> {
    if accum.contains(&point) {
        return accum;
    } else {
        accum.insert(point);
    }
    let (row_idx, col_idx) = point;
    let rows = floor.len();
    let columns = floor.get(0).unwrap().len();
    for (nrow, ncol) in Neighbors::iter(row_idx as i32, rows as i32, col_idx as i32, columns as i32)
    {
        if floor[nrow as usize][ncol as usize] != 9 && !accum.contains(&(nrow as usize, ncol as usize)) {
            let neighbor = (nrow as usize, ncol as usize);
            accum = collect_basin(floor, neighbor, accum);
        }
    }
    accum
}

fn basins(floor: Vec<Vec<i32>>) -> Vec<BTreeSet<(usize, usize)>> {
    let mut basins = Vec::new();
    let rows = floor.len();
    let columns = floor.get(0).unwrap().len();
    for row_idx in 0..rows {
        for col_idx in 0..columns {
            let value = floor[row_idx][col_idx];
            if value == 9 {
                continue;
            }
            if !basins
                .iter()
                .any(|b: &BTreeSet<(usize, usize)>| b.contains(&(row_idx, col_idx)))
            {
                basins.push(collect_basin(&floor, (row_idx, col_idx), BTreeSet::new()));
            }
        }
    }
    basins
}

#[test]
fn example() {
    let input = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;
    let lines: Vec<String> = input.split("\n").map(|s| s.to_string()).collect();
    let floor: Vec<Vec<i32>> = lines
        .into_iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    assert_eq!(risk_level(floor.clone()), 15);
    let mut basins = basins(floor);
    basins.sort_by(|a, b| a.len().cmp(&b.len()));
    assert_eq!(
        basins
            .into_iter()
            .rev()
            .take(3)
            .inspect(|b| println!("{:?}", b))
            .map(|b| b.len())
            .product::<usize>(),
        1134
    );
}
