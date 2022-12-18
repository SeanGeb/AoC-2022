use std::error::Error;

use aoc2022::utils::file::get_input_lines;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = get_input_lines("day06")?;
    let input = lines.next().unwrap()?;
    assert!(lines.next().is_none());

    let input = input.as_str();
    println!("part one: {}", solve::<4>(input).unwrap());
    println!("part two: {}", solve::<14>(input).unwrap());

    Ok(())
}

pub fn solve<const N: usize>(input: &str) -> Option<usize> {
    //! solve finds the first run of N consecutive values in input that are all
    //! unique. It returns a None if no such result can exist given input.
    let mut buf: [char; N] = ['\0'; N];

    // This appears naive but for a buf of size 4, this will be much faster than
    // an "asymptotically optimal" data structure that allocs and indirects.
    for (i, c) in input.chars().enumerate() {
        buf[i % N] = c;

        // Skip if this is in the first N-1 elements.
        if i < N - 1 {
            continue;
        }

        // Check for any duplication in the buffer.
        if buf.iter().unique().count() == N {
            // Every item in the buffer is unique!
            // We're 0-indexed but the task is 1-indexed, so add 1.
            return Some(i + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve::<4>("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
        assert_eq!(solve::<4>("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(solve::<4>("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(solve::<4>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(solve::<4>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));

        assert_eq!(solve::<14>("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(solve::<14>("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(solve::<14>("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(solve::<14>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(solve::<14>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));

        assert_eq!(solve::<0>(""), None);
        assert_eq!(solve::<1>(""), None);
        assert_eq!(solve::<1>("a"), Some(1));
        assert_eq!(solve::<2>("a"), None);
    }
}
