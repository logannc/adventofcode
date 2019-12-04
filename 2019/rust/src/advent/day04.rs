use crate::utils::errors::Error;
use crate::utils::files::{problem_input_path, read_file_split_on};

fn is_valid_part_one(candidate: u32) -> bool {
    let digits: Vec<u32> = candidate
        .to_string()
        .chars()
        .flat_map(|c| c.to_digit(10))
        .collect();
    // has any two same digits adjacent
    let (has_adjacent, _) = digits
        .iter()
        .skip(1)
        .fold((false, digits[0]), |(found, prev), curr| {
            (found || prev == *curr, *curr)
        });
    // left-to-right monotonic non-decreasing
    let (monotonic, _) = digits
        .iter()
        .skip(1)
        .fold((true, digits[0]), |(nondecreasing, prev), curr| {
            (nondecreasing && prev <= *curr, *curr)
        });
    has_adjacent && monotonic
}

pub fn part_one() -> Result<u32, Error> {
    let input_path = problem_input_path(4, None);
    let range = read_file_split_on(&input_path, "-")?;
    let (min, max) = (range[0], range[1]);
    let mut count = 0;
    for candidate in min..max {
        if is_valid_part_one(candidate) {
            count += 1;
        }
    }
    Ok(count)
}

fn is_valid_part_two(candidate: u32) -> bool {
    let digits: Vec<u32> = candidate
        .to_string()
        .chars()
        .flat_map(|c| c.to_digit(10))
        .collect();
    // has any two same digits adjacent
    let (found, last_run_length, _) =
        digits
            .iter()
            .skip(1)
            .fold((false, 1, digits[0]), |(found, run_length, prev), curr| {
                let continues_run = prev == *curr;
                (
                    found || (run_length == 2 && !continues_run),
                    if continues_run { run_length + 1 } else { 1 },
                    *curr,
                )
            });
    let has_adjacent = found || last_run_length == 2;
    // left-to-right monotonic non-decreasing
    let (monotonic, _) = digits
        .iter()
        .skip(1)
        .fold((true, digits[0]), |(nondecreasing, prev), curr| {
            (nondecreasing && prev <= *curr, *curr)
        });
    has_adjacent && monotonic
}

pub fn part_two() -> Result<u32, Error> {
    let input_path = problem_input_path(4, None);
    let range = read_file_split_on(&input_path, "-")?;
    let (min, max) = (range[0], range[1]);
    let mut count = 0;
    for candidate in min..max {
        if is_valid_part_two(candidate) {
            count += 1;
        }
    }
    Ok(count)
}
