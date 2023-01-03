use crate::utils::*;
use eyre::{Report, Result};
use std::{fmt::Debug, fs, str::FromStr};

pub fn part_one() -> Result<usize> {
    let input_path = problem_input_path(13, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_one_inner(&content)?;
    println!("{result}");
    Ok(result)
}

pub fn part_two() -> Result<usize> {
    let input_path = problem_input_path(13, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_two_inner(&content)?;
    println!("{result}");
    Ok(result)
}

fn part_one_inner(input: &str) -> Result<usize> {
    let pairs = input.trim().split("\n\n");
    let sum = pairs
        .into_iter()
        .map(|pair| {
            let (left, right) = pair.split_once('\n').unwrap();
            let left: SpecialLists = str::parse(left).unwrap();
            let right: SpecialLists = str::parse(right).unwrap();
            left <= right
        })
        .enumerate()
        .filter(|(_, in_order)| *in_order)
        .map(|(idx, _)| idx + 1)
        .sum();
    Ok(sum)
}

fn part_two_inner(input: &str) -> Result<usize> {
    let left_decoder = SpecialLists::List(vec![SpecialLists::List(vec![SpecialLists::Number(2)])]);
    let right_decoder = SpecialLists::List(vec![SpecialLists::List(vec![SpecialLists::Number(6)])]);
    let mut lists: Vec<SpecialLists> = input
        .trim()
        .split("\n\n")
        .into_iter()
        .flat_map(|pair| {
            pair.lines()
                .map(|line| str::parse::<SpecialLists>(line).unwrap())
        })
        .collect();
    lists.push(left_decoder.clone());
    lists.push(right_decoder.clone());
    lists.sort();
    let left_position = lists.iter().position(|i| *i == left_decoder).unwrap() + 1;
    let right_position = lists.iter().position(|i| *i == right_decoder).unwrap() + 1;
    Ok(left_position * right_position)
}

#[derive(PartialEq, Eq, Clone)]
enum SpecialLists {
    List(Vec<SpecialLists>),
    Number(usize),
}

impl Debug for SpecialLists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecialLists::Number(num) => f.write_str(&format!("{num}")),
            SpecialLists::List(list) => f.write_str(&format!("{:?}", &list)),
        }
    }
}

fn parse_item(mut input: &str) -> Result<(&str, SpecialLists)> {
    let item: String = input.chars().take_while(|c| c.is_ascii_digit()).collect();
    (_, input) = input.split_at(item.len());
    Ok((input, SpecialLists::Number(str::parse(&item)?)))
}

fn parse_list(mut input: &str) -> Result<(&str, SpecialLists)> {
    input = input.strip_prefix('[').unwrap();
    let mut items = Vec::new();
    loop {
        input = input.trim_start_matches(',');
        let item;
        if input.starts_with('[') {
            (input, item) = parse_list(input)?;
        } else if input.starts_with(']') {
            return Ok((input.strip_prefix(']').unwrap(), SpecialLists::List(items)));
        } else {
            (input, item) = parse_item(input)?;
        }
        items.push(item);
    }
}

impl FromStr for SpecialLists {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let copy = s.to_owned();
        let (_, result) = parse_list(&copy)?;
        Ok(result)
    }
}

impl PartialOrd for SpecialLists {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for SpecialLists {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (SpecialLists::Number(left), SpecialLists::Number(right)) => Ord::cmp(left, right),
            (SpecialLists::List(left), SpecialLists::List(right)) => {
                for (li, ri) in left.iter().zip(right.iter()) {
                    match Ord::cmp(li, ri) {
                        std::cmp::Ordering::Equal => {}
                        o => return o,
                    }
                }
                Ord::cmp(&left.len(), &right.len())
            }
            (SpecialLists::Number(left), SpecialLists::List(right)) => Ord::cmp(
                &SpecialLists::List(vec![SpecialLists::Number(*left)]),
                &SpecialLists::List(right.clone()),
            ),
            (SpecialLists::List(left), SpecialLists::Number(right)) => Ord::cmp(
                &SpecialLists::List(left.clone()),
                &SpecialLists::List(vec![SpecialLists::Number(*right)]),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one_inner(TEST_INPUT).unwrap(), 13);
        assert_eq!(part_one().unwrap(), 5806);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two_inner(TEST_INPUT).unwrap(), 140);
        assert_eq!(part_two().unwrap(), 23600);
    }
}
