use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::*;

fn parse_input(
    lines: Vec<String>,
) -> (
    char,
    HashMap<(char, char), usize>,
    HashMap<(char, char), char>,
) {
    let template: Vec<char> = lines[0].chars().collect();
    let mut counts = HashMap::new();
    for window in template.windows(2) {
        if let [first, second] = window {
            *counts.entry((*first, *second)).or_insert(0) += 1;
        }
    }
    let mut pairs = HashMap::new();
    for line in lines.into_iter().skip(2) {
        let mut split = line.split(" -> ");
        let mut input = split.next().unwrap().chars();
        pairs.insert(
            (input.next().unwrap(), input.next().unwrap()),
            split.next().unwrap().chars().next().unwrap(),
        );
    }
    (template[0], counts, pairs)
}

fn process(
    counts: HashMap<(char, char), usize>,
    pairs: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    let mut new_counts = HashMap::new();
    for ((first, last), count) in counts {
        let middle = *pairs.get(&(first, last)).unwrap();
        *new_counts.entry((first, middle)).or_insert(0) += count;
        *new_counts.entry((middle, last)).or_insert(0) += count;
    }
    new_counts
}

fn count_letters(counts: HashMap<(char, char), usize>, extra: char) -> HashMap<char, usize> {
    let mut letter_counts: HashMap<char, usize> = HashMap::from_iter([(extra, 1usize)]);
    counts
        .into_iter()
        .fold(&mut letter_counts, |m, ((_, second), count)| {
            *m.entry(second).or_insert(0) += count;
            m
        });
    letter_counts
}

// TODO: improvements
// - [char; 2] is hashable
// - counts.values().max().unwrap() - counts.values().min().unwrap() is O(2N) instead of O(N log N) where log N is almost certainly higher than 2
// - only difference is step count
pub fn part_one() {
    let ip = problem_input_path(14, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    let (extra, mut counts, pairs) = parse_input(lines);
    for _ in 0..10 {
        counts = process(counts, &pairs);
    }
    let letter_counts: Vec<usize> = count_letters(counts, extra)
        .values()
        .copied()
        .sorted()
        .collect();
    let result = letter_counts[letter_counts.len() - 1] - letter_counts[0];
    println!("{}", result);
}

pub fn part_two() {
    let ip = problem_input_path(14, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    let (extra, mut counts, pairs) = parse_input(lines);
    for _ in 0..40 {
        counts = process(counts, &pairs);
    }
    let letter_counts: Vec<usize> = count_letters(counts, extra)
        .values()
        .copied()
        .sorted()
        .collect();
    let result = letter_counts[letter_counts.len() - 1] - letter_counts[0];
    println!("{}", result);
}

#[test]
fn example() {
    let example = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;
    let lines = example.split("\n").map(|s| s.to_string()).collect();
    let (extra, mut counts, pairs) = parse_input(lines);
    for _ in 0..10 {
        counts = process(counts, &pairs);
    }
    let letter_counts = count_letters(counts, extra);
    let expected_counts = HashMap::from_iter([('B', 1749), ('C', 298), ('H', 161), ('N', 865)]);
    assert_eq!(letter_counts, expected_counts);
    let letter_counts: Vec<usize> = letter_counts.values().copied().sorted().collect();
    let result = letter_counts[letter_counts.len() - 1] - letter_counts[0];
    assert_eq!(result, 1588);
}
