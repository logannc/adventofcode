use std::collections::HashMap;

use crate::utils::*;

fn init_points() -> HashMap<char, usize> {
    HashMap::from_iter([(')', 3), (']', 57), ('}', 1197), ('>', 25137)])
}

fn init_pairs() -> HashMap<char, char> {
    HashMap::from_iter([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')])
}

fn score_corrupted(lines: Vec<String>) -> usize {
    let pairs = init_pairs();
    let mut score: HashMap<char, usize> = HashMap::new();
    for line in lines {
        let mut iter = line.chars();
        let mut state = vec![pairs.get(&iter.next().unwrap()).unwrap()];
        for item in iter {
            if pairs.contains_key(&item) {
                state.push(pairs.get(&item).unwrap());
            } else if item != *state[state.len() - 1] {
                let entry = score.entry(item).or_insert(0);
                *entry += 1;
                break;
            } else {
                state.pop();
            }
        }
    }
    let points = init_points();
    score
        .into_iter()
        .map(|(i, c)| points.get(&i).unwrap() * c)
        .sum()
}

pub fn part_one() {
    let ip = problem_input_path(10, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    println!("{}", score_corrupted(lines));
}

fn score_incomplete(lines: Vec<String>) -> usize {
    let pairs = init_pairs();
    let mut score: HashMap<char, usize> = HashMap::new();
    let mut line_states = Vec::new();
    for line in lines {
        let mut iter = line.chars();
        let mut state = vec![pairs.get(&iter.next().unwrap()).unwrap()];
        let mut corrupted = false;
        for item in iter {
            if pairs.contains_key(&item) {
                state.push(pairs.get(&item).unwrap());
            } else if item != *state[state.len() - 1] {
                let entry = score.entry(item).or_insert(0);
                *entry += 1;
                corrupted = true;
                break;
            } else {
                state.pop();
            }
        }
        if !corrupted {
            line_states.push(state);
        }
    }
    let points: HashMap<char, usize> = HashMap::from_iter([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let mut scores: Vec<usize> = line_states
        .into_iter()
        .map(|s| {
            s.into_iter()
                .rev()
                .fold(0, |accum, item| accum * 5 + points.get(item).unwrap())
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

pub fn part_two() {
    let ip = problem_input_path(10, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    println!("{}", score_incomplete(lines));
}

#[test]
fn example() {
    let example = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;
    let lines: Vec<String> = example.split("\n").map(|s| s.into()).collect();
    assert_eq!(score_corrupted(lines.clone()), 26397);
    assert_eq!(score_incomplete(lines), 288957);
}
