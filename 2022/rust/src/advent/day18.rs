use crate::utils::*;
use eyre::Result;
use std::{collections::BTreeSet, fs};

pub fn part_one() -> Result<usize> {
    let input_path = problem_input_path(18, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_one_inner(&content)?;
    println!("{result}");
    Ok(result)
}

pub fn part_two() -> Result<usize> {
    let input_path = problem_input_path(18, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_two_inner(&content)?;
    println!("{}", result);
    Ok(result)
}

fn part_one_inner(input: &str) -> Result<usize> {
    let points = parse_input(input);
    Ok(points
        .iter()
        .map(|point| neighbors(point))
        .flatten()
        .filter(|p| !points.contains(p))
        .count())
}

fn part_two_inner(input: &str) -> Result<usize> {
    let points = parse_input(input);
    let exterior = find_exterior(&points);
    Ok(points
        .iter()
        .map(|point| neighbors(point))
        .flatten()
        .filter(|p| !points.contains(p) && exterior.contains(p))
        .count())
}

type Point = (isize, isize, isize);

fn find_exterior(points: &BTreeSet<Point>) -> BTreeSet<Point> {
    let mut exterior = BTreeSet::new();
    let mut frontier = BTreeSet::new();
    let upper_bound = (
        points.iter().map(|p| p.0).max().unwrap() + 1,
        points.iter().map(|p| p.1).max().unwrap() + 1,
        points.iter().map(|p| p.2).max().unwrap() + 1,
    );
    let lower_bound = (
        points.iter().map(|p| p.0).min().unwrap() - 1,
        points.iter().map(|p| p.1).min().unwrap() - 1,
        points.iter().map(|p| p.2).min().unwrap() - 1,
    );
    frontier.insert(lower_bound);
    while let Some(candidate) = frontier.pop_first() {
        if exterior.insert(candidate) {
            for neighbor in neighbors(&candidate) {
                if !points.contains(&neighbor) && bounded(neighbor, lower_bound, upper_bound) {
                    frontier.insert(neighbor);
                }
            }
        }
    }
    exterior
}

fn bounded(candidate: Point, lower_bound: Point, upper_bound: Point) -> bool {
    lower_bound.0 <= candidate.0
        && candidate.0 <= upper_bound.0
        && lower_bound.1 <= candidate.1
        && candidate.1 <= upper_bound.1
        && lower_bound.2 <= candidate.2
        && candidate.2 <= upper_bound.2
}

fn neighbors(p: &Point) -> impl Iterator<Item = Point> {
    let cardinals = vec![
        (-1, 0, 0),
        (1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    let points: Vec<Point> = cardinals
        .into_iter()
        .map(move |(xdelta, ydelta, zdelta)| (p.0 + xdelta, p.1 + ydelta, p.2 + zdelta))
        .collect();
    points.into_iter()
}

fn parse_input(input: &str) -> BTreeSet<Point> {
    input
        .trim()
        .lines()
        .map(|line| {
            let coords: Result<Vec<isize>, _> = line.split(',').map(str::parse::<isize>).collect();
            let [x, y, z]: [isize; 3] = coords.unwrap().try_into().unwrap();
            (x, y, z)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = "1,1,1\n2,1,1";

    const TEST_INPUT: &str = r#"
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
"#;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one_inner(SIMPLE_INPUT).unwrap(), 10);
        assert_eq!(part_one_inner(TEST_INPUT).unwrap(), 64);
        assert_eq!(part_one().unwrap(), 4390);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two_inner(TEST_INPUT).unwrap(), 58);
        assert_eq!(part_two().unwrap(), 2534);
    }
}
