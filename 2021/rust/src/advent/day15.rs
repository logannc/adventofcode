use std::collections::{BinaryHeap, HashSet};

use crate::utils::*;

const CARDINAL_DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Debug, Clone)]
struct Path {
    path: Vec<(i32, i32)>,
    risk: u32,
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.risk == other.risk
    }
}

impl Eq for Path {}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.risk.partial_cmp(&other.risk).map(|o| o.reverse())
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.risk.cmp(&other.risk).reverse()
    }
}

fn parse_input(lines: Vec<String>) -> Vec<Vec<u32>> {
    let mut map = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        map.push(row);
    }
    map
}

fn find_best_path(map: &Vec<Vec<u32>>) -> Path {
    let start = (0 as i32, 0 as i32);
    let (end_r, end_c) = ((map.len() - 1) as i32, (map[0].len() - 1) as i32);
    let mut visited = HashSet::new();
    let mut frontier = BinaryHeap::new();
    // The starting position is not 'entered', so its risk does not count
    frontier.push(Path {
        path: vec![start],
        risk: 0,
    });
    visited.insert(start);
    while let Some(path) = frontier.pop() {
        // println!("visiting {:?}", path);
        let tip = path.path[path.path.len() - 1];
        let (r, c) = tip;
        for (dr, dc) in CARDINAL_DIRECTIONS {
            let (nr, nc) = (r + dr, c + dc);
            if !visited.contains(&(nr, nc)) && 0 <= nr && nr <= end_r && 0 <= nc && nc <= end_c {
                let mut neighbor_path = path.path.clone();
                neighbor_path.push((nr, nc));
                let neighbor_risk = map[nr as usize][nc as usize];
                let neighbor = Path {
                    path: neighbor_path,
                    risk: path.risk + neighbor_risk,
                };
                if nr == end_r && nc == end_c {
                    return neighbor;
                }
                frontier.push(neighbor);
                visited.insert((nr, nc));
            }
        }
    }
    panic!("we should have returned the end path by now")
}

fn find_best_path_tiled(map: &Vec<Vec<u32>>) -> Path {
    let start = (0 as i32, 0 as i32);
    let (rows, columns) = (map.len() as i32, map[0].len() as i32);
    let (end_r, end_c) = (rows * 5 - 1 as i32, columns * 5 - 1 as i32);
    let mut visited = HashSet::new();
    let mut frontier = BinaryHeap::new();
    // The starting position is not 'entered', so its risk does not count
    frontier.push(Path {
        path: vec![start],
        risk: 0,
    });
    visited.insert(start);
    while let Some(path) = frontier.pop() {
        // println!("visiting {:?}", path);
        let tip = path.path[path.path.len() - 1];
        let (r, c) = tip;
        for (dr, dc) in CARDINAL_DIRECTIONS {
            let (nr, nc) = (r + dr, c + dc);
            if !visited.contains(&(nr, nc)) && 0 <= nr && nr <= end_r && 0 <= nc && nc <= end_c {
                let mut neighbor_path = path.path.clone();
                neighbor_path.push((nr, nc));
                let (row_tile, row_offset) = (nr / rows, nr % rows);
                let (col_tile, col_offset) = (nc / columns, nc % columns);
                let neighbor_risk =
                    map[row_offset as usize][col_offset as usize] as i32 + row_tile + col_tile;
                let risk = if neighbor_risk > 9 {
                    neighbor_risk % 10 + 1
                } else {
                    neighbor_risk
                };
                let neighbor = Path {
                    path: neighbor_path,
                    risk: path.risk + risk as u32,
                };
                if nr == end_r && nc == end_c {
                    return neighbor;
                }
                frontier.push(neighbor);
                visited.insert((nr, nc));
            }
        }
    }
    panic!("we should have returned the end path by now")
}

pub fn part_one() {
    let ip = problem_input_path(15, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    let map = parse_input(lines);
    let best_path = find_best_path(&map);
    print_path(&map, &best_path);
    println!("{}", best_path.risk);
}

pub fn part_two() {
    let ip = problem_input_path(15, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    let map = parse_input(lines);
    let best_path = find_best_path_tiled(&map);
    print_path(&map, &best_path);
    println!("{}", best_path.risk);
}

fn print_path(map: &Vec<Vec<u32>>, path: &Path) {
    let rows = map.len();
    let columns = map[0].len();
    let mut text = String::new();
    for row in 0..(rows * 5) {
        for col in 0..(columns * 5) {
            if path.path.contains(&(row as i32, col as i32)) {
                let (row_tile, row_offset) = (row / rows, row % rows);
                let (col_tile, col_offset) = (col / columns, col % columns);
                let neighbor_risk =
                    map[row_offset as usize][col_offset as usize] as usize + row_tile + col_tile;
                let risk = if neighbor_risk > 9 {
                    neighbor_risk % 10 + 1
                } else {
                    neighbor_risk
                };
                text.extend(std::iter::once(risk.to_string()));
            } else {
                text.extend(std::iter::once('.'));
            }
        }
        text.extend(std::iter::once('\n'))
    }
    println!("{}", text);
}

#[test]
fn example() {
    let example = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;
    let lines = example.split("\n").map(|s| s.to_string()).collect();
    let map = parse_input(lines);
    let best_path = find_best_path(&map);
    assert_eq!(best_path.risk, 40);
    let best_path = find_best_path_tiled(&map);
    print_path(&map, &best_path);
    assert_eq!(best_path.risk, 315);
}
