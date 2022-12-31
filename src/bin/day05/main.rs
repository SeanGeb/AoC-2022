#[macro_use]
extern crate scan_fmt;

use std::error::Error;

use aoc2022::utils::file::get_input_lines;

mod dt;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = get_input_lines("day05")?;
    println!("part one: {}", solve(lines, dt::MoveType::Restack)?);

    let lines = get_input_lines("day05")?;
    println!("part two: {}", solve(lines, dt::MoveType::Block)?);

    Ok(())
}

use std::io;

use itertools::Itertools;

pub fn solve(
    mut lines: impl Iterator<Item = Result<String, io::Error>>,
    move_type: dt::MoveType,
) -> Result<String, Box<dyn Error>> {
    let mut state = dt::State::new_from_lines(&mut lines)?;
    dt::Move::parse_from_lines(lines, move_type)
        .for_each(|m| state.apply_move(&m.unwrap()));

    Ok(state
        .get_top_of_stacks()
        .iter()
        .filter_map(|o| o.as_ref())
        .join(""))
}
