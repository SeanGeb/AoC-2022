use std::error::Error;

use aoc2022::utils::file::get_input_lines;
use dt::parse;

use crate::dt::MaybeVec;

mod dt;

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = get_input_lines("day13")?;
    let mut pair = 1;
    let mut score = 0;
    loop {
        let line_1 = match lines.next() {
            None => break,
            Some(l) => l?,
        };
        let line_1 = parse(line_1.as_str());
        let line_2 = parse(lines.next().expect("unexpected end of input")?.as_str());
        assert!(match lines.next() {
            Some(l) => l?.is_empty(),
            None => true,
        });
        if line_1 <= line_2 {
            score += pair;
        }
        pair += 1;
    }
    println!("part one: {score}");

    let mut packets: Vec<MaybeVec> = vec![parse("[[2]]"), parse("[[6]]")];
    for line in get_input_lines("day13")? {
        let line = line?;
        if !line.is_empty() {
            packets.push(parse(line.as_str()));
        }
    }

    packets.sort();

    let idx_div_2 = packets
        .binary_search(&parse("[[2]]"))
        .expect("unable to find [[2]]")
        + 1;
    let idx_div_6 = packets
        .binary_search(&parse("[[6]]"))
        .expect("unable to find [[6]]")
        + 1;
    println!("part two: {}", idx_div_2 * idx_div_6);

    Ok(())
}
