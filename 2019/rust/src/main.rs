mod advent;
mod utils;

fn main() -> Result<(), utils::errors::Error> {
    println!(
        "Day 1: Part One: Fuel Required: {}",
        advent::day01::part_one()? // 3576689
    );
    println!(
        "Day 1: Part Two: Fuel Required: {}",
        advent::day01::part_two()? // 5362136
    );
    println!(
        "Day 2: Part One: Gravity Assist: {}",
        advent::day02::part_one()? // 9581917
    );
    println!(
        "Day 2: Part Two: Gravity Assist: {}",
        advent::day02::part_two()? // 2505
    );
    println!(
        "Day 3: Part One: Crossed Wires: {}",
        advent::day03::part_one()? // 709
    );
    println!(
        "Day 3: Part Two: Wire Length: {}",
        advent::day03::part_two()? // 13836
    );
    println!(
        "Day 4: Part One: Secure Container: {}",
        advent::day04::part_one()? // 2090
    );
    println!(
        "Day 4: Part One: Secure Container Fewer Dupes: {}",
        advent::day04::part_two()? // 1419
    );
    Ok(())
}
