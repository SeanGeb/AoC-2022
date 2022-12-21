#[macro_use]
extern crate scan_fmt;

use std::error::Error;

use aoc2022::utils::file::get_input_lines;

mod dt;
mod parse;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = get_input_lines("day10")?;
    let instrs = parse::parse_lines(lines);

    let mut machine = dt::Machine::new();
    instrs.for_each(|i| machine.exec(i.unwrap()));
    println!("part one: {}", machine.get_part1_score()?);

    println!("part two:\n{machine}");

    Ok(())
}
