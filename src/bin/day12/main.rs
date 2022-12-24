use std::error::Error;

use aoc2022::utils::file::get_input_lines;

use crate::dt::HMap;

mod dt;
mod parse;

fn main() -> Result<(), Box<dyn Error>> {
    let map = HMap::parse_from_lines(get_input_lines("day12")?)?;
    println!("part one: {}", map.find_part_one_dist());
    println!("part two: {}", map.find_part_two_dist());
    Ok(())
}
