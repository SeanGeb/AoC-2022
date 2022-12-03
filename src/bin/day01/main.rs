mod part1;
mod part2;

use std::error::Error;

use aoc2022::utils::file::get_input_lines;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = get_input_lines("day01")?;
    let part1 = part1::process_lines(lines)?;
    println!("part one: {part1}");

    let lines = get_input_lines("day01")?;
    let part2 = part2::process_lines(lines)?;
    println!("part two: {part2}");

    Ok(())
}
