#[macro_use]
extern crate scan_fmt;

use std::error::Error;

use rayon::prelude::*;

use dt::*;
use state::*;

mod dt;
mod state;

const PART1_MINUTES: u16 = 24;
const PART2_MINUTES: u16 = 32;

fn main() -> Result<(), Box<dyn Error>> {
    let blueprints = get_blueprints("day19")?;

    let part1: u16 = blueprints
        .0
        .par_iter()
        .map(|bp| {
            StateSet::get_at_time(bp, PART1_MINUTES).get_quality_score(bp)
        })
        .inspect(|r| eprintln!("got a result: {r}"))
        .sum();

    println!("part one: {part1}");

    let part2: u32 = blueprints
        .0
        .iter()
        .take(3)
        .par_bridge()
        .map(|bp| StateSet::get_at_time(bp, PART2_MINUTES).get_max_geodes())
        .inspect(|r| eprintln!("got a result: {r}"))
        .map(|s| s as u32)
        .product();

    println!("part two: {part2}");

    Ok(())
}
