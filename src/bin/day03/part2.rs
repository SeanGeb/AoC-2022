use std::{collections::HashSet, io};

use aoc2022::utils::error::invalid_data_err_from;
use itertools::{self, Itertools};

use crate::common;

pub fn solve(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<u64, io::Error> {
    // Summary for part 2: pop three lines and find the common item, and score them.
    lines
        .chunks(3)
        .into_iter()
        .map(|mut lines| -> Result<u64, io::Error> {
            let line1 = lines.next().unwrap()?;

            let mut common: HashSet<u8> = HashSet::from_iter(line1.bytes());

            for line in lines {
                let line: HashSet<u8> = HashSet::from_iter(line?.bytes());
                common.retain(|c| line.contains(c));
            }

            let common_item = match common.into_iter().exactly_one() {
                Ok(x) => x,
                Err(e) => return Err(invalid_data_err_from(e)),
            };

            Ok(common::score(&common_item))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use aoc2022::utils::file::get_input_lines;

    use super::*;

    #[test]
    fn test_solve() {
        let lines = get_input_lines("example/day03").unwrap();
        assert_eq!(solve(lines).unwrap(), 70);
    }
}
