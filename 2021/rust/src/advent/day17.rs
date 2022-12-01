use std::cmp::max;

use regex::Regex;

use crate::utils::*;

#[derive(Debug)]
struct TargetArea {
    x: (i32, i32),
    y: (i32, i32),
}

#[derive(Debug)]
struct Probe {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Probe {
    fn new(x: i32, y: i32) -> Probe {
        Probe {
            position: Default::default(),
            velocity: (x, y),
        }
    }
}

fn parse_input(line: &str) -> TargetArea {
    let r = Regex::new(r"x=(.+)\.\.(.+), y=(.+)\.\.(.+)").unwrap();
    let captures = r.captures(&line).unwrap();
    let x = (captures[1].parse().unwrap(), captures[2].parse().unwrap());
    let y = (captures[3].parse().unwrap(), captures[4].parse().unwrap());
    TargetArea { x, y }
}

fn step(probe: Probe) -> Probe {
    Probe {
        position: (
            probe.position.0 + probe.velocity.0,
            probe.position.1 + probe.velocity.1,
        ),
        velocity: (max(0, probe.velocity.0 - 1), probe.velocity.1 - 1),
    }
}

fn contains(probe: &Probe, target_area: &TargetArea) -> bool {
    target_area.x.0 <= probe.position.0
        && probe.position.0 <= target_area.x.1
        && target_area.y.0 <= probe.position.1
        && probe.position.1 <= target_area.y.1
}

fn high_shot(target_area: &TargetArea) -> (i32, i32) {
    for x in (0..target_area.x.1).rev() {
        let distance = sum_to_n(x as u32) as i32;
        if target_area.x.0 <= distance && distance <= target_area.x.1 {
            return (x, -(1 + target_area.y.0));
        }
    }
    panic!("oops")
}

fn minimum_x(target_area: &TargetArea) -> i32 {
    for x in 0..target_area.x.1 {
        let distance = sum_to_n(x as u32) as i32;
        if target_area.x.0 <= distance && distance <= target_area.x.1 {
            return x;
        }
    }
    panic!("oops")
}

fn maximum_x(target_area: &TargetArea) -> i32 {
    target_area.x.1
}

fn minimum_y(target_area: &TargetArea) -> i32 {
    target_area.y.0
}

fn maximum_y(target_area: &TargetArea) -> i32 {
    -(1 + target_area.y.0)
}

fn count_solutions(target_area: &TargetArea) -> u32 {
    let mut count = 0;
    for x in minimum_x(target_area)..=maximum_x(target_area) {
        for y in minimum_y(target_area)..=maximum_y(target_area){
            // println!("testing {:?}", (x, y));
            let mut probe = Probe::new(x, y);
            while target_area.y.0 < probe.position.1 {
                probe = step(probe);
                if contains(&probe, target_area) {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

pub fn part_one() {
    let ip = problem_input_path(17, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    assert!(lines.len() == 1);
    let target_area = parse_input(&lines[0]);
    let (x, y) = high_shot(&target_area);
    let mut max_y = 0;
    let mut probe = Probe::new(x, y);
    while !contains(&probe, &target_area) {
        probe = step(probe);
        if probe.position.1 > max_y {
            max_y = probe.position.1;
        }
        if probe.position.1 < target_area.y.0 {
            panic!("oops")
        }
    }
    println!("{}", max_y);
}

pub fn part_two() {
    let ip = problem_input_path(17, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    assert!(lines.len() == 1);
    let target_area = parse_input(&lines[0]);
    let solution_count = count_solutions(&target_area);
    println!("{}", solution_count);
}

fn sum_to_n(n: u32) -> u32 {
    n / 2 * (n + 1)
}

#[test]
fn example() {
    let example = "target area: x=20..30, y=-10..-5";
    let target_area = parse_input(example);
    let mut probe = Probe::new(7, 2);
    for _ in 0..6 {
        probe = step(probe);
        assert!(!contains(&probe, &target_area));
    }
    probe = step(probe);
    assert!(contains(&probe, &target_area));
    let velocity = high_shot(&target_area);
    assert_eq!(velocity, (7, 9));
    // let mut probe = Probe::new(6, 9);
    // for _step in 0..100 {
    //     probe = step(probe);
    //     println!("{:?}", probe);
    //     if contains(&probe, &target_area) {
    //         println!("{:?} in {:?}", probe, target_area);
    //         break;
    //     }
    // }
    // let mut valid_x = Vec::new();
    // for x in 0..target_area.x.1 {
    //     let distance = sum_to_n(x as u32) as i32;
    //     if target_area.x.0 <= distance && distance <= target_area.x.1 {
    //         valid_x.push(x);
    //     }
    // }
    // assert_eq!(valid_x, vec![6, 7]);
}
