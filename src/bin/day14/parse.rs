use std::error::Error;
use std::io;

use aoc2022::utils::parse::Parser;
use itertools::Itertools;

use crate::dt::State;

fn parse_line(line: &str) -> Vec<(u32, u32)> {
    let mut r: Vec<(u32, u32)> = Vec::new();
    let mut p: Parser = line.into();

    while !p.is_empty() {
        let x = p.u32();
        p.str(",");
        let y = p.u32();
        r.push((x, y));

        if !p.is_empty() {
            p.str(" -> ");
        }
    }

    r
}

fn parse_line_walls(
    line: &str,
) -> impl Iterator<Item = ((u32, u32), (u32, u32))> {
    parse_line(line).into_iter().tuple_windows()
}

pub fn parse_lines(
    lines: impl Iterator<Item = Result<String, io::Error>>,
) -> Result<State, Box<dyn Error>> {
    let mut s = State::new();
    for line in lines {
        let line = line?;
        for (from, to) in parse_line_walls(line.as_str()) {
            s.draw_rock(from, to);
        }
    }
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "1,2 -> 3,4 -> 5,6 -> 7,8";
        assert_eq!(parse_line(line), vec![(1, 2), (3, 4), (5, 6), (7, 8)]);
        assert_eq!(
            parse_line_walls(line).collect_vec(),
            vec![((1, 2), (3, 4)), ((3, 4), (5, 6)), ((5, 6), (7, 8)),]
        );
    }
}
