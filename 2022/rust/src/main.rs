#![feature(iter_array_chunks)]
#![feature(array_windows)]
#![feature(get_many_mut)]
#![feature(int_roundings)]

mod advent;
mod utils;

use clap::Parser;
use eyre::Result;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    day: u8,
    #[arg(short, long)]
    part: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let start = std::time::Instant::now();
    match (args.day, args.part) {
        (1, 1) => {
            advent::day01::part_one()?;
        }
        (1, 2) => {
            advent::day01::part_two()?;
        }
        (2, 1) => {
            advent::day02::part_one()?;
        }
        (2, 2) => {
            advent::day02::part_two()?;
        }
        (3, 1) => {
            advent::day03::part_one()?;
        }
        (3, 2) => {
            advent::day03::part_two()?;
        }
        (4, 1) => {
            advent::day04::part_one()?;
        }
        (4, 2) => {
            advent::day04::part_two()?;
        }
        (5, 1) => {
            advent::day05::part_one()?;
        }
        (5, 2) => {
            advent::day05::part_two()?;
        }
        (6, 1) => {
            advent::day06::part_one()?;
        }
        (6, 2) => {
            advent::day06::part_two()?;
        }
        (7, 1) => {
            advent::day07::part_one()?;
        }
        (7, 2) => {
            advent::day07::part_two()?;
        }
        (8, 1) => {
            advent::day08::part_one()?;
        }
        (8, 2) => {
            advent::day08::part_two()?;
        }
        (9, 1) => {
            advent::day09::part_one()?;
        }
        (9, 2) => {
            advent::day09::part_two()?;
        }
        (10, 1) => {
            advent::day10::part_one()?;
        }
        (10, 2) => {
            advent::day10::part_two()?;
        }
        (11, 1) => {
            advent::day11::part_one()?;
        }
        (11, 2) => {
            advent::day11::part_two()?;
        }
        (12, 1) => {
            advent::day12::part_one()?;
        }
        (12, 2) => {
            advent::day12::part_two()?;
        }
        (13, 1) => {
            advent::day13::part_one()?;
        }
        (13, 2) => {
            advent::day13::part_two()?;
        }
        (14, 1) => {
            advent::day14::part_one()?;
        }
        (14, 2) => {
            advent::day14::part_two()?;
        }
        (15, 1) => {
            advent::day15::part_one()?;
        }
        (15, 2) => {
            advent::day15::part_two()?;
        }
        (16, 1) => {
            advent::day16::part_one()?;
        }
        (16, 2) => {
            advent::day16::part_two()?;
        }
        (17, 1) => {
            advent::day17::part_one()?;
        }
        (17, 2) => {
            advent::day17::part_two()?;
        }
        (18, 1) => {
            advent::day18::part_one()?;
        }
        (18, 2) => {
            advent::day18::part_two()?;
        }
        (19, 1) => {
            advent::day19::part_one()?;
        }
        (19, 2) => {
            advent::day19::part_two()?;
        }
        (20, 1) => {
            advent::day20::part_one()?;
        }
        (20, 2) => {
            advent::day20::part_two()?;
        }
        (21, 1) => {
            advent::day21::part_one()?;
        }
        (21, 2) => {
            advent::day21::part_two()?;
        }
        (d, p) => {
            println!("Day {d}, Part {p} is not yet implemented.");
        }
    }
    let elapsed = start.elapsed();
    println!("Elapsed: {elapsed:?}");
    Ok(())
}
