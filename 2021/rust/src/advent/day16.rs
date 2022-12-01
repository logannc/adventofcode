use crate::utils::*;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Debug)]
struct Literal {
    version: u64,
    ty: u64,
    value: u64,
}

#[derive(PartialEq, Eq, Debug)]
struct Operator {
    version: u64,
    ty: u64,
    subpackets: Vec<Packet>,
}

#[derive(PartialEq, Eq, Debug)]
enum Packet {
    Lit(Literal),
    Op(Operator),
}

#[derive(Debug)]
struct BitStream {
    buffer: VecDeque<bool>,
    characters: VecDeque<char>,
}

impl BitStream {
    fn is_empty(&self) -> bool {
        self.buffer.is_empty() && self.characters.is_empty()
    }
    // fn clear_buffer(&mut self) {
    //     self.buffer.clear();
    // }
}

impl From<Vec<bool>> for BitStream {
    fn from(v: Vec<bool>) -> Self {
        BitStream {
            buffer: v.into(),
            characters: Default::default(),
        }
    }
}

impl Iterator for BitStream {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(bit) = self.buffer.pop_front() {
            Some(bit)
        } else if let Some(c) = self.characters.pop_front() {
            let bits = c.to_digit(16).unwrap();
            // println!("{} = '{}'", c, format!("{:04b}", bits));
            for c in format!("{:04b}", bits).chars() {
                // println!("'{}'", c);
                self.buffer.push_back(match c {
                    '0' => false,
                    '1' => true,
                    _ => unreachable!(),
                });
            }
            self.buffer.pop_front()
        } else {
            None
        }
    }
}

fn parse_line(line: &str) -> BitStream {
    BitStream {
        buffer: Default::default(),
        characters: line.chars().collect(),
    }
}

fn bools_to_bits(bools: impl Iterator<Item = bool>) -> u64 {
    let bools: Vec<bool> = bools.collect();
    bools
        .into_iter()
        .rev()
        .enumerate()
        .map(|(idx, b)| (b as u64) << idx)
        .sum()
}

fn parse_stream(stream: &mut BitStream) -> Packet {
    // println!("entering new parsing with {:?}", stream);
    let version = bools_to_bits(stream.by_ref().take(3));
    let ty = bools_to_bits(stream.by_ref().take(3));
    match ty {
        4 => {
            let mut bools = Vec::new();
            // let mut taken = 6;
            while let Some(b) = stream.next() {
                // taken += 5;
                if b {
                    bools.extend(stream.by_ref().take(4));
                } else {
                    bools.extend(stream.by_ref().take(4));
                    break;
                }
            }
            let value = bools_to_bits(bools.into_iter());
            // let alignment = taken % 4;
            // println!("{}, {:?}", alignment, stream);
            // if alignment != 0 {
            //     let _drop: Vec<bool> = stream.by_ref().take(4 - alignment).collect();
            // }
            // println!("{:?}", stream);
            Packet::Lit(Literal { version, ty, value })
        },
        _ => {
            let mut subpackets = Vec::new();
            if stream.next().unwrap() {
                let number_of_subpackets = bools_to_bits(stream.by_ref().take(11));
                for _ in 0..number_of_subpackets {
                    subpackets.push(parse_stream(stream));
                }
            } else {
                let number_of_bits = bools_to_bits(stream.by_ref().take(15)) as usize;
                let bits: Vec<bool> = stream.by_ref().take(number_of_bits).collect();
                let mut bits: BitStream = bits.into();
                while !bits.is_empty() {
                    subpackets.push(parse_stream(&mut bits));
                }
            }
            Packet::Op(Operator {
                version,
                ty,
                subpackets,
            })
        }
    }
}

fn version_sum(packet: &Packet) -> u64 {
    match packet {
        Packet::Lit(literal) => literal.version,
        Packet::Op(operator) => {
            let subpacket_sum: u64 = operator.subpackets.iter().map(version_sum).sum();
            operator.version + subpacket_sum
        }
    }
}

