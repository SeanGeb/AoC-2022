use std::error::Error;

use aoc2022::utils::file::get_input_lines;

use sol::*;

mod sol;

const PART2_KEY: i64 = 811589153;
const PART2_ROUNDS: u8 = 10;

fn main() -> Result<(), Box<dyn Error>> {
    let input = parse_input("day20")?;
    println!("part 1: {}", mix_and_score(&input).into_iter().sum::<i16>());

    let mut input: Vec<TaggedI64> = input
        .into_iter()
        .enumerate()
        .map(|(i, v)| TaggedI64 {
            val: (v as i64) * PART2_KEY,
            original_idx: i.try_into().unwrap(),
        })
        .collect();

    for _ in 0..PART2_ROUNDS {
        eprint!(".");
        perform_mix_part2(&mut input);
    }
    eprintln!();
    println!("part 2: {:?}", score_i64(&input).into_iter().sum::<i64>());

    Ok(())
}

fn parse_input(file: &str) -> Result<Vec<i16>, Box<dyn Error>> {
    let mut r: Vec<i16> = Vec::new();
    for line in get_input_lines(file)? {
        r.push(line?.parse()?);
    }

    Ok(r)
}
