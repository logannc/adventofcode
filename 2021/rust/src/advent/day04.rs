use crate::utils::*;
use itertools::Itertools;

type BingoBoard = Vec<(usize, bool)>;

fn check_row(board: &BingoBoard, row: usize) -> bool {
    let start = 5 * row;
    let row = &board[start..(start + 5)];
    row.iter().all(|(_, marked)| *marked)
}

fn check_column(board: &BingoBoard, col: usize) -> bool {
    board.iter().skip(col).step_by(5).all(|(_, marked)| *marked)
}

enum BoardResult {
    Miss,
    Score,
    Win(usize),
}

fn score(board: &BingoBoard) -> usize {
    board.iter().filter(|(_, m)| !m).map(|(n, _)| n).sum()
}

fn mark(board: &mut BingoBoard, draw: usize) -> BoardResult {
    let mut result = BoardResult::Miss;
    for idx in 0..board.len() {
        let (num, marked) = &mut board[idx];
        if *num == draw {
            *marked = true;
            let row = idx / 5;
            let col = idx % 5;
            if check_row(board, row) || check_column(board, col) {
                return BoardResult::Win(draw * score(board));
            } else {
                result = BoardResult::Score;
            }
        }
    }
    result
}

pub fn part_one() {
    let ip = problem_input_path(4, Some(1));
    let data: Vec<String> = read_file_split_whitespace(&ip).expect("failed to parse");
    let (numbers, boards) = data.split_at(1);
    let numbers: Result<Vec<usize>, _> = numbers[0].split(",").map(str::parse::<usize>).collect();
    let numbers = numbers.unwrap();
    let mut boards: Vec<BingoBoard> = boards
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .chunks(25)
        .into_iter()
        .map(|chunk| chunk.map(|num| (num, false)).collect::<BingoBoard>())
        .collect();
    for draw in numbers {
        for board in boards.iter_mut() {
            match mark(board, draw) {
                BoardResult::Win(score) => {
                    println!("Score of first win: {}", score);
                    return;
                }
                _ => {}
            }
        }
    }
}

pub fn part_two() {
    let ip = problem_input_path(4, Some(1));
    let data: Vec<String> = read_file_split_whitespace(&ip).expect("failed to parse");
    let (numbers, boards) = data.split_at(1);
    let numbers: Result<Vec<usize>, _> = numbers[0].split(",").map(str::parse::<usize>).collect();
    let numbers = numbers.unwrap();
    let mut boards: Vec<(BingoBoard, bool)> = boards
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .chunks(25)
        .into_iter()
        .map(|chunk| (chunk.map(|num| (num, false)).collect::<BingoBoard>(), false))
        .collect();
    let mut score = 0;
    for draw in numbers {
        for idx in 0..boards.len() {
            let (board, already_won) = &mut boards[idx];
            if *already_won {
                continue;
            }
            match mark(board, draw) {
                BoardResult::Win(s) => {
                    *already_won = true;
                    score = s;
                }
                _ => {}
            }
        }
    }
    println!("Score of last win: {}", score);
}
