use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Write};

// Width of the column.
const WIDTH: usize = 7;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Jet {
    Left,
    Right,
}

// Creates a Vec<Jet> from a puzzle input.
pub fn jets_from(s: &str) -> Vec<Jet> {
    s.chars()
        .map(|c| -> Jet { c.try_into().unwrap() })
        .collect()
}

impl Display for Jet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Jet::Left => '<',
            Jet::Right => '>',
        })
    }
}

impl TryFrom<char> for Jet {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err("provided a bad direction"),
        }
    }
}

// The part of the puzzle state that gets hashed into the memo_table.
// - grid: compacted state of the column.
// - rock_idx: the index of the next rock type in ROCK_ORDER.
// - iter_idx: the index of the next jet to hit the falling rock.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct HashableState {
    grid: VecDeque<u8>,
    rock_idx: u8,
    iter_idx: u16,
}

// The part of the puzzle state memoized against HashableState. Contains the
// total height of rocks in the column, and the number of rocks dropped so far.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HashableStateResult {
    pub total_height: u64,
    pub rock_count: u64,
}

#[derive(Debug)]
pub struct State {
    hs: HashableState,
    compacted_height: u64,
    memo_table: HashMap<HashableState, HashableStateResult>,
}

impl State {
    pub fn new() -> Self {
        Self {
            hs: HashableState {
                grid: VecDeque::new(),
                rock_idx: 0,
                iter_idx: 0,
            },
            compacted_height: 0,
            memo_table: HashMap::new(),
        }
    }

    pub fn height(&self) -> usize {
        self.hs.grid.len() + self.compacted_height as usize
    }

    // Compaction identifies the first row where the cells are all inaccessible:
    // there is no path (using left/right/down movements) from the top of the
    // stack to an unoccupied cell.
    pub fn compact(&mut self) {
        const ALL_ONES: u8 = (1 << WIDTH) - 1;
        let mut accessible_rows = ALL_ONES;

        let mut remove_rows_below = 0;

        for (i, row) in self.hs.grid.iter().enumerate().rev() {
            // Turn row into a bitmap of empty cells, excluding cells that were
            // inaccessible in the last iteration.
            let cells_open = !row & accessible_rows;
            // Shift this left and right and bitwise or back into itself, which
            // simulates the fact a rock can move one cell left or right per
            // unit moved down, or be blocked and descend straight.
            let cells_open = cells_open | (cells_open << 1) | (cells_open >> 1);
            // Then exclude cells containing rock in the current row.
            accessible_rows = cells_open & !row & ALL_ONES;

            // If none of the cells in this row are accessible, we can delete
            // this row and all the accumulated rocks beneath it.
            if accessible_rows == 0 {
                remove_rows_below = i + 1;
                break;
            }
        }

        if remove_rows_below != 0 {
            // Perform the compaction by deleting from the bottom upwards, and
            // add the number of rows deleted for later accounting.
            self.compacted_height += self.hs.grid.drain(0..remove_rows_below).count() as u64;
        }
    }

    // Drops the next rock and memoizes the result, over and over again, until
    // it detects a cycle.
    pub fn drop_rocks_memo(
        &mut self,
        jets: &Vec<Jet>,
    ) -> (HashableStateResult, HashableStateResult) {
        for rock_count in 1u64.. {
            self.drop_next_rock(jets);
            self.compact();
            let hsr = HashableStateResult {
                total_height: self.height() as u64,
                rock_count,
            };
            if let Some(cycle_hsr) = self.memo_table.insert(self.hs.clone(), hsr) {
                return (cycle_hsr, hsr);
            }
        }
        unreachable!();
    }

    // Drops the next rock in this state.
    pub fn drop_next_rock(&mut self, jets: &Vec<Jet>) {
        let rock = ROCK_ORDER[self.hs.rock_idx as usize];
        self.drop_rock(rock, jets);
        self.hs.rock_idx = ((self.hs.rock_idx + 1) as usize % ROCK_ORDER.len())
            .try_into()
            .unwrap();
    }

