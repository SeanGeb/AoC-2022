use std::fmt::{self, Write};
use std::iter::{self, repeat, repeat_with};

use itertools::{unfold, Itertools};

/// A Grid is an X by Y grid of items stored in row-major order.
pub struct Grid<T>(Vec<Vec<T>>);

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> {
    type Error = &'static str;

    /// Attempt to create a Grid from a Vec of Vecs. Returns an error iff the
    /// inner Vecs are not all an equal size. Assumes the input is in row-major
    /// order.
    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        if value.iter().map(|v| v.len()).all_equal() {
            Ok(Grid { 0: value })
        } else {
            Err("inner vecs have differing lengths")
        }
    }
}

impl<T: Copy> Grid<T> {
    /// Creates a new Grid using the given value of T. (To create a Grid from
    /// existing data, use try_into/try_from.)
    pub fn new(default: T, x: usize, y: usize) -> Self {
        repeat_with(|| repeat(default).take(x).collect_vec())
            .take(y)
            .collect_vec()
            .try_into()
            .unwrap()
    }
}

impl<T> Grid<T> {
    /// Enumerates every grid element in row-major order.
    pub fn enumerate(&self) -> Box<dyn Iterator<Item = ((usize, usize), &T)> + '_> {
        let mut iter: Box<dyn Iterator<Item = ((usize, usize), &T)>> = Box::new(iter::empty());
        for (y, row) in self.0.iter().enumerate() {
            iter = Box::new(iter.chain(row.iter().enumerate().map(move |(x, val)| ((x, y), val))))
        }
        iter
    }

    /// Gets a ref to the item at (x, y); returns a None if those indexes are
    /// out of bounds for the grid in question.
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.0.get(y)?.get(x)
    }

    /// As get, but returns a mutable ref.
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.0.get_mut(y)?.get_mut(x)
    }

    /// Returns the height of the grid (i.e. y in 0..height is valid).
    pub fn height(&self) -> usize {
        self.0.len()
    }

    /// Returns an iterator of iterators of rows in right then down order.
    pub fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = ((usize, usize), &T)>> {
        self.0
            .iter()
            .enumerate()
            .map(|(y, row)| row.iter().enumerate().map(move |(x, val)| ((x, y), val)))
    }

    /// Returns an iterator of iterators of rows in left then down order.
    pub fn iter_rev_rows(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = ((usize, usize), &T)>> {
        self.0.iter().enumerate().map(|(y, row)| {
            row.iter()
                .enumerate()
                .rev()
                .map(move |(x, val)| ((x, y), val))
        })
    }

    /// Returns an iterator of iterators of cols in down then right order.
    pub fn iter_cols(&self) -> impl Iterator<Item = impl Iterator<Item = ((usize, usize), &T)>> {
        unfold(
            // Initial state is a set of iters over rows, enumerated to give x
            // values, and collected to a vec.
            self.0
                .iter()
                .enumerate()
                .map(|(y, row)| (y, row.iter().enumerate()))
                .collect_vec(),
            // For each row iter, build a vec over all the next vals, then yield
            // those as an iterator.
            |row_iters| {
                let row_iters_nexts = row_iters
                    .iter_mut()
                    .map(|(y, row_iter)| (y.to_owned(), row_iter.next()))
                    .collect_vec();
                if row_iters_nexts
                    .iter()
                    .all(|(_, row_iter_next)| row_iter_next.is_none())
                {
                    None
                } else {
                    Some(row_iters_nexts.into_iter().map(|(y, x_val)| match x_val {
                        Some((x, val)) => ((x, y), val),
                        None => unreachable!(),
                    }))
                }
            },
        )
    }

    /// Returns an iterator of iterators of cols in up then right order.
    pub fn iter_rev_cols(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = ((usize, usize), &T)>> {
        unfold(
            self.0
                .iter()
                .enumerate()
                .map(|(y, row)| (y, row.iter().enumerate()))
                .collect_vec(),
            |row_iters| {
                let row_iters_nexts = row_iters
                    .iter_mut()
                    .rev()
                    .map(|(y, row_iter)| (y.to_owned(), row_iter.next()))
                    .collect_vec();
                if row_iters_nexts
                    .iter()
                    .all(|(_, row_iter_next)| row_iter_next.is_none())
                {
                    None
                } else {
                    Some(row_iters_nexts.into_iter().map(|(y, x_val)| match x_val {
                        Some((x, val)) => ((x, y), val),
                        None => unreachable!(),
                    }))
                }
            },
        )
    }

    /// Turns an (x, y) into its position in the row-major order.
    pub fn row_major_pos(&self, x: usize, y: usize) -> Result<usize, &'static str> {
        if (0..self.width()).contains(&x) && (0..self.height()).contains(&y) {
            Ok(x.checked_add(self.width().checked_mul(y).unwrap()).unwrap())
        } else {
            Err("(x, y) given is out of range for this grid")
        }
    }

    /// Returns the total number of items in the grid.
    pub fn size(&self) -> usize {
        self.width().checked_mul(self.height()).unwrap()
    }

    /// Returns the width of the grid (i.e. x in 0..width is valid).
    pub fn width(&self) -> usize {
        match self.0.get(0) {
            None => 0,
            Some(v) => v.len(),
        }
    }
}

/// FixedWidthFormat is a marker trait indicating the type in question can be
/// formatted into a fixed width.
pub trait FixedWidthDisplay: fmt::Display {}
impl FixedWidthDisplay for char {}

impl<T: FixedWidthDisplay> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for val in row {
                val.fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl fmt::Display for Grid<bool> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for val in row {
                f.write_char(match val {
                    true => 'T',
                    false => 'F',
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::types::digit::Digit;

    use super::*;

    #[test]
    fn test_iters() {
        let grid: Grid<Digit> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
            .into_iter()
            .map(|r| r.into_iter().map(|x| x.try_into().unwrap()).collect())
            .collect::<Vec<Vec<Digit>>>()
            .try_into()
            .unwrap();

        println!("{grid}");

        println!("rows:");
        grid.iter_rows()
            .map(|row| row.for_each(|x| print!("{x:?}\t")))
            .for_each(|()| println!(""));
        println!("rev rows");
        grid.iter_rev_rows()
            .map(|row| row.for_each(|x| print!("{x:?}\t")))
            .for_each(|()| println!(""));
        println!("cols");
        grid.iter_cols()
            .map(|row| row.for_each(|x| print!("{x:?}\t")))
            .for_each(|()| println!(""));
        println!("rev cols");
        grid.iter_rev_cols()
            .map(|row| row.for_each(|x| print!("{x:?}\t")))
            .for_each(|()| println!(""));
    }
}
