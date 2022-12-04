// Summary: given inclusive ranges of the form A-B,X-Y, count the number of
// overlaps.

use itertools::Itertools;
use std::error::Error;
use std::io;

use crate::common;

pub fn solve(
    lines: impl Iterator<Item = Result<String, io::Error>>,
) -> Result<u64, Box<dyn Error>> {
    Ok(lines
        .map(|l| common::parse_line(l?.as_str()))
        .filter_ok(|(r1, r2)| r1.overlaps_with(r2))
        .count() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2022::utils::file::get_input_lines;

    #[test]
    fn test_solve() {
        let lines = get_input_lines("example/day04").unwrap();
        let res = solve(lines).unwrap();
        assert_eq!(res, 4);
    }
}
