use crate::utils::*;
use eyre::{Report, Result};
use itertools::Itertools;
use std::{collections::BTreeMap, fmt::Display, fs};

pub fn part_one() -> Result<i128> {
    let input_path = problem_input_path(17, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_one_inner(&content)?;
    println!("{result}");
    Ok(result)
}

pub fn part_two() -> Result<i128> {
    let input_path = problem_input_path(17, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_two_inner(&content)?;
    println!("{result}");
    Ok(result)
}

fn part_one_inner(input: &str) -> Result<i128> {
    let directions = parse_input(input);
    let mut directions = directions.into_iter().cycle();
    let mut tunnel = Tunnel::default();
    let mut pieces = Piece::iter().map(|piece| piece.into());
    tunnel.add_piece(pieces.next().unwrap());
    let mut count = 1;
    while count <= 2022 {
        if tunnel.simulate_direction(directions.next().unwrap()) {
            tunnel.add_piece(pieces.next().unwrap());
            count += 1;
        }
    }
    Ok(tunnel.highest + 1)
}

fn part_two_inner(input: &str) -> Result<i128> {
    const GOAL: i128 = 1_000_000_000_000;
    let directions = parse_input(input);
    let mut directions = directions.into_iter().enumerate().cycle();
    let mut tunnel = Tunnel::default();
    let mut pieces = Piece::iter().map(|piece| piece.into());
    tunnel.add_piece(pieces.next().unwrap());
    let mut count: i128 = 1;
    let mut seen_states = BTreeMap::new();
    let mut count_heights = BTreeMap::new();
    while count <= GOAL {
        let state = tunnel.state_snapshot();
        let (move_idx, next_move) = directions.next().unwrap();
        if let Some((prior_count, prior_highest)) =
            seen_states.insert((move_idx, state), (count, tunnel.highest))
        {
            let elapsed = count - prior_count;
            if elapsed > 0 {
                println!("Found a cycle! {prior_count} to {count}");
                let remaining = dbg!(GOAL - count);
                let fast_forward_cycles = dbg!(remaining / elapsed);
                let growth = dbg!(tunnel.highest - prior_highest);
                tunnel.highest += dbg!(growth * fast_forward_cycles);
                count += dbg!(elapsed * fast_forward_cycles);
                let remaining_after_fast_forward = GOAL - count;
                let prior_difference_highest =
                    count_heights[&(prior_count + remaining_after_fast_forward)] - prior_highest;
                return Ok(tunnel.highest + prior_difference_highest + 1);
            }
        }
        if tunnel.simulate_direction(next_move) {
            tunnel.add_piece(pieces.next().unwrap());
            // println!("{}:\n{}", count, tunnel);
            count_heights.insert(count, tunnel.highest);
            count += 1;
        }
    }
    Ok(tunnel.highest + 1)
}

fn parse_input(input: &str) -> Vec<Direction> {
    let directions: Result<Vec<Direction>, _> = input
        .chars()
        .map(|c| {
            Ok(match c {
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => return Err(Report::msg(format!("Bad character [{c}]"))),
            })
        })
        .collect();
    directions.unwrap()
}

#[derive(Clone)]
enum Piece {
    Flat,
    Plus,
    L,
    Straight,
    Square,
}

#[derive(Clone)]
enum Direction {
    Left,
    Right,
}

impl Piece {
    fn iter() -> impl Iterator<Item = Piece> {
        vec![
            Piece::Flat,
            Piece::Plus,
            Piece::L,
            Piece::Straight,
            Piece::Square,
        ]
        .into_iter()
        .cycle()
    }
}

impl From<Piece> for Vec<u8> {
    fn from(value: Piece) -> Self {
        match value {
            Piece::Flat => vec![0b0011110],
            Piece::L => vec![0b0000100, 0b0000100, 0b0011100],
            Piece::Plus => vec![0b0001000, 0b0011100, 0b0001000],
            Piece::Straight => vec![0b0010000, 0b0010000, 0b0010000, 0b0010000],
            Piece::Square => vec![0b0011000, 0b0011000],
        }
    }
}

struct Tunnel {
    piece: Vec<u8>,
    piece_bottom: usize,
    rows: Vec<u8>,
    highest: i128,
}

impl Tunnel {
    fn add_piece(&mut self, piece: Vec<u8>) {
        self.piece = piece;
        self.piece_bottom = (self.highest + 4_i128) as usize;
        let extra_rows = (self.piece_bottom + self.piece.len()).saturating_sub(self.rows.len());
        for _ in 0..extra_rows {
            self.rows.push(0);
        }
    }

    fn tunnel_collision(&self, piece: &Vec<u8>, depth: usize) -> bool {
        piece
            .iter()
            .rev()
            .zip(self.rows[depth..depth + self.piece.len()].iter())
            .any(|(piece, row)| piece & row > 0)
    }

    fn move_left(&mut self) {
        let wall_collision = self.piece.iter().any(|r| (r & 0b100_0000) > 0);
        let candidate = self.piece.iter().map(|r| r << 1).collect();
        let tunnel_collision = self.tunnel_collision(&candidate, self.piece_bottom);
        if !wall_collision && !tunnel_collision {
            self.piece = candidate;
        }
    }
    fn move_right(&mut self) {
        let wall_collision = self.piece.iter().any(|r| (r & 0b000_0001) > 0);
        let candidate = self.piece.iter().map(|r| r >> 1).collect();
        let tunnel_collision = self.tunnel_collision(&candidate, self.piece_bottom);
        if !wall_collision && !tunnel_collision {
            self.piece = candidate;
        }
    }
    fn simulate_direction(&mut self, direction: Direction) -> bool {
        match direction {
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
        if self.piece_bottom == 0 || self.tunnel_collision(&self.piece, self.piece_bottom - 1) {
            for (piece_row, tunnel_row) in
                self.piece.iter().rev().zip(
                    &mut self.rows[self.piece_bottom..self.piece_bottom + self.piece.len() + 1],
                )
            {
                *tunnel_row |= piece_row;
            }
            self.highest = i128::max(
                self.highest,
                (self.piece_bottom + self.piece.len() - 1) as i128,
            );
            true
        } else {
            self.piece_bottom -= 1;
            false
        }
    }

    fn state_snapshot(&self) -> u128 {
        let mut state = 0;
        for (idx, row) in self
            .rows
            .iter()
            .rev()
            .skip_while(|r| **r == 0)
            .enumerate()
            .take(10)
        {
            state |= (*row as u128) << (idx * 8)
        }
        for (idx, row) in self.piece.iter().enumerate() {
            state |= (*row as u128) << ((15 - idx) * 8)
        }
        state
    }
}

impl Display for Tunnel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut row_strs = vec![];
        for (i, row) in self.rows.iter().enumerate() {
            let mut copy = *row;
            if self.piece_bottom <= i && i < self.piece_bottom + self.piece.len() {
                let idx = self.piece.len() - (i - self.piece_bottom) - 1;
                if let Some(piece_row) = self.piece.get(idx) {
                    copy |= piece_row;
                }
            }
            row_strs.push(format!("{copy:0>7b}"));
        }
        let tunnel = row_strs.into_iter().rev().join("\n");
        f.write_str(&tunnel)
    }
}

impl Default for Tunnel {
    fn default() -> Self {
        Tunnel {
            piece: vec![],
            piece_bottom: 0,
            rows: vec![],
            highest: -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one_inner(TEST_INPUT).unwrap(), 3068);
        assert_eq!(part_one().unwrap(), 3130);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two_inner(TEST_INPUT).unwrap(), 1_514_285_714_288);
        assert_eq!(part_two().unwrap(), 1556521739139);
    }
}
