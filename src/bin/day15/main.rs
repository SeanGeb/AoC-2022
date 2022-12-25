#[macro_use]
extern crate scan_fmt;

use std::error::Error;

use aoc2022::utils::file::get_input_lines;
use parse::parse_lines;

#[allow(unused_imports)]
use part1::solve_part1;

use crate::part2::solve_part2;

mod parse;
mod part1;
mod part2;

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn Error>> {
    let (filename, part1_row, part2_lim) =
        // ("example/day15", 10, 20);
        ("day15", 2_000_000, 4_000_000);
    let sensors = parse_lines(get_input_lines(filename)?)?;
    println!("part one: {}", solve_part1(part1_row, &sensors));
    println!("part two: {}", solve_part2(part2_lim, &sensors));
    Ok(())
}
