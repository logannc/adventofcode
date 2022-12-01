use std::collections::{BTreeSet, HashMap};
use itertools::Itertools;

use crate::utils::*;

// 1, 4, 7, 8 in 7-segment displays use a unique number of digits
const UNIQUE_COUNTS: [usize; 4] = [2, 4, 3, 7];

pub fn part_one() {
    let ip = problem_input_path(8, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    let unambiguous_count: usize = lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .skip_while(|s| *s != "|")
                .skip(1) // skip the |
                .map(str::len)
                .filter(|l| UNIQUE_COUNTS.contains(l))
                .count()
        })
        .sum();
    println!("unambiguous characters: {}", unambiguous_count);
}

fn solve(input: &[&str]) -> HashMap<BTreeSet<char>, char> {
    let mut sets: Vec<BTreeSet<char>> = input
        .iter()
        .map(|s| s.chars().collect::<BTreeSet<char>>())
        .collect();
    let one = sets.remove(sets.iter().position(|h| h.len() == 2).unwrap());
    let seven = sets.remove(sets.iter().position(|h| h.len() == 3).unwrap());
    let four = sets.remove(sets.iter().position(|h| h.len() == 4).unwrap());
    let eight = sets.remove(sets.iter().position(|h| h.len() == 7).unwrap());
    let nine = sets.remove(
        sets.iter()
            .position(|h| h.len() == 6 && h & &four == four)
            .unwrap(),
    );
    let zero = sets.remove(
        sets.iter()
            .position(|h| h.len() == 6 && (h - &one).len() == 4)
            .unwrap(),
    );
    // nine and zero have been removed, leaving only six
    let six = sets.remove(sets.iter().position(|h| h.len() == 6).unwrap());
    let five = sets.remove(
        sets.iter()
            .position(|h| h.len() == 5 && (h - &six).is_empty())
            .unwrap(),
    );
    let three = sets.remove(
        sets.iter()
            .position(|h| h.len() == 5 && (h - &one).len() == 3)
            .unwrap(),
    );
    // five and three have been removed, leaving only two
    let two = sets.remove(sets.iter().position(|h| h.len() == 5).unwrap());
    HashMap::from_iter(
        [zero, one, two, three, four, five, six, seven, eight, nine]
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i.to_string().chars().next().unwrap())),
    )
}

pub fn part_two() {
    let ip = problem_input_path(8, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    let mut sum: usize = 0;
    for line in lines {
        let items: Vec<&str> = line.split_whitespace().collect();
        let mut split_by_delimiter = items.split(|s| *s == "|");
        let input = split_by_delimiter.next().unwrap();
        let output = split_by_delimiter.next().unwrap();
        let permutation = solve(input);
        let number = output
            .into_iter()
            .map(|s| {
                permutation
                    .get(&s.chars().collect::<BTreeSet<char>>())
                    .unwrap()
            })
            .join("");
        sum += number.parse::<usize>().unwrap();
    }
    println!("sum {}", sum);
}

#[test]
fn example() {
    let example = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
    cdfeb fcadb cdfeb cdbaf";
    let items: Vec<&str> = example.split_whitespace().collect();
    let mut split_by_delimiter = items.split(|s| *s == "|");
    let input = split_by_delimiter.next().unwrap();
    let output = split_by_delimiter.next().unwrap();
    let permutation = solve(input);
    let number = output
        .into_iter()
        .map(|s| {
            permutation
                .get(&s.chars().collect::<BTreeSet<char>>())
                .unwrap()
        })
        .join("");
    assert_eq!(number, "5353");
}
