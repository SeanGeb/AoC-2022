use std::cmp::{max, min};
use std::fmt::{self, Write};
use std::iter::{self, repeat, repeat_with};

use itertools::{unfold, Itertools};

/// A Dir is a cardinal direction.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

/// A Grid is an X by Y grid of items stored in row-major order.
#[derive(Debug, Eq, Hash, PartialEq)]
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
    pub fn enumerate(
        &self,
    ) -> Box<dyn Iterator<Item = ((usize, usize), &T)> + '_> {
        let mut iter: Box<dyn Iterator<Item = ((usize, usize), &T)>> =
            Box::new(iter::empty());
        for (y, row) in self.0.iter().enumerate() {
            iter = Box::new(iter.chain(
                row.iter().enumerate().map(move |(x, val)| ((x, y), val)),
            ))
        }
        iter
    }

    /// Enumerates up to four neighbours in the up/down/left/right direction
    /// around the point given.
    pub fn enumerate_n4(
        &self,
        (x, y): (usize, usize),
    ) -> impl Iterator<Item = (Dir, &T)> {
        let dirs: [((isize, isize), Dir); 4] = [
            ((-1, 0), Dir::Left),
            ((1, 0), Dir::Right),
            ((0, 1), Dir::Up),
            ((0, -1), Dir::Down),
        ];
        dirs.into_iter().filter_map(move |((d_x, d_y), dir)| {
            Some((
                dir,
                self.get(checked_u_add_i(x, d_x)?, checked_u_add_i(y, d_y)?)?,
            ))
        })
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
    pub fn iter_rows(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = ((usize, usize), &T)>> {
        self.0.iter().enumerate().map(|(y, row)| {
            row.iter().enumerate().map(move |(x, val)| ((x, y), val))
        })
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
    pub fn iter_cols(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = ((usize, usize), &T)>> {
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
                    Some(row_iters_nexts.into_iter().map(|(y, x_val)| {
                        match x_val {
                            Some((x, val)) => ((x, y), val),
                            None => unreachable!(),
                        }
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
                    Some(row_iters_nexts.into_iter().map(|(y, x_val)| {
                        match x_val {
                            Some((x, val)) => ((x, y), val),
                            None => unreachable!(),
                        }
                    }))
                }
            },
        )
    }

    /// Turns an (x, y) into its position in the row-major order.
    pub fn row_major_pos(
        &self,
        x: usize,
        y: usize,
    ) -> Result<usize, &'static str> {
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

    /// Taxicab distance between two points in the grid. None if either point is
    /// not in the grid.
    pub fn taxicab_dist(
        &self,
        from: (usize, usize),
        to: (usize, usize),
    ) -> Option<usize> {
        if self.get(from.0, from.1).is_none() || self.get(to.0, to.1).is_none()
        {
            return None;
        }

        let d_x = max(from.0, to.0) - min(from.0, to.0);
        let d_y = max(from.1, to.1) - min(from.1, to.1);
        return Some(d_x + d_y);
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

fn checked_u_add_i(a: usize, b: isize) -> Option<usize> {
    let a: isize = a.try_into().ok()?;
    a.checked_add(b)?.try_into().ok()
}

#[cfg(test)]
mod tests {
    use crate::utils::test::assert_vec_eq_multiset;

    use super::*;

    #[test]
    fn test_iters() {
        let grid: Grid<u8> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
            .into_iter()
            .map(|r| r.into_iter().map(|x| x.try_into().unwrap()).collect())
            .collect::<Vec<Vec<u8>>>()
            .try_into()
            .unwrap();

        println!("{grid:?}");

        assert_eq!(
            grid.iter_cols().flatten().collect_vec(),
            vec![
                ((0, 0), &1),
                ((0, 1), &4),
                ((0, 2), &7),
                ((1, 0), &2),
                ((1, 1), &5),
                ((1, 2), &8),
                ((2, 0), &3),
                ((2, 1), &6),
                ((2, 2), &9),
            ]
        );

        assert_eq!(
            grid.iter_rev_cols().flatten().collect_vec(),
            vec![
                ((0, 2), &7),
                ((0, 1), &4),
                ((0, 0), &1),
                ((1, 2), &8),
                ((1, 1), &5),
                ((1, 0), &2),
                ((2, 2), &9),
                ((2, 1), &6),
                ((2, 0), &3),
            ]
        );

        assert_eq!(
            grid.iter_rows().flatten().collect_vec(),
            vec![
                ((0, 0), &1),
                ((1, 0), &2),
                ((2, 0), &3),
                ((0, 1), &4),
                ((1, 1), &5),
                ((2, 1), &6),
                ((0, 2), &7),
                ((1, 2), &8),
                ((2, 2), &9),
            ]
        );

        assert_eq!(
            grid.iter_rev_rows().flatten().collect_vec(),
            vec![
                ((2, 0), &3),
                ((1, 0), &2),
                ((0, 0), &1),
                ((2, 1), &6),
                ((1, 1), &5),
                ((0, 1), &4),
                ((2, 2), &9),
                ((1, 2), &8),
                ((0, 2), &7),
            ]
        );

        assert_vec_eq_multiset(
            grid.enumerate_n4((1, 1)).collect(),
            vec![
                (Dir::Down, &2),
                (Dir::Left, &4),
                (Dir::Right, &6),
                (Dir::Up, &8),
            ],
        );

        assert_vec_eq_multiset(
            grid.enumerate_n4((1, 2)).collect(),
            vec![(Dir::Down, &5), (Dir::Left, &7), (Dir::Right, &9)],
        );

        assert_vec_eq_multiset(
            grid.enumerate_n4((2, 2)).collect(),
            vec![(Dir::Down, &6), (Dir::Left, &8)],
        );
    }
}