fn evaluate(packet: &Packet) -> u64 {
    match packet {
        Packet::Lit(literal) => literal.value,
        Packet::Op(operator) => {
            match operator.ty {
                0 => {
                    // sum
                    operator.subpackets.iter().map(evaluate).sum()
                }
                1 => {
                    // product
                    operator.subpackets.iter().map(evaluate).product()
                }
                2 => {
                    // minimum
                    operator.subpackets.iter().map(evaluate).min().unwrap()
                }
                3 => {
                    // max
                    operator.subpackets.iter().map(evaluate).max().unwrap()
                }
                5 => {
                    // greater than
                    if let [first, second] = &*operator.subpackets {
                        if evaluate(first) > evaluate(second) {
                            1
                        } else {
                            0
                        }
                    } else {
                        panic!("bad pattern match for gt")
                    }
                }
                6 => {
                    // less than
                    if let [first, second] = &*operator.subpackets {
                        if evaluate(first) < evaluate(second) {
                            1
                        } else {
                            0
                        }
                    } else {
                        panic!("bad pattern match for gt")
                    }
                }
                7 => {
                    // equal to
                    if let [first, second] = &*operator.subpackets {
                        if evaluate(first) == evaluate(second) {
                            1
                        } else {
                            0
                        }
                    } else {
                        panic!("bad pattern match for gt")
                    }
                }
                _ => panic!("ahhh!"),
            }
        }
    }
}

pub fn part_one() {
    let ip = problem_input_path(16, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    assert!(lines.len() == 1);
    let mut bits = parse_line(&lines[0]);
    let packet = parse_stream(&mut bits);
    println!("{}", version_sum(&packet));
}

pub fn part_two() {
    let ip = problem_input_path(16, Some(1));
    let lines: Vec<String> = read_file_split_on(&ip, "\n").unwrap();
    assert!(lines.len() == 1);
    let mut bits = parse_line(&lines[0]);
    let packet = parse_stream(&mut bits);
    println!("{}", evaluate(&packet));
}

#[test]
fn example() {
    let examples = vec![
        (
            "D2FE28",
            Packet::Lit(Literal {
                version: 6,
                ty: 4,
                value: 2021,
            }),
            6,
        ),
        (
            "38006F45291200",
            Packet::Op(Operator {
                version: 1,
                ty: 6,
                subpackets: vec![
                    Packet::Lit(Literal {
                        version: 6,
                        ty: 4,
                        value: 10,
                    }),
                    Packet::Lit(Literal {
                        version: 2,
                        ty: 4,
                        value: 20,
                    }),
                ],
            }),
            9,
        ),
        (
            "EE00D40C823060",
            Packet::Op(Operator {
                version: 7,
                ty: 3,
                subpackets: vec![
                    Packet::Lit(Literal {
                        version: 2,
                        ty: 4,
                        value: 1,
                    }),
                    Packet::Lit(Literal {
                        version: 4,
                        ty: 4,
                        value: 2,
                    }),
                    Packet::Lit(Literal {
                        version: 1,
                        ty: 4,
                        value: 3,
                    }),
                ],
            }),
            14,
        ),
        (
            "8A004A801A8002F478",
            Packet::Op(Operator {
                version: 4,
                ty: 2,
                subpackets: vec![Packet::Op(Operator {
                    version: 1,
                    ty: 2,
                    subpackets: vec![Packet::Op(Operator {
                        version: 5,
                        ty: 2,
                        subpackets: vec![Packet::Lit(Literal {
                            version: 6,
                            ty: 4,
                            value: 15,
                        })],
                    })],
                })],
            }),
            16,
        ),
        // "620080001611562C8802118E34",
        // "C0015000016115A2E0802F182340",
        // "A0016C880162017C3686B18A3D4780",
    ];
    for (line, expected, expected_version_sum) in examples {
        let mut bits = parse_line(line);
        let packet = parse_stream(&mut bits);
        assert_eq!(packet, expected);
        assert_eq!(version_sum(&packet), expected_version_sum);
    }

    let examples = vec![
        ("C200B40A82", 3),
        ("04005AC33890", 54),
        ("880086C3E88112", 7),
        ("CE00C43D881120", 9),
        ("D8005AC2A8F0", 1),
        ("F600BC2D8F", 0),
        ("9C005AC2F8F0", 0),
        ("9C0141080250320F1802104A08", 1)
    ];
    for (line, expected) in examples {
        let mut bits = parse_line(line);
        let packet = parse_stream(&mut bits);
        assert_eq!(evaluate(&packet), expected);
    }
}
