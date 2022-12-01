use crate::utils::*;

fn nth_sum(n: i32) -> i32 {
    (n * (n+1)) / 2
}

fn solve_median_linear(mut positions: Vec<i32>) -> i32 {
    positions.sort_unstable();
    let length = positions.len();
    let median = if length & 1 == 1 {
        positions[length/2]
    } else {
        let middle = length/2;
        (positions[middle-1] + positions[middle])/2
    };
    positions.into_iter().map(|p| (p-median).abs()).sum()
}

fn brute_force(mut positions: Vec<i32>) -> i32 {
    positions.sort_unstable();
    let mut _best = 0;
    let mut cost = 1 << 30; // big number, not relevant
    for candidate_position in positions[0]..positions[positions.len()-1] {
        let candidate_cost: i32 = positions.iter().map(|p| nth_sum((*p-candidate_position).abs())).sum();
        if candidate_cost < cost {
            _best = candidate_position;
            cost = candidate_cost;
        }
    }
    cost
}

pub fn part_one() {
    let ip = problem_input_path(7, Some(1));
    let positions: Vec<i32> = read_file_split_on(&ip, ",").unwrap();
    println!("fuel needed: {}", solve_median_linear(positions));
}

pub fn part_two() {
    let ip = problem_input_path(7, Some(1));
    let positions: Vec<i32> = read_file_split_on(&ip, ",").unwrap();
    println!("fuel needed: {}", brute_force(positions));
}

#[test]
fn example() {
    let positions = vec![16,1,2,0,4,2,7,1,2,14];
    assert_eq!(solve_median_linear(positions.clone()), 37);
    assert_eq!(brute_force(positions), 168);
}
