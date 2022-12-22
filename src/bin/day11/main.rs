use std::error::Error;

use aoc2022::utils::file::get_input_lines;
use dt::State;

#[macro_use]
extern crate scan_fmt;

mod dt;

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = State::try_parse_from(get_input_lines("day11")?, 3)?;
    println!("{state}");
    for _ in 0..20 {
        state.step();
    }

    state.print_items_thrown();
    println!("part one: {}", state.monkey_business_value());

    let mut state = State::try_parse_from(get_input_lines("day11")?, 1)?;
    println!("{state}");
    for _ in 0..10_000 {
        state.step();
    }

    state.print_items_thrown();
    println!("part two: {}", state.monkey_business_value());

    Ok(())
}
