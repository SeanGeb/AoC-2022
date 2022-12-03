mod dt;
mod part1;
mod part2;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Each line is of the form [ABC] [XYZ]. A/X beats B/Y beats C/Z beats A/X.
    let moves = dt::parse_moves_from_file("day02")?;
    let score = part1::score_all_moves(moves)?;
    println!("part one: {score}");

    let moves = dt::parse_moves_from_file("day02")?;
    let resolved_moves = part2::resolve_moves(moves);
    let score = part1::score_all_moves(resolved_moves)?;
    println!("part two: {score}");

    Ok(())
}
