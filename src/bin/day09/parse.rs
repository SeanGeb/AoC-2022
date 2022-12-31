use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::{self, Write};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'u' => Ok(Self::Up),
            'd' => Ok(Self::Down),
            'l' => Ok(Self::Left),
            'r' => Ok(Self::Right),
            _ => Err("no direction for that char"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Movement(Direction, u32);

impl TryFrom<&str> for Movement {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (d, n) = scan_fmt!(value, "{} {}", char, u32)
            .or(Err("couldn't parse line"))?;
        let dir: Direction = d.try_into()?;
        Ok(Movement(dir, n))
    }
}

#[derive(Debug)]
pub struct State {
    pub rope: Vec<(i32, i32)>,
    visited: HashSet<(i32, i32)>,
    max_pos: (i32, i32),
    min_pos: (i32, i32),
}

impl State {
    pub fn new(len: usize) -> State {
        assert!(len >= 2, "len must be >= 2");
        assert!(len <= 10, "len must be <= 10");

        State {
            rope: vec![(0, 0); len],
            visited: HashSet::new(),
            max_pos: (0, 0),
            min_pos: (0, 0),
        }
    }

    pub fn do_move(&mut self, Movement(d, n): Movement) {
        for _ in 0..n {
            self.do_single_move(&d);
        }
    }

    pub fn do_single_move(&mut self, d: &Direction) {
        // Update head of rope
        {
            let (x, y) = self.rope[0];
            self.rope[0] = match d {
                Direction::Up => (x, y.checked_add(1).unwrap()),
                Direction::Down => (x, y.checked_sub(1).unwrap()),
                Direction::Right => (x.checked_add(1).unwrap(), y),
                Direction::Left => (x.checked_sub(1).unwrap(), y),
            };
        }

        // Update rest of rope
        for i in 1..self.rope.len() {
            let (x, y) = self.rope[i - 1];
            let (t_x, t_y) = self.rope[i];
            if !abuts_9(&(x, y), &(t_x, t_y)) {
                let (d_x, d_y) = (x - t_x, y - t_y);

                // Move up to one unit towards the knot we're trailing.
                self.rope[i] = (t_x + clamp_01(d_x), t_y + clamp_01(d_y));

                assert!(abuts_9(&self.rope[i], &self.rope[i - 1]));
            }
        }

        self.update_extremums();
        self.visited.insert(*self.rope.last().unwrap());
    }

    pub fn count_visited(&self) -> usize {
        self.visited.len()
    }

    fn update_extremums(&mut self) {
        for pos in self.rope.iter() {
            self.max_pos =
                (max(self.max_pos.0, pos.0), max(self.max_pos.1, pos.1));
            self.min_pos =
                (min(self.min_pos.0, pos.0), min(self.min_pos.1, pos.1));
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (self.min_pos.1..=self.max_pos.1).rev() {
            for x in self.min_pos.0..=self.max_pos.0 {
                let mut c =
                    match self.rope.iter().position(|pos| (x, y) == *pos) {
                        Some(0) => 'h',
                        Some(n) => {
                            assert!((1..=9).contains(&n));
                            format!("{n}").chars().next().unwrap()
                        },
                        None => match self.visited.contains(&(x, y)) {
                            true => '#',
                            false => '.',
                        },
                    };

                if self.visited.contains(&(x, y)) {
                    c.make_ascii_uppercase();
                }

                if (x, y) == (0, 0) && c == '#' {
                    c = 's';
                }

                f.write_char(c)?;
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

// Is b in the 3x3 squares centered at a?
fn abuts_9(a: &(i32, i32), b: &(i32, i32)) -> bool {
    let x_diff = (a.0 - b.0).abs();
    let y_diff = (a.1 - b.1).abs();
    x_diff <= 1 && y_diff <= 1
}

// Clamps a within the range [-1, 1]
fn clamp_01(a: i32) -> i32 {
    use std::cmp::Ordering::*;
    match a.cmp(&0) {
        Less => -1,
        Equal => 0,
        Greater => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abuts() {
        assert!(abuts_9(&(0, 0), &(0, 0)));
        assert!(abuts_9(&(1, 1), &(1, 0)));
        assert!(abuts_9(&(10, 10), &(10, 9)));
        assert!(abuts_9(&(10, 10), &(9, 11)));
        assert!(!abuts_9(&(10, 10), &(10, 12)));
        assert!(!abuts_9(&(10, 10), &(11, 12)));
    }

    #[test]
    fn test_move_2() {
        let mut s = State::new(2);
        println!("initial:\n{s}");
        s.do_move(Movement(Direction::Right, 4));
        s.do_move(Movement(Direction::Up, 4));
        s.do_move(Movement(Direction::Left, 3));
        s.do_move(Movement(Direction::Down, 1));
        s.do_move(Movement(Direction::Right, 4));
        s.do_move(Movement(Direction::Down, 1));
        s.do_move(Movement(Direction::Left, 5));
        s.do_move(Movement(Direction::Right, 2));
        println!("{s}");
        assert_eq!(s.count_visited(), 13);
    }

    #[test]
    fn test_move_10() {
        let mut s = State::new(10);
        println!("initial:\n{s}");
        s.do_move(Movement(Direction::Right, 5));
        s.do_move(Movement(Direction::Up, 8));
        s.do_move(Movement(Direction::Left, 8));
        s.do_move(Movement(Direction::Down, 3));
        s.do_move(Movement(Direction::Right, 17));
        s.do_move(Movement(Direction::Down, 10));
        s.do_move(Movement(Direction::Left, 25));
        s.do_move(Movement(Direction::Up, 20));
        println!("{s}");
        assert_eq!(s.count_visited(), 36);
    }
}
