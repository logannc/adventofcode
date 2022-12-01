use std::collections::{HashMap, VecDeque};

use crate::utils::*;

fn parse_input(lines: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut edges = HashMap::new();
    for line in lines {
        let mut vertices = line.split("-");
        let first = vertices.next().unwrap();
        let second = vertices.next().unwrap();
        edges
            .entry(first.to_string())
            .or_insert(Vec::new())
            .push(second.to_string());
        edges
            .entry(second.to_string())
            .or_insert(Vec::new())
            .push(first.to_string());
    }
    edges
}

fn get_all_paths_1(edges: HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut finished_paths: Vec<Vec<String>> = Vec::new();
    let mut frontier = VecDeque::new();
    frontier.push_back(vec!["start"]);
    while !frontier.is_empty() {
        let path = frontier.pop_front().unwrap();
        // println!("getting {} from {:?}", path[path.len() - 1], edges);
        let options = edges.get(path[path.len() - 1]).unwrap();
        for edge in options {
            if "end" == edge {
                finished_paths.push(path.iter().map(|s| s.to_string()).collect());
            } else if "start" == edge {
                continue;
            } else if edge.chars().all(|c| c.is_uppercase()) || !path.contains(&edge.as_str()) {
                let mut new_path = path.clone();
                new_path.push(edge);
                frontier.push_back(new_path);
            }
        }
    }
    finished_paths
}

fn get_all_paths_2(edges: HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut finished_paths: Vec<Vec<String>> = Vec::new();
    let mut frontier = VecDeque::new();
    frontier.push_back((vec!["start"], None));
    while !frontier.is_empty() {
        let (path, double) = frontier.pop_front().unwrap();
        // println!("getting {} from {:?}", path[path.len() - 1], edges);
        let options = edges.get(path[path.len() - 1]).unwrap();
        for edge in options {
            if "end" == edge {
                finished_paths.push(path.iter().map(|s| s.to_string()).collect());
            } else if "start" == edge {
                continue;
            } else if edge.chars().all(|c| c.is_uppercase()) {
                let mut new_path = path.clone();
                new_path.push(edge);
                frontier.push_back((new_path, double));
            } else if !path.contains(&edge.as_str()) {
                let mut new_path = path.clone();
                new_path.push(edge);
                frontier.push_back((new_path, double));
            } else if double.is_none() {
                let mut new_path = path.clone();
                new_path.push(edge);
                frontier.push_back((new_path, Some(edge)));
            } else {
                // println!("from {:?} to {}? no", path, edge);
            }
        }
    }
    finished_paths
}

pub fn part_one() {
    let ip = problem_input_path(12, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    let edges = parse_input(lines);
    let finished_paths = get_all_paths_1(edges);
    println!("{}", finished_paths.len());
}

pub fn part_two() {
    let ip = problem_input_path(12, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    let edges = parse_input(lines);
    let finished_paths = get_all_paths_2(edges);
    println!("{}", finished_paths.len());
}

#[test]
fn example() {
    let example = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;
    let edges = parse_input(example.split("\n").map(|s| s.to_string()).collect());
    let finished_paths = get_all_paths_1(edges.clone());
    assert_eq!(finished_paths.len(), 10);
    let finished_paths = get_all_paths_2(edges.clone());
    assert_eq!(finished_paths.len(), 36);
    let example = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;
    let edges = parse_input(example.split("\n").map(|s| s.to_string()).collect());
    let finished_paths = get_all_paths_1(edges.clone());
    assert_eq!(finished_paths.len(), 19);
    let finished_paths = get_all_paths_2(edges.clone());
    assert_eq!(finished_paths.len(), 103);
    let example = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;
    let edges = parse_input(example.split("\n").map(|s| s.to_string()).collect());
    let finished_paths = get_all_paths_1(edges.clone());
    assert_eq!(finished_paths.len(), 226);
    let finished_paths = get_all_paths_2(edges.clone());
    assert_eq!(finished_paths.len(), 3509);
}