    // Drops the provided rock from the top of the chamber.
    fn drop_rock(&mut self, rock: Rock, jets: &Vec<Jet>) {
        let mut pos: (usize, usize) = (2, self.hs.grid.len() + 3);
        loop {
            let jet = jets[self.hs.iter_idx as usize];
            self.hs.iter_idx =
                (self.hs.iter_idx + 1) % (<usize as TryInto<u16>>::try_into(jets.len()).unwrap());

            let next_x = match jet {
                Jet::Left => match pos.0.checked_sub(1) {
                    Some(n) => n,
                    None => 0,
                },
                Jet::Right => (pos.0 + 1).clamp(0, WIDTH - 1),
            };

            // Only move per the jet if this wouldn't intersect.
            if !rock.intersects_with(self, (next_x, pos.1)) {
                pos.0 = next_x;
            }

            // If we've reached the bottom, settle.
            if pos.1 == 0 {
                break;
            }

            // Otherwise, work out what happens if we move down.
            let next_y = pos.1 - 1;
            if rock.intersects_with(self, (pos.0, next_y)) {
                break;
            }

            pos.1 = next_y;
        }

        // Allocate enough height. height_required is the intended len of the
        // column Vec.
        let height_required = pos.1 + rock.height();
        for _ in self.hs.grid.len()..height_required {
            self.hs.grid.push_back(0);
        }

        // Settle the rock by setting bits in the bitmap.
        for delta_pos in rock.pos_iter() {
            let (x, y) = (pos.0 + delta_pos.0, pos.1 + delta_pos.1);
            let bitmask = 1 << x;
            assert_eq!(
                self.hs.grid[y] & bitmask,
                0,
                "tried to settle in occupied cell"
            );
            self.hs.grid[y] |= bitmask;
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.hs.grid.iter().rev() {
            f.write_char('|')?;
            for x in 0..7 {
                let cell = (row & (1 << x)) != 0;
                f.write_char(match cell {
                    true => '#',
                    false => '.',
                })?;
            }
            f.write_str("|\n")?;
        }

        f.write_char('+')?;
        for _ in 0..WIDTH {
            f.write_char('-')?;
        }
        f.write_str("+\n")
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Rock {
    Wide,
    Plus,
    Corner,
    Tall,
    Square,
}

pub const ROCK_ORDER: [Rock; 5] = [
    Rock::Wide,
    Rock::Plus,
    Rock::Corner,
    Rock::Tall,
    Rock::Square,
];

impl Rock {
    // Returns the height of this rock.
    pub fn height(&self) -> usize {
        match self {
            Rock::Wide => 1,
            Rock::Square => 2,
            Rock::Plus | Rock::Corner => 3,
            Rock::Tall => 4,
        }
    }

    // Checks if this rock intersects with existing rocks in the State at the
    // given position.
    // The position defines the bottom-left corner (smallest x, smallest y).
    pub fn intersects_with(&self, s: &State, pos: (usize, usize)) -> bool {
        for test_pos in self.pos_iter() {
            let (x, y) = (pos.0 + test_pos.0, pos.1 + test_pos.1);
            if x >= WIDTH {
                // Would end up positioning rock outside column.
                return true;
            }

            // If we've not created that row, there's nothing to collide with.
            if let Some(row) = s.hs.grid.get(y) {
                if row & (1 << x) != 0 {
                    return true;
                }
            }
        }

        false
    }

    // Returns the position of each part of the rock, measured from the bottom-
    // left corner of the rock.
    fn pos_iter(&self) -> impl Iterator<Item = (usize, usize)> {
        let positions: Vec<(usize, usize)> = match self {
            Rock::Wide => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Rock::Plus => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            Rock::Corner => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Rock::Tall => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Rock::Square => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        };
        positions.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_rock() {
        let mut s = State::new();
        let jet_iter = jets_from(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");

        for _ in 0..10 {
            s.drop_next_rock(&jet_iter);
        }

        assert_eq!(
            format!("{s}"),
            concat!(
                "|....#..|\n",
                "|....#..|\n",
                "|....##.|\n",
                "|##..##.|\n",
                "|######.|\n",
                "|.###...|\n",
                "|..#....|\n",
                "|.####..|\n",
                "|....##.|\n",
                "|....##.|\n",
                "|....#..|\n",
                "|..#.#..|\n",
                "|..#.#..|\n",
                "|#####..|\n",
                "|..###..|\n",
                "|...#...|\n",
                "|..####.|\n",
                "+-------+\n",
            )
        );

        for _ in 0..10 {
            s.drop_next_rock(&jet_iter);
        }

        let old_height = s.height();

        s.compact();
        assert_eq!(
            format!("{s}"),
            concat!(
                "|....#..|\n",
                "|....#..|\n",
                "|....#..|\n",
                "|....#..|\n",
                "|.##.#..|\n",
                "|.##.#..|\n",
                "|..###..|\n",
                "|...#...|\n",
                "|..###..|\n",
                "|...#...|\n",
                "|..####.|\n",
                "|.....##|\n",
                "|.....##|\n",
                "|......#|\n",
                "|......#|\n",
                "|...####|\n",
                "|..###..|\n",
                "|...#...|\n",
                "|#..####|\n",
                "|#...#..|\n",
                "|#...#..|\n",
                "|#...##.|\n",
                "|##..##.|\n",
                "+-------+\n"
            )
        );

        assert_eq!(s.height(), old_height);

        for _ in 0..6 {
            s.drop_rock(Rock::Wide, &jet_iter);
        }

        let old_height = s.height();
        s.compact();

        assert_eq!(
            format!("{s}"),
            concat!(
                "|####...|\n",
                "|..####.|\n",
                "|..####.|\n",
                "|..####.|\n",
                "+-------+\n",
            )
        );
        assert_eq!(s.height(), old_height);
    }
}
