mod common;
mod part1;
mod part2;

use std::error::Error;

use aoc2022::utils::file;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = file::get_input_lines("day03")?;
    println!("part one: {}", part1::solve(lines)?);

    let lines = file::get_input_lines("day03")?;
    println!("part two: {}", part2::solve(lines)?);

    Ok(())
}
