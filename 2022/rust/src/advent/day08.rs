use crate::utils::*;
use eyre::{ContextCompat, Report, Result};
use rayon::prelude::*;
use std::{fs, str::FromStr};

#[derive(Debug)]
struct VisibilityGrid {
    content: Vec<Vec<(u8, bool)>>,
}

impl FromStr for VisibilityGrid {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim().lines();
        let content: Vec<Vec<(u8, bool)>> = lines
            .map(|line| {
                line.chars()
                    .map(|c| (c.to_digit(10).expect("expected all u8s") as u8, false))
                    .collect()
            })
            .collect();
        Ok(VisibilityGrid { content })
    }
}

impl VisibilityGrid {
    fn row_wise_idx_generator(
        &self,
    ) -> impl Iterator<Item = impl DoubleEndedIterator<Item = (usize, usize)>> {
        let rows = self.content.len();
        let columns = self.content[0].len();
        (0..rows).into_iter().map(move |row_idx| {
            (0..columns)
                .into_iter()
                .map(move |col_idx| (row_idx, col_idx))
        })
    }

    fn column_wise_idx_generator(
        &self,
    ) -> impl Iterator<Item = impl DoubleEndedIterator<Item = (usize, usize)>> {
        let rows = self.content.len();
        let columns = self.content[0].len();
        (0..columns)
            .into_iter()
            .map(move |col_idx| (0..rows).into_iter().map(move |row_idx| (row_idx, col_idx)))
    }

    fn items(self) -> impl Iterator<Item = (u8, bool)> {
        self.content
            .into_iter()
            .map(|row| row.into_iter())
            .flatten()
    }
}

#[derive(Debug)]
struct SightGrid {
    content: Vec<Vec<char>>,
}

impl FromStr for SightGrid {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim().lines();
        let content: Vec<Vec<char>> = lines
            .map(|line| {
                line.chars()
                    // .map(|c| c.to_digit(10).expect("expected all u8s") as u8)
                    .collect()
            })
            .collect();
        Ok(SightGrid { content })
    }
}

impl SightGrid {
    fn idx_generator(&self) -> impl Iterator<Item = (usize, usize)> {
        let rows = self.content.len();
        let columns = self.content[0].len();
        (0..rows)
            .into_iter()
            .map(move |row_idx| {
                (0..columns)
                    .into_iter()
                    .map(move |col_idx| (row_idx, col_idx))
            })
            .flatten()
    }

    fn cardinal_idx_generator(
        &self,
        r: usize,
        c: usize,
    ) -> impl Iterator<Item = Box<dyn Iterator<Item = (usize, usize)>>> {
        let rows = self.content.len();
        let columns = self.content[0].len();
        let iterators: Vec<Box<dyn Iterator<Item = (usize, usize)>>> = vec![
            Box::new((0..r).rev().into_iter().map(move |row_idx| (row_idx, c))),
            Box::new((r + 1..rows).into_iter().map(move |row_idx| (row_idx, c))),
            Box::new((0..c).rev().into_iter().map(move |col_idx| (r, col_idx))),
            Box::new(
                (c + 1..columns)
                    .into_iter()
                    .map(move |col_idx| (r, col_idx)),
            ),
        ];
        iterators.into_iter()
    }
}

pub fn part_one() -> Result<usize> {
    let input_path = problem_input_path(8, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_one_inner(&content)?;
    println!("{}", result);
    Ok(result)
}

pub fn part_two() -> Result<u32> {
    let input_path = problem_input_path(8, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_two_inner(&content)?;
    println!("{}", result);
    Ok(result)
}

fn part_one_inner(input: &str) -> Result<usize> {
    let mut grid: VisibilityGrid = str::parse(input)?;
    for row_wise in grid.row_wise_idx_generator() {
        traverse_iter(&mut grid, row_wise);
    }
    for row_wise in grid.row_wise_idx_generator() {
        traverse_iter(&mut grid, row_wise.rev());
    }
    for col_wise in grid.column_wise_idx_generator() {
        traverse_iter(&mut grid, col_wise);
    }
    for col_wise in grid.column_wise_idx_generator() {
        traverse_iter(&mut grid, col_wise.rev());
    }
    Ok(grid.items().filter(|(_, visible)| *visible).count())
}

fn part_two_inner(input: &str) -> Result<u32> {
    let grid: SightGrid = str::parse(input)?;
    grid.idx_generator()
        .par_bridge()
        .map(|(r, c)| {
            let iterators = grid.cardinal_idx_generator(r, c);
            let tree_height = grid.content[r][c];
            iterators
                .map(|cardinal| {
                    let mut score = 0;
                    for (nr, nc) in cardinal {
                        let neighbor_height = grid.content[nr][nc];
                        score += 1;
                        if neighbor_height >= tree_height {
                            break;
                        }
                    }
                    score
                })
                .product()
            // let mut scores = Vec::with_capacity(4);
            // for cardinal in iterators {
            //     let mut score = 0;
            //     for (nr, nc) in cardinal {
            //         let neighbor_height = grid.content[nr][nc];
            //         score += 1;
            //         if neighbor_height >= tree_height {
            //             break;
            //         }
            //     }
            //     scores.push(score);
            // }
            // scores.into_iter().product()
        })
        .max()
        .wrap_err_with(|| Report::msg("expect nonempty"))

    // let mut max_score = 0;
    // for (r, c) in grid.idx_generator() {
    //     let iterators = grid.cardinal_idx_generator(r, c);
    //     let tree_height = grid.content[r][c];
    //     let mut scores = Vec::with_capacity(4);
    //     for cardinal in iterators {
    //         let mut score = 0;
    //         for (nr, nc) in cardinal {
    //             let neighbor_height = grid.content[nr][nc];
    //             score += 1;
    //             if neighbor_height >= tree_height {
    //                 break;
    //             }
    //         }
    //         scores.push(score);
    //     }
    //     let score = scores.into_iter().product();
    //     if score > max_score {
    //         max_score = score;
    //     }
    // }
    // Ok(max_score)
}

fn traverse_iter<'a>(grid: &mut VisibilityGrid, iter: impl Iterator<Item = (usize, usize)>) {
    let mut max_height: i8 = -1;
    for (row_idx, col_idx) in iter {
        let (tree_height, visible) = &mut grid.content[row_idx][col_idx];
        if *tree_height as i8 > max_height {
            *visible = true;
            max_height = *tree_height as i8;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
30373
25512
65332
33549
35390
"#;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one_inner(TEST_INPUT).unwrap(), 21);
        assert_eq!(part_one().unwrap(), 1851);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two_inner(TEST_INPUT).unwrap(), 8);
        assert_eq!(part_two().unwrap(), 574080);
    }
}
