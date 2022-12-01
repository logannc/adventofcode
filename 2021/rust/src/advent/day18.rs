use std::{iter::Peekable, ops::Add};

use crate::utils::*;

#[derive(Debug, Clone)]
struct SnailNumber {
    left: Box<Element>,
    right: Box<Element>,
}

impl SnailNumber {
    // extra gross because the one is added to a regular number in the node
    // the other is added to the next left or right digit, potentially in an entirely different part of the tree!
    // have to backtrack with the number up
    fn explode(&mut self, depth: u32) -> bool {
        println!("depth {}, left: {:?}, right: {:?}", depth, self.left, self.right);
        match *self.left {
            Element::Pair(ref mut sn) => {
                if depth >= 3 {
                    println!("we should be exploding");
                    *self.right = Element::Regular(match (*sn.right.clone(), *self.right.clone()) {
                        (Element::Regular(ref a), Element::Regular(ref b)) => a + b,
                        _ => panic!("oops"),
                    });

                    // *self.left = Element::Regular(0);
                    println!("post-splode: {:?}", self);
                    return true;
                } else if sn.explode(depth + 1) {
                    return true;
                }
            }
            _ => {}
        }
        match *self.right.clone() {
            Element::Pair(ref mut sn) => {
                if depth >= 3 {
                    *self.right = Element::Regular(0);
                    *self.left = Element::Regular(match (*sn.left.clone(), *self.left.clone()) {
                        (Element::Regular(ref a), Element::Regular(ref b)) => a + b,
                        _ => panic!("oops"),
                    });
                    return true;
                } else if sn.explode(depth + 1) {
                    return true;
                }
            }
            _ => {}
        }
        false
    }

    fn split(&mut self) -> bool {
        false
    }

    fn reduce(mut self) -> Self {
        loop {
            if self.explode(0) {
                break;
                continue;
            } else if self.split() {
                continue;
            } else {
                break;
            }
        }
        self
    }
}

impl Add<SnailNumber> for SnailNumber {
    type Output = SnailNumber;

    fn add(self, rhs: SnailNumber) -> Self::Output {
        SnailNumber {
            left: Box::new(Element::Pair(self)),
            right: Box::new(Element::Pair(rhs)),
        }
        .reduce()
    }
}

#[derive(Debug, Clone)]
enum Element {
    Regular(u32),
    Pair(SnailNumber),
}

fn parse_element(characters: &mut impl Iterator<Item = char>) -> Element {
    loop {
        match characters.next().unwrap() {
            '[' => {
                return Element::Pair(SnailNumber {
                    left: Box::new(parse_element(characters)),
                    right: Box::new(parse_element(characters)),
                });
            }
            ']' => {}
            ',' => {}
            d => {
                return Element::Regular(d.to_digit(10).unwrap());
            }
        }
    }
}

fn parse_inner(characters: &mut impl Iterator<Item = char>) -> SnailNumber {
    SnailNumber {
        left: Box::new(parse_element(characters)),
        right: Box::new(parse_element(characters)),
    }
}

fn parse_line(line: String) -> SnailNumber {
    parse_inner(&mut line.chars().peekable().skip(1))
}

pub fn part_one() {
    let ip = problem_input_path(18, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    // assert!(lines.len() == 1);
    // let target_area = parse_input(&lines[0]);
    // let (x, y) = high_shot(&target_area);
    // let mut max_y = 0;
    // let mut probe = Probe::new(x, y);
    // while !contains(&probe, &target_area) {
    //     probe = step(probe);
    //     if probe.position.1 > max_y {
    //         max_y = probe.position.1;
    //     }
    //     if probe.position.1 < target_area.y.0 {
    //         panic!("oops")
    //     }
    // }
    // println!("{}", max_y);
}

pub fn part_two() {
    let ip = problem_input_path(18, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    // assert!(lines.len() == 1);
    // let target_area = parse_input(&lines[0]);
    // let solution_count = count_solutions(&target_area);
    // println!("{}", solution_count);
}

#[test]
fn example() {
    let snail = parse_line("[[[[[9,8],1],2],3],4]".to_string());
    println!("{:?}", snail.reduce());
    let example = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;
    let lines: Vec<String> = example.split("\n").map(|s| s.to_string()).collect();
    println!("{:?}", parse_line(lines[0].clone()));

}
