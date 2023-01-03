use crate::utils::*;
use eyre::Result;
use std::{collections::HashMap, fs};

pub fn part_one() -> Result<isize> {
    let input_path = problem_input_path(21, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_one_inner(&content)?;
    println!("{result}");
    Ok(result)
}

pub fn part_two() -> Result<isize> {
    let input_path = problem_input_path(21, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_two_inner(&content)?;
    println!("{result}");
    Ok(result)
}

fn part_one_inner(input: &str) -> Result<isize> {
    let c = Compute {
        entries: parse_input(input),
    };
    Ok(c.get("root").unwrap())
}

fn part_two_inner(input: &str) -> Result<isize> {
    let mut c = Compute {
        entries: parse_input(input),
    };
    let (left, right) = match c.entries.get("root").unwrap() {
        Job::Plus(left, right) => (left.clone(), right.clone()),
        Job::Minus(left, right) => (left.clone(), right.clone()),
        Job::Mul(left, right) => (left.clone(), right.clone()),
        Job::Div(left, right) => (left.clone(), right.clone()),
        _ => panic!("literal root"),
    };
    c.entries.remove("humn");
    let (unknown, result) = match (c.get(&left), c.get(&right)) {
        (Some(v), None) => (right, v),
        (None, Some(v)) => (left, v),
        _ => panic!("both depend on humn"),
    };
    Ok(c.inverse(&unknown, result).unwrap())
}

struct Compute {
    entries: HashMap<String, Job>,
}

impl Compute {
    fn get(&self, id: &str) -> Option<isize> {
        let entry = self.entries.get(id);
        match entry? {
            Job::Literal(value) => Some(*value),
            Job::Plus(left, right) => Some(self.get(left)? + self.get(right)?),
            Job::Minus(left, right) => Some(self.get(left)? - self.get(right)?),
            Job::Mul(left, right) => Some(self.get(left)? * self.get(right)?),
            Job::Div(left, right) => Some(self.get(left)? / self.get(right)?),
        }
    }

    fn inverse(&self, from: &str, result: isize) -> Option<isize> {
        let entry = self.entries.get(from);
        match entry {
            None => Some(result),
            Some(Job::Literal(_)) => None,
            Some(Job::Plus(left, right)) => match (self.get(left), self.get(right)) {
                (Some(v), None) => self.inverse(right, result - v),
                (None, Some(v)) => self.inverse(left, result - v),
                _ => panic!("both depend on humn"),
            },
            Some(Job::Minus(left, right)) => match (self.get(left), self.get(right)) {
                (Some(v), None) => self.inverse(right, v - result),
                (None, Some(v)) => self.inverse(left, result + v),
                _ => panic!("both depend on humn"),
            },
            Some(Job::Mul(left, right)) => match (self.get(left), self.get(right)) {
                (Some(v), None) => self.inverse(right, result / v),
                (None, Some(v)) => self.inverse(left, result / v),
                _ => panic!("both depend on humn"),
            },
            Some(Job::Div(left, right)) => match (self.get(left), self.get(right)) {
                (Some(v), None) => self.inverse(right, v / result),
                (None, Some(v)) => self.inverse(left, result * v),
                _ => panic!("both depend on humn"),
            },
        }
    }
}

fn parse_input(input: &str) -> HashMap<String, Job> {
    let mut entries = HashMap::new();
    for (k, v) in input.trim().lines().map(parse_line) {
        entries.insert(k, v);
    }
    entries
}

enum Job {
    Literal(isize),
    Plus(String, String),
    Minus(String, String),
    Mul(String, String),
    Div(String, String),
}

fn parse_line(s: &str) -> (String, Job) {
    let (name, rest) = s.split_once(':').unwrap();
    let job = if rest.contains('+') {
        let (left, right) = rest.split_once('+').unwrap();
        Job::Plus(left.trim().into(), right.trim().into())
    } else if rest.contains('-') {
        let (left, right) = rest.split_once('-').unwrap();
        Job::Minus(left.trim().into(), right.trim().into())
    } else if rest.contains('*') {
        let (left, right) = rest.split_once('*').unwrap();
        Job::Mul(left.trim().into(), right.trim().into())
    } else if rest.contains('/') {
        let (left, right) = rest.split_once('/').unwrap();
        Job::Div(left.trim().into(), right.trim().into())
    } else {
        Job::Literal(str::parse(rest.trim()).unwrap())
    };
    (name.to_owned(), job)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
"#;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one_inner(TEST_INPUT).unwrap(), 152);
        assert_eq!(part_one().unwrap(), 256_997_859_093_114);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two_inner(TEST_INPUT).unwrap(), 301);
        assert_eq!(part_two().unwrap(), 3952288690726);
    }
}
