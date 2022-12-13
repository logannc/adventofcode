#![feature(iter_array_chunks)]
#![feature(array_windows)]
#![feature(get_many_mut)]

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
        (d, p) => {
            println!("Day {}, Part {} is not yet implemented.", d, p);
        }
    }
    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
    Ok(())
}
