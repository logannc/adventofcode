use crate::utils::*;
use eyre::{Report, Result};
use itertools::Itertools;
use std::{collections::HashMap, fs, str::FromStr};

pub fn part_one() -> Result<usize> {
    let input_path = problem_input_path(14, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_one_inner(&content)?;
    println!("{result}");
    Ok(result)
}

pub fn part_two() -> Result<usize> {
    let input_path = problem_input_path(14, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_two_inner(&content)?;
    println!("{result}");
    Ok(result)
}

fn part_one_inner(input: &str) -> Result<usize> {
    let mut grid: Grid = str::parse(input)?;
    let mut count = 0;
    while grid.add_sand(false)? {
        count += 1;
    }
    println!("{}", grid.render_grid(false));
    Ok(count)
}

fn part_two_inner(input: &str) -> Result<usize> {
    let mut grid: Grid = str::parse(input)?;
    let mut count = 0;
    while grid.add_sand(true)? {
        count += 1;
    }
    println!("{}", grid.render_grid(true));
    Ok(count)
}

struct Path {
    points: Vec<(isize, isize)>,
}

impl FromStr for Path {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<(isize, isize)> = s
            .split("->")
            .map(|point| {
                let (left, right) = point.trim().split_once(',').unwrap();
                (str::parse(left).unwrap(), str::parse(right).unwrap())
            })
            .collect();
        Ok(Path { points })
    }
}

impl Path {
    fn points(&self) -> impl Iterator<Item = (isize, isize)> {
        self.points
            .clone()
            .into_iter()
            .tuple_windows()
            .flat_map(|(left, right)| {
                let diff = (right.0 - left.0, right.1 - left.1);
                let unit_vector = (diff.0.signum(), diff.1.signum());
                let times = diff.0.abs().max(diff.1.abs());
                let mut points = vec![left];
                for _ in 0..times {
                    let previous = points.last().unwrap();
                    let next = (previous.0 + unit_vector.0, previous.1 + unit_vector.1);
                    points.push(next);
                }
                points.into_iter()
            })
    }
}

enum Material {
    Sand,
    Rock,
}

struct Grid {
    contents: HashMap<(isize, isize), Material>,
    bounds: ((isize, isize), (isize, isize)),
}

impl FromStr for Grid {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let paths: Result<Vec<Path>, _> = s.trim().lines().map(str::parse).collect();
        let mut contents = HashMap::new();
        let mut bounds = ((Grid::SAND_SPOUT.0, 0), (Grid::SAND_SPOUT.0, 0));
        for path in paths?.into_iter() {
            for point in path.points() {
                contents.insert(point, Material::Rock);
                if point.0 < bounds.0 .0 {
                    bounds.0 .0 = point.0;
                }
                // omit finding top left point's y since it'll be 0
                if point.0 > bounds.1 .0 {
                    bounds.1 .0 = point.0;
                }
                if point.1 > bounds.1 .1 {
                    bounds.1 .1 = point.1;
                }
            }
        }
        Ok(Grid { contents, bounds })
    }
}

impl Grid {
    const SAND_SPOUT: (isize, isize) = (500, 0);

    fn _contains(&self, key: &(isize, isize), with_floor: bool) -> bool {
        if with_floor && key.1 >= self.bounds.1 .1 + 2 {
            return true;
        }
        self.contents.contains_key(key)
    }

    fn add_sand(&mut self, with_floor: bool) -> Result<bool> {
        let (mut x, mut y) = Self::SAND_SPOUT;
        loop {
            if self._contains(&(x, y + 1), with_floor) {
                if self._contains(&(x - 1, y + 1), with_floor) {
                    if self._contains(&(x + 1, y + 1), with_floor) {
                        if self.contents.insert((x, y), Material::Sand).is_some() {
                            return Ok(false);
                        } else {
                            return Ok(true);
                        }
                    } else {
                        x += 1;
                        y += 1;
                    }
                } else {
                    x -= 1;
                    y += 1;
                }
            } else {
                y += 1;
            }
            if !with_floor && y >= self.bounds.1 .1 {
                return Ok(false);
            }
        }
    }

    fn render_grid(&self, with_floor: bool) -> String {
        (self.bounds.0 .1..=(self.bounds.1 .1 + if with_floor { 2 } else { 0 }))
            .into_iter()
            .map(|y| {
                (self.bounds.0 .0..=self.bounds.1 .0)
                    .into_iter()
                    .map(|x| {
                        if y == self.bounds.1 .1 + 2 {
                            '#'
                        } else if let Some(material) = self.contents.get(&(x, y)) {
                            match material {
                                Material::Rock => '#',
                                Material::Sand => 'o',
                            }
                        } else {
                            '.'
                        }
                    })
                    .join("")
            })
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one_inner(TEST_INPUT).unwrap(), 24);
        assert_eq!(part_one().unwrap(), 696);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two_inner(TEST_INPUT).unwrap(), 93);
        assert_eq!(part_two().unwrap(), 23610);
    }
}
