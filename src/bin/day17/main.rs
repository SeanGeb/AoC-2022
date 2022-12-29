use std::error::Error;

use aoc2022::utils::file::get_input_lines;

use crate::dt::*;

mod dt;

const PART2_TARGET: u64 = 1000000000000;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_iter = get_input_lines("day17")?;
    let input = input_iter.next().unwrap()?;
    assert!(input_iter.next().is_none());

    let jets = jets_from(input.as_str());
    let mut s = State::new();

    for _ in 0..2022 {
        s.drop_next_rock(&jets);
    }

    println!("part one: {}", s.height());

    let jets = jets_from(input.as_str());
    let mut s = State::new();

    let (cycle_from, cycle_to) = s.drop_rocks_memo(&jets);

    let mut height = cycle_from.total_height;
    let mut count = cycle_from.rock_count;
    let cycle_height = cycle_to.total_height - cycle_from.total_height;
    let cycle_count = cycle_to.rock_count - cycle_from.rock_count;

    let cycle_iterations = (PART2_TARGET - count) / cycle_count;
    let cycle_remainder = (PART2_TARGET - count) % cycle_count;

    height += cycle_height * cycle_iterations;
    count += cycle_count * cycle_iterations;

    let h_before = s.height();
    for _ in 0..cycle_remainder {
        s.drop_next_rock(&jets);
        count += 1;
    }
    height += (s.height() - h_before) as u64;

    println!("part two: {height}");

    Ok(())
}
