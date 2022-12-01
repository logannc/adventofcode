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
    match (args.day, args.part) {
        (1, 1) => {
            advent::day01::part_one()?;
        }
        (1, 2) => {
            advent::day01::part_two()?;
        }
        (d, p) => {
            println!("Day {}, Part {} is not yet implemented.", d, p);
        }
    }
    Ok(())
}