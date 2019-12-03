mod advent;
mod utils;

fn main() -> Result<(), utils::errors::Error> {
    println!(
        "Day 1: Part One: Fuel Required: {}",
        advent::day01::part_one()?
    );
    println!(
        "Day 1: Part Two: Fuel Required: {}",
        advent::day01::part_two()?
    );
    Ok(())
}
