#[macro_use]
extern crate scan_fmt;

use std::error::Error;

use aoc2022::utils::file::get_input_lines;
use parse::{Movement, State};

mod parse;

fn main() -> Result<(), Box<dyn Error>> {
    let moves: Vec<Movement> = get_input_lines("day09")?
        .map(|l| l.unwrap().as_str().try_into().unwrap())
        .collect();

    let mut s = State::new(2);
    moves.iter().for_each(|m| s.do_move(*m));
    println!("{s}");
    println!("part one: {}", s.count_visited());

    let mut s = State::new(10);
    moves.iter().for_each(|m| s.do_move(*m));
    println!("{s}");
    println!("part two: {}", s.count_visited());

    Ok(())
}
