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
        "Day 4: Part Two: Secure Container Fewer Dupes: {}",
        advent::day04::part_two()? // 1419
    );
    println!(
        "Day 5: Part One: Emulator Enhancement: {}",
        advent::day05::part_one()? // 16434972
    );
    println!(
        "Day 5: Part Two: Emulator Enhancement: {}",
        advent::day05::part_two()? // 16694270
    );
    println!(
        "Day 6: Part One: Orbits Dont Work This Way: {}",
        advent::day06::part_one()? // 278744
    );
    println!(
        "Day 6: Part Two: I am a Steely-eyed Missile Man: {}",
        advent::day06::part_two()? // 475
    );
    println!(
        "Day 7: Part One: Maximizing Warp Drives: {}",
        advent::day07::part_one()? // 199988
    );
    println!(
        "Day 7: Part Two: Resumable Tape Machine: {}",
        advent::day07::part_two()? // 17519904
    );
    println!(
        "Day 8: Part One: Space Selfies: {}",
        advent::day08::part_one()? // 1360
    );
    println!(
        "Day 8: Part Two: Space Memes:",
    );
    advent::day08::part_two()?; // FPUAR
    Ok(())
}
