use crate::utils::*;
use eyre::Result;
use std::{collections::HashSet, fs};

pub fn part_one() -> Result<()> {
    let input_path = problem_input_path(6, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = solve::<4>(&content);
    println!("{}", result);
    Ok(())
}

pub fn part_two() -> Result<()> {
    let input_path = problem_input_path(6, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = solve::<14>(&content);
    println!("{}", result);
    Ok(())
}

fn all_different<const N: usize>(items: &[u8]) -> bool {
    let mut set: HashSet<u8> = HashSet::with_capacity(N);
    set.extend(items);
    set.len() == N
}

fn solve<const N: usize>(input: &str) -> u32 {
    input
        .as_bytes()
        .windows(N)
        .enumerate()
        .filter(|(_, w)| all_different::<N>(w))
        .next()
        .unwrap()
        .0 as u32
        + N as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUTS_PART_ONE: [(&str, u32); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    #[test]
    fn part_one_works() {
        for (input, expected) in TEST_INPUTS_PART_ONE {
            assert_eq!(solve::<4>(input), expected);
        }
    }

    const TEST_INPUTS_PART_TWO: [(&str, u32); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
    ];

    #[test]
    fn part_two_works() {
        for (input, expected) in TEST_INPUTS_PART_TWO {
            assert_eq!(solve::<14>(input), expected);
        }
    }
}
