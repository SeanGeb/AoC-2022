use std::cmp;
use std::collections::BinaryHeap;
use std::fmt;
use std::io;
use std::num;

pub fn process_lines(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<u64, Err> {
    Ok(find_top_n::<3>(lines)?.iter().sum())
}

/// find_top_n retrieves the largest n lines of numbers, performing the required
/// summation/resetting along the way.
fn find_top_n<const N: usize>(
    lines: impl Iterator<Item = Result<String, io::Error>>,
) -> Result<[u64; N], Err> {
    let mut heap = BinaryHeap::with_capacity(N + 1);
    let mut sum_this_one: u64 = 0;

    let mut add_val = |line: String| -> Result<(), num::ParseIntError> {
        if line.is_empty() {
            heap.push(cmp::Reverse(sum_this_one));
            sum_this_one = 0;

            if heap.len() > N {
                heap.pop();
            }
        } else {
            sum_this_one += line.parse::<u64>()?;
        }

        Ok(())
    };

    for line in lines {
        add_val(line?)?
    }

    add_val("".into())?;

    // Raise error if too few values were provided.
    if heap.len() != N {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("too few values provided (got {}, wanted {})", heap.len(), N),
        )
        .into());
    }

    // Empty the heap into an array.
    let mut res = [0u64; N];
    for res_i in res.iter_mut() {
        *res_i = heap.pop().unwrap().0;
    }

    Ok(res)
}

/// Err may be an io::Error or a num::ParseIntError.
#[derive(Debug)]
pub enum Err {
    IOError(io::Error),
    ParseIntError(num::ParseIntError),
}

impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Err::IOError(e) => e.fmt(f),
            Err::ParseIntError(e) => e.fmt(f),
        }
    }
}

impl From<io::Error> for Err {
    fn from(e: io::Error) -> Self {
        Err::IOError(e)
    }
}

impl From<num::ParseIntError> for Err {
    fn from(e: num::ParseIntError) -> Self {
        Err::ParseIntError(e)
    }
}

impl From<Err> for Box<dyn std::error::Error> {
    fn from(e: Err) -> Box<dyn std::error::Error> {
        e.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2022::utils::file::get_input_lines;

    #[test]
    fn test_find_top_3() {
        let lines = get_input_lines("example/day01").unwrap();
        let res = find_top_n::<3>(lines).unwrap();
        assert_eq!(res, [24000, 11000, 10000]);
    }

    #[test]
    fn test_process_lines() {
        let lines = get_input_lines("example/day01").unwrap();
        let res = process_lines(lines).unwrap();
        assert_eq!(res, 45000);
    }
}
