use crate::utils::*;

fn gamma_digits_from_count(counts: &Vec<u32>, threshold: u32) -> Vec<u32> {
    counts
        .iter()
        .map(|&count| if count >= threshold { 1 } else { 0 })
        .collect()
}

fn epsilon_digits_from_count(counts: &Vec<u32>, threshold: u32) -> Vec<u32> {
    counts
        .iter()
        .map(|&count| if count >= threshold { 0 } else { 1 })
        .collect()
}

fn number_from_digits(digits: &Vec<u32>) -> u32 {
    digits
        .iter()
        .rev()
        .enumerate()
        .fold(0, |accum, (idx, num)| accum + (num << idx))
}

fn occurence_counter(data: &Vec<String>) -> Vec<u32> {
    data.into_iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(2).unwrap())
                .collect::<Vec<u32>>()
        })
        .reduce(|v1, v2| v1.into_iter().zip(v2).map(|(a, b)| a + b).collect())
        .unwrap()
}

pub fn part_one() {
    let ip = problem_input_path(3, Some(1));
    let data: Vec<String> = read_file_split_on(&ip, "\n").expect("failed to parse");
    let threshold = (data.len() / 2) as u32;
    let counts = occurence_counter(&data);
    let gamma_digits = gamma_digits_from_count(&counts, threshold);
    let epsilon_digits = epsilon_digits_from_count(&counts, threshold);
    let gamma = number_from_digits(&gamma_digits);
    let epsilon = number_from_digits(&epsilon_digits);
    println!(
        "gamma = {}, epsilon = {}, power = {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

fn find_rating(mut candidates: Vec<String>, digitizer: impl Fn(&Vec<u32>, u32) -> Vec<u32>) -> u32 {
    let mut idx = 0;
    while candidates.len() > 1 {
        // when odd, round up
        // e.g., when there are 7 items, 3.5 => 4 so that 4 items are needed to be the most common
        let threshold = (candidates.len() as f32 / 2.).ceil() as u32;
        let counts = occurence_counter(&candidates);
        let gamma_digits = digitizer(&counts, threshold);
        let digit_char = gamma_digits
            .get(idx)
            .unwrap()
            .to_string()
            .chars()
            .next()
            .unwrap();
        candidates = candidates
            .into_iter()
            .filter(|s| s.chars().skip(idx).next().unwrap() == digit_char)
            .collect();
        idx += 1;
    }
    u32::from_str_radix(candidates.get(0).unwrap(), 2).unwrap()
}

pub fn part_two() {
    let ip = problem_input_path(3, Some(1));
    let oxygen_generator_candidates: Vec<String> =
        read_file_split_on(&ip, "\n").expect("failed to parse");
    let co2_scrubber_candidates = oxygen_generator_candidates.clone();
    let oxygen_generator_rating = find_rating(oxygen_generator_candidates, gamma_digits_from_count);
    let co2_scrubber_rating = find_rating(co2_scrubber_candidates, epsilon_digits_from_count);
    println!(
        "oxygen generator rating = {}, co2 scrubber rating = {}, life support rating = {}",
        oxygen_generator_rating,
        co2_scrubber_rating,
        oxygen_generator_rating * co2_scrubber_rating
    );
}
