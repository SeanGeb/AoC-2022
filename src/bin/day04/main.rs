use std::error::Error;

use aoc2022::utils::file::get_input_lines;

mod common;
mod part1;
mod part2;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = get_input_lines("day04")?;
    println!("part two: {}", part1::solve(lines)?);

    let lines = get_input_lines("day04")?;
    println!("part two: {}", part2::solve(lines)?);

    Ok(())
}
