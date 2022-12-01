use itertools::{Itertools};
use std::{fmt::Display, fs};

use crate::utils::*;

type Octopus = usize;

// it's a pod of octopii!
struct OctoPod<const N: usize> {
    octopuses: [[Octopus; N]; N],
    flash_count: usize,
}

impl<const N: usize> Display for OctoPod<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.octopuses
                .iter()
                .map(|row| row.iter().join(""))
                .join("\n")
        )
    }
}

impl<const N: usize> TryFrom<String> for OctoPod<N> {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut octopuses = [[Default::default(); N]; N];
        for (row, line) in value.split_whitespace().enumerate() {
            if row >= N {
                return Err(());
            }
            for (column, char) in line.chars().enumerate() {
                if column >= N {
                    return Err(());
                }
                if let Some(energy_level) = char.to_digit(10) {
                    octopuses[row][column] = energy_level as usize;
                } else {
                    return Err(());
                }
            }
        }
        Ok(Self::new(octopuses))
    }
}

impl<const N: usize> OctoPod<N> {
    fn new(octopuses: [[Octopus; N]; N]) -> Self {
        Self {
            octopuses,
            flash_count: 0,
        }
    }
    fn neighbors(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
        (-1 as i32..=1 as i32)
            .cartesian_product(-1 as i32..=1 as i32)
            .map(move |(xdelta, ydelta)| (row as i32 + xdelta, col as i32 + ydelta))
            .filter(move |(r, c)| {
                (0..N as i32).contains(r)
                    && (0..N as i32).contains(c)
                    && (row as i32, col as i32) != (*r, *c)
            })
            .map(|(r, c)| (r as usize, c as usize))
    }
    fn step(&mut self) -> bool {
        self.octopuses
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|o| *o += 1));
        let mut has_flashed = [[false; N]; N];
        let mut flash_occurred = true;
        while flash_occurred {
            flash_occurred = false;
            for row in 0..N {
                for col in 0..N {
                    if self.octopuses[row][col] > 9 && !has_flashed[row][col] {
                        flash_occurred = true;
                        has_flashed[row][col] = true;
                        for (nrow, ncol) in self.neighbors(row, col) {
                            self.octopuses[nrow][ncol] += 1;
                        }
                    }
                }
            }
        }
        for row in 0..N {
            for col in 0..N {
                if has_flashed[row][col] {
                    self.flash_count += 1;
                    self.octopuses[row][col] = 0;
                }
            }
        }
        has_flashed.iter().flatten().all(|b| *b)
    }
    fn flashes(&self) -> usize {
        self.flash_count
    }
}

pub fn part_one() {
    let ip = problem_input_path(11, Some(1));
    let mut pod: OctoPod<10> = fs::read_to_string(&ip).unwrap().try_into().unwrap();
    for _ in 0..100 {
        pod.step();
    }
    println!("{}", pod.flashes());
}

pub fn part_two() {
    let ip = problem_input_path(11, Some(1));
    let mut pod: OctoPod<10> = fs::read_to_string(&ip).unwrap().try_into().unwrap();
    let mut step = 1;
    while !pod.step() {
        step += 1;
    }
    println!("{}", step);
}

#[test]
fn example() {
    let example = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
    let mut pod: OctoPod<10> = example.to_string().try_into().unwrap();
    for _ in 0..10 {
        pod.step();
    }
    assert_eq!(pod.flashes(), 204);
    for _ in 0..90 {
        pod.step();
    }
    assert_eq!(pod.flashes(), 1656);
}
