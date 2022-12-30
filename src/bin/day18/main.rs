#[macro_use]
extern crate scan_fmt;

use std::error::Error;

use aoc2022::utils::file::get_input_lines;
use dt::parse_lines;

use crate::part1::solve_part1;
use crate::part2::solve_part2;

mod dt;
mod part1;
mod part2;

fn main() -> Result<(), Box<dyn Error>> {
    let voxels = parse_lines(get_input_lines("day18")?)?;
    println!("part one: {}", solve_part1(&voxels));
    println!("part two: {}", solve_part2(&voxels));
    Ok(())
}
