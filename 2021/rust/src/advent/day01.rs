use crate::utils::*;

pub fn part_one() {
    let ip = problem_input_path(1, Some(1));
    let data: Vec<i64> = read_file_split_whitespace(&ip).expect("failed to parse");
    let increase_count = data
        .iter()
        .zip(data.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count();
    println!("{}", increase_count);
}

pub fn part_two() {
    // reuses the same input
    let ip = problem_input_path(1, Some(1));
    let data: Vec<i64> = read_file_split_whitespace(&ip).expect("failed to parse");
    let window_sums = data.windows(3).map(|w| w.iter().sum());
    let zip = window_sums.clone().zip(window_sums.skip(1));
    let increase_count = zip.filter(|(a, b): &(i64, i64)| a < b).count();
    println!("{}", increase_count)
}
