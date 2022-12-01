use crate::utils::*;

// rotate_left would have made this so much easier
fn simulate(populations: &mut [u128; 9]) {
    let previous = populations.clone();
    for (idx, count) in previous.into_iter().enumerate().rev() {
        if idx == 0 {
            populations[6] += count;
            populations[8] = count;
        } else {
            populations[idx - 1] = count;
        }
    }
}

pub fn part_one() {
    let ip = problem_input_path(6, Some(1));
    let fishes: Vec<usize> = read_file_split_on(&ip, ",").unwrap();
    let mut populations: [u128; 9] = [0; 9];
    for fish in fishes.into_iter() {
        populations[fish] += 1;
    }
    for _day in 0..80 {
        simulate(&mut populations);
    }
    println!("number of fish: {}", populations.into_iter().sum::<u128>());
}

pub fn part_two() {
    let ip = problem_input_path(6, Some(1));
    let fishes: Vec<usize> = read_file_split_on(&ip, ",").unwrap();
    let mut populations: [u128; 9] = [0; 9];
    for fish in fishes.into_iter() {
        populations[fish] += 1;
    }
    for _day in 0..256 {
        simulate(&mut populations);
    }
    println!("number of fish: {}", populations.into_iter().sum::<u128>());
}

#[test]
fn example() {
    let fishes = vec![3, 4, 3, 1, 2];
    let mut populations: [u128; 9] = [0; 9];
    for fish in fishes.into_iter() {
        populations[fish] += 1;
    }
    for _day in 0..18 {
        println!("{:?}", populations);
        simulate(&mut populations);
    }
    assert_eq!(populations.iter().sum::<u128>(), 26);
    for _day in 0..(80 - 18) {
        simulate(&mut populations);
    }
    assert_eq!(populations.iter().sum::<u128>(), 5934);
}
