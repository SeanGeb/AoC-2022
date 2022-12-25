use std::error::Error;

use aoc2022::utils::file::get_input_lines;

use crate::parse::parse_lines;

mod dt;
mod parse;

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = parse_lines(get_input_lines("day14")?)?;
    println!("{s}");
    let sand_added = s.add_sand_to_stable();
    println!("{s}\npart one: {sand_added}");

    let mut s = parse_lines(get_input_lines("day14")?)?;
    s.draw_floor();
    println!("drew the floor");
    println!("{s}");
    let sand_added = s.add_sand_to_stable();
    println!("{s}\npart two: {sand_added}");
    Ok(())
}
