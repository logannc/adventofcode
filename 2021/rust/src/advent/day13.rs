use std::collections::HashSet;

use regex::Regex;

use crate::utils::*;

fn parse_input(lines: Vec<String>) -> (HashSet<(usize, usize)>, Vec<Fold>) {
    let mut dot_coords = HashSet::new();
    for dot_line in lines.iter().take_while(|s| !s.is_empty()) {
        let mut coords = dot_line.split(",");
        let x: usize = coords.next().unwrap().parse().unwrap();
        let y: usize = coords.next().unwrap().parse().unwrap();
        dot_coords.insert((x, y));
    }
    let mut folds = Vec::new();
    let fold_re = Regex::new(r"([xy])=(\d+)").unwrap();
    for fold_line in lines.iter().skip(dot_coords.len() + 1) {
        let fold_captures = fold_re.captures(fold_line).unwrap();
        let axis = &fold_captures[1];
        let coord = &fold_captures[2];
        match axis {
            "x" => folds.push(Fold::Vertical(coord.parse().unwrap())),
            "y" => folds.push(Fold::Horiztonal(coord.parse().unwrap())),
            _ => panic!("invalid axis"),
        }
    }
    (dot_coords, folds)
}

#[derive(Debug)]
enum Fold {
    Vertical(usize),
    Horiztonal(usize),
}

fn perform_fold(coords: HashSet<(usize, usize)>, fold: Fold) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();
    for (x, y) in coords {
        match fold {
            Fold::Vertical(f) => {
                if x != f {
                    let new_x = f - (f as i32 - x as i32).abs() as usize;
                    result.insert((new_x, y));
                }
            }
            Fold::Horiztonal(f) => {
                if y != f {
                    let new_y = f - (f as i32 - y as i32).abs() as usize;
                    result.insert((x, new_y));
                }
            }
        }
    }
    result
}

pub fn part_one() {
    let ip = problem_input_path(13, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    let (mut coords, folds) = parse_input(lines);
    let mut folds = folds.into_iter();
    coords = perform_fold(coords, folds.next().unwrap());
    println!("{}", coords.len());
}

fn print_coords(coords: HashSet<(usize, usize)>) {
    let mut text = String::new();
    let x_max = *coords.iter().map(|(x, _)| x).max().unwrap();
    let y_max = *coords.iter().map(|(_, y)| y).max().unwrap();
    for y in 0..=y_max {
        for x in 0..=x_max {
            if coords.contains(&(x, y)) {
                text.extend(std::iter::once('#'));
            } else {
                text.extend(std::iter::once('.'));
            }
        }
        text.extend(std::iter::once('\n'));
    }
    println!("{}", text);
}

pub fn part_two() {
    let ip = problem_input_path(13, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    let (mut coords, folds) = parse_input(lines);
    for fold in folds {
        coords = perform_fold(coords, fold);
    }
    print_coords(coords);
}

#[test]
fn example() {
    let example = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;
    let lines = example.split("\n").map(|s| s.to_string()).collect();
    let (mut coords, folds) = parse_input(lines);
    let mut folds = folds.into_iter();
    coords = perform_fold(coords, folds.next().unwrap());
    assert_eq!(coords.len(), 17);
}
