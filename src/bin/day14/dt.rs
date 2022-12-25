use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Display, Write};

use aoc2022::{max, min};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Cell {
    Rock,
    Sand,
}

#[derive(Debug)]
pub struct State {
    map: HashMap<(u32, u32), Cell>,
    source: (u32, u32),
    min: (u32, u32),
    max: (u32, u32),
}

impl State {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            source: (500, 0),
            min: (500, 0),
            max: (500, 0),
        }
    }

    /// Adds a unit of sand. Returns true iff the sand was placed inside the
    /// grid (i.e. didn't fall out of the min/max bounds).
    fn add_sand(&mut self) -> bool {
        let mut pos = self.source;
        while self.in_bounds(pos) {
            pos = if self.map.get(&(pos.0, pos.1 + 1)).is_none() {
                (pos.0, pos.1 + 1)
            } else if self.map.get(&(pos.0 - 1, pos.1 + 1)).is_none() {
                (pos.0 - 1, pos.1 + 1)
            } else if self.map.get(&(pos.0 + 1, pos.1 + 1)).is_none() {
                (pos.0 + 1, pos.1 + 1)
            } else if self.map.get(&pos).is_none() {
                self.map.insert(pos, Cell::Sand);
                return true;
            } else {
                assert_eq!(pos, self.source);
                return false;
            }
        }

        false
    }

    /// Adds sand until stable and returns the number of units of sand added.
    pub fn add_sand_to_stable(&mut self) -> u32 {
        let mut sand_added = 0;
        while self.add_sand() {
            sand_added += 1;
        }
        sand_added
    }

    pub fn draw_rock(&mut self, from: (u32, u32), to: (u32, u32)) {
        if from.0 == to.0 {
            let x = from.0;
            for y in min(from.1, to.1)..=max(from.1, to.1) {
                let pos = (x, y);
                assert_ne!(pos, self.source, "tried to overwrite source");
                if let Some(old) = self.map.insert(pos, Cell::Rock) {
                    assert_eq!(old, Cell::Rock, "overwrote sand with rock");
                }
            }
        } else if from.1 == to.1 {
            let y = from.1;
            for x in min(from.0, to.0)..=max(from.0, to.0) {
                let pos = (x, y);
                assert_ne!(pos, self.source, "tried to overwrite source");
                if let Some(old) = self.map.insert((x, y), Cell::Rock) {
                    assert_eq!(old, Cell::Rock, "overwrote sand with rock");
                }
            }
        } else {
            panic!("tried to draw a diagonal line {from:?} -> {to:?}");
        }

        self.min = (
            min!(self.min.0, from.0, to.0),
            min!(self.min.1, from.1, to.1),
        );
        self.max = (
            max!(self.max.0, from.0, to.0),
            max!(self.max.1, from.1, to.1),
        );
    }

    pub fn draw_floor(&mut self) {
        self.draw_rock(
            (self.min.0 - self.max.1, self.max.1 + 2),
            (self.max.0 + self.max.1, self.max.1 + 2),
        );
    }

    fn in_bounds(&self, pos: (u32, u32)) -> bool {
        let x_range = self.min.0..=self.max.0;
        let y_range = self.min.1..=self.max.1;
        x_range.contains(&pos.0) && y_range.contains(&pos.1)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let y_digits = self.max.1.to_string().len();
        for y in self.min.1..=self.max.1 {
            write!(f, "{:>width$} ", y, width = y_digits)?;
            for x in self.min.0..=self.max.0 {
                f.write_char(if self.source == (x, y) {
                    '+'
                } else {
                    match self.map.get(&(x, y)) {
                        None => '.',
                        Some(Cell::Rock) => '#',
                        Some(Cell::Sand) => 'o',
                    }
                })?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_sand() {
        let mut s = State::new();
        s.draw_rock((498, 4), (498, 6));
        s.draw_rock((498, 6), (496, 6));
        s.draw_rock((503, 4), (502, 4));
        s.draw_rock((502, 4), (502, 9));
        s.draw_rock((502, 9), (494, 9));
        println!("{s}");

        let mut sand_added = 0;
        while s.add_sand() {
            println!("{s}");
            sand_added += 1;
        }
        println!("{s}\nadded {sand_added} units of sand");
    }
}
