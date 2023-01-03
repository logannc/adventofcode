use crate::utils::*;
use eyre::{Report, Result};
use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    fs,
    str::FromStr,
};

pub fn part_one() -> Result<usize> {
    let input_path = problem_input_path(12, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_one_inner(&content)?;
    println!("{result}");
    Ok(result)
}

pub fn part_two() -> Result<usize> {
    let input_path = problem_input_path(12, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_two_inner(&content)?;
    println!("{result}");
    Ok(result)
}

fn part_one_inner(input: &str) -> Result<usize> {
    let grid: Grid = str::parse(input)?;
    let path = grid.find_shortest_path_up()?;
    println!("{}", grid.render_path(&path)?);
    Ok(path.path.len() - 1)
}

fn part_two_inner(input: &str) -> Result<usize> {
    let grid: Grid = str::parse(input)?;
    let path = grid.find_shortest_path_down()?;
    println!("{}", grid.render_path(&path)?);
    Ok(path.path.len() - 1)
}

#[derive(Clone, Debug)]
struct Path {
    path: Vec<(usize, usize)>,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<usize>>,
    start: (usize, usize),
    end: (usize, usize),
    dim: (usize, usize),
}

impl FromStr for Grid {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim().lines();
        let mut grid = Vec::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (row_idx, line) in lines.enumerate() {
            let mut row = Vec::new();
            for (col_idx, c) in line.char_indices() {
                if c == 'S' {
                    start = (row_idx, col_idx);
                    row.push('a' as usize);
                } else if c == 'E' {
                    end = (row_idx, col_idx);
                    row.push('z' as usize);
                } else {
                    row.push(c as usize);
                }
            }
            grid.push(row);
        }
        let dim = (grid.len(), grid[0].len());
        Ok(Grid {
            grid,
            start,
            end,
            dim,
        })
    }
}

impl Grid {
    fn render_path(&self, path: &Path) -> Result<String> {
        let row: Vec<char> = std::iter::repeat('.').take(self.dim.1).collect();
        let mut grid: Vec<Vec<char>> = std::iter::repeat(row).take(self.dim.0).collect();
        for point in path.path.iter() {
            grid[point.0][point.1] = char::from_u32(self.grid[point.0][point.1] as u32).unwrap();
        }
        let point = path.path.last().unwrap();
        grid[point.0][point.1] = char::from_u32(self.grid[point.0][point.1] as u32)
            .unwrap()
            .to_ascii_uppercase();
        Ok(grid
            .into_iter()
            .map(|row| row.into_iter().join(""))
            .join("\n"))
    }
    fn up_neighbors(&self, point: &(usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let height = self.grid[point.0][point.1];
        let cardinals = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let neighbors: Vec<(usize, usize)> = cardinals
            .into_iter()
            .map(move |direction| {
                (
                    point.0.wrapping_add_signed(direction.0),
                    point.1.wrapping_add_signed(direction.1),
                )
            })
            .filter(move |neighbor| {
                if let Some(&neighbor_height) = self
                    .grid
                    .get(neighbor.0)
                    .and_then(|row| row.get(neighbor.1))
                {
                    neighbor_height as isize - height as isize <= 1
                } else {
                    false
                }
            })
            .collect();
        neighbors.into_iter()
    }
    fn down_neighbors(&self, point: &(usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let height = self.grid[point.0][point.1];
        let cardinals = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let neighbors: Vec<(usize, usize)> = cardinals
            .into_iter()
            .map(move |direction| {
                (
                    point.0.wrapping_add_signed(direction.0),
                    point.1.wrapping_add_signed(direction.1),
                )
            })
            .filter(move |neighbor| {
                if let Some(&neighbor_height) = self
                    .grid
                    .get(neighbor.0)
                    .and_then(|row| row.get(neighbor.1))
                {
                    height as isize - neighbor_height as isize <= 1
                } else {
                    false
                }
            })
            .collect();
        neighbors.into_iter()
    }
    fn find_shortest_path_up(&self) -> Result<Path> {
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let mut frontier: VecDeque<Path> = vec![Path {
            path: vec![self.start],
        }]
        .into();
        loop {
            if let Some(mut path) = frontier.pop_front() {
                let most_recent = path.path.last().unwrap();
                for neighbor in self.up_neighbors(most_recent) {
                    if !seen.insert(neighbor) {
                        continue;
                    } else if neighbor == self.end {
                        path.path.push(neighbor);
                        return Ok(path);
                    } else if !path.path.contains(&neighbor) {
                        let mut new_path = path.clone();
                        new_path.path.push(neighbor);
                        frontier.push_back(new_path);
                    }
                }
            } else {
                return Err(Report::msg("path could not be found"));
            }
        }
    }

    fn find_shortest_path_down(&self) -> Result<Path> {
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let mut frontier: VecDeque<Path> = vec![Path {
            path: vec![self.end],
        }]
        .into();
        loop {
            if let Some(mut path) = frontier.pop_front() {
                let most_recent = path.path.last().unwrap();
                for neighbor in self.down_neighbors(most_recent) {
                    let neighbor_height = self.grid[neighbor.0][neighbor.1];
                    if !seen.insert(neighbor) {
                        continue;
                    } else if neighbor_height == 'a' as usize {
                        path.path.push(neighbor);
                        return Ok(path);
                    } else if !path.path.contains(&neighbor) {
                        let mut new_path = path.clone();
                        new_path.path.push(neighbor);
                        frontier.push_back(new_path);
                    }
                }
            } else {
                return Err(Report::msg("path could not be found"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one_inner(TEST_INPUT).unwrap(), 31);
        assert_eq!(part_one().unwrap(), 497);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two_inner(TEST_INPUT).unwrap(), 29);
        assert_eq!(part_two().unwrap(), 492);
    }
}
