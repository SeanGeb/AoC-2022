use std::error::Error;

use aoc2022::utils::file::get_input_lines;
use parse::State;

use crate::part1::solve_part1;
use crate::part2::solve_part2;

mod graph;
mod parse;
mod part1;
mod part2;

fn main() -> Result<(), Box<dyn Error>> {
    let s = State::parse(get_input_lines("day16")?)?;
    println!("part one: {}", solve_part1(&s));
    println!("part two: {}", solve_part2(&s));

    Ok(())
}
