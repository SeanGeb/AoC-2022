use core::fmt;
use std::error::Error;
use std::io;

use aoc2022::utils::error::{parse_error, ParseError};

#[derive(Debug, Clone, Copy)]
pub enum MoveType {
    Restack,
    Block,
}

#[derive(Debug)]
pub struct Move {
    from: usize,
    to: usize,
    count: usize,
    move_type: MoveType,
}

impl Move {
    pub fn parse_from_lines(
        lines: impl Iterator<Item = Result<String, io::Error>>,
        move_type: MoveType,
    ) -> impl Iterator<Item = Result<Move, Box<dyn Error>>> {
        // Parse "move X from Y to Z".
        lines
            .map(|l| {
                scan_fmt!(
                    l.unwrap().as_str(),
                    "move {d} from {d} to {d}",
                    usize,
                    usize,
                    usize
                )
            })
            .map(move |r| {
                let (count, from, to) = r?;
                let this_move = Move {
                    count,
                    from,
                    to,
                    move_type,
                };
                Ok(this_move)
            })
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Move: {} [{}]--> {}", self.from, self.count, self.to)
    }
}

#[derive(Debug)]
pub struct State {
    stacks: Vec<Vec<char>>,
}

impl State {
    fn new(stacks: Vec<Vec<char>>) -> Result<State, ParseError> {
        if stacks.is_empty() {
            return Err(parse_error("zero stacks provided"));
        }

        Ok(State { stacks })
    }

    pub fn new_from_lines(
        lines: &mut impl Iterator<Item = Result<String, io::Error>>,
    ) -> Result<State, Box<dyn Error>> {
        // Use the first line to determine the width of the stack.
        let first_line = match lines.next() {
            None => return Err(parse_error("no first line in input").into()),
            Some(s) => s?,
        };

        assert_eq!(
            (first_line.chars().count() + 1) % 4,
            0,
            "malformed first line (not 4n+3 chars)"
        );

        let num_stacks = (first_line.chars().count() + 1) / 4;
        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];
        let mut lines = [Ok(first_line)].into_iter().chain(lines);

        loop {
            // Parse the initial state columns.
            let line = match lines.next() {
                None => return Err(parse_error("start state ended early").into()),
                Some(l) => l?,
            };

            // Parsing hack: take every 4th char, skipping the first, and stop
            // when a digit is returned.
            let mut chars = line.chars().skip(1).peekable();
            match chars.peek() {
                Some('A'..='Z' | ' ') => chars
                    .step_by(4)
                    .zip(stacks.iter_mut())
                    .filter(|(c, _)| c != &' ')
                    .for_each(|(c, stack)| stack.push(c)),
                Some('0'..='9') => break,
                Some(c) => return Err(parse_error(format!("bad char {c:?}").as_str()).into()),
                None => return Err(parse_error("line too short").into()),
            };
        }

        // Discard the empty line.
        match lines.next() {
            Some(r) => match r?.as_str() {
                "" => Ok::<(), Box<dyn Error>>(()),
                l => Err(parse_error(format!("expected empty line, got {l}").as_str()).into()),
            },
            None => Err(parse_error("early end of file").into()),
        }?;

        // Reverse the order of the stacks to get 0..n in bottom..top ordering.
        stacks.iter_mut().for_each(|v| v.reverse());

        Ok(State::new(stacks)?)
    }

    pub fn apply_move(&mut self, m: &Move) {
        let (count, from, to) = (m.count, m.from, m.to);

        // Note: from/to are 1-indexed.
        assert_ne!(from, 0, "there is no 0th stack to move from");
        assert_ne!(to, 0, "there is no 0th stack to move to");

        assert!(
            from < self.stacks.len() + 1,
            "cannot move from stack {from} with only {} stacks",
            self.stacks.len()
        );
        assert!(
            to < self.stacks.len() + 1,
            "cannot move to {to} with only {} stacks",
            self.stacks.len()
        );

        // Remember that from/to were 1-indexed? Fix that.
        let from = from - 1;
        let to = to - 1;

        assert!(
            count <= self.stacks[from].len(),
            "cannot remove {count} items from stack of size {}",
            self.stacks[from].len()
        );

        match m.move_type {
            // Pop count items from the "from" stack and push to the "to" stack
            MoveType::Restack => {
                for _ in 0..count {
                    let val = self.stacks[from].pop().unwrap();
                    self.stacks[to].push(val);
                }
            }
            MoveType::Block => {
                let mut buf: Vec<char> = vec![];

                for _ in 0..count {
                    buf.push(self.stacks[from].pop().unwrap());
                }

                buf.reverse();

                self.stacks[to].append(&mut buf);
            }
        }
    }

    pub fn get_top_of_stacks(&self) -> Vec<Option<&char>> {
        self.stacks.iter().map(|s| s.last()).collect()
    }

    fn max_depth(&self) -> usize {
        self.stacks.iter().map(|s| s.len()).max().unwrap()
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let height = self.max_depth();

        // For an empty cell, print "    "
        // For a non-empty cell, print "[c] "
        // Work from index max_height-1 downwards.
        for i in (0..height).rev() {
            if let Some(err) = self
                .stacks
                .iter() // for each stack
                .map(|s| match s.get(i) {
                    Some(x) => format!("[{}]", x),
                    None => "    ".to_string(),
                }) // convert extant values to [c] form
                .map(|cell| write!(f, "{:<4}", cell)) // and write
                .find(fmt::Result::is_err)
            {
                // return the first of any errors that appeared
                return err;
            }

            writeln!(f)?;
        }

        // Then print " i  ", 1-indexed
        for i in 1..=self.stacks.len() {
            write!(f, "{i:^4}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use aoc2022::utils::file::get_input_lines;

    use super::*;

    #[test]
    fn test_state_new_from_lines() {
        let mut lines = get_input_lines("example/day05").unwrap();

        let s = State::new_from_lines(&mut lines);
        eprintln!("{}", s.unwrap());

        let ms = Move::parse_from_lines(lines, MoveType::Restack);
        ms.for_each(|m| eprintln!("{}", m.unwrap()));
    }

    #[test]
    fn test_state_new_display_top_of_stacks_apply_move() {
        let s = State::new([].into());
        assert!(s.is_err());

        let mut s = State::new(
            [
                ['A', 'B', 'C'].into(),
                ['D'].into(),
                ['E', 'F'].into(),
                [].into(),
                ['G'].into(),
            ]
            .into(),
        )
        .unwrap();
        assert_eq!(s.stacks.len(), 5);
        assert_eq!(s.stacks[0].len(), 3);
        assert_eq!(s.stacks[3].len(), 0);

        assert_eq!(
            format!("{s}"),
            concat!(
                "[C]                 \n",
                "[B]     [F]         \n",
                "[A] [D] [E]     [G] \n",
                " 1   2   3   4   5  ",
            )
            .to_string()
        );

        assert_eq!(
            s.get_top_of_stacks(),
            [Some(&'C'), Some(&'D'), Some(&'F'), None, Some(&'G')]
        );

        // Now try applying a move and check the stacks are OK.
        s.apply_move(&Move {
            count: 2,
            from: 1,
            to: 2,
            move_type: MoveType::Restack,
        });

        assert_eq!(
            s.stacks,
            [
                ['A'].to_vec(),
                ['D', 'C', 'B'].to_vec(),
                ['E', 'F'].to_vec(),
                [].to_vec(),
                ['G'].to_vec(),
            ]
        );

        s.apply_move(&Move {
            count: 2,
            from: 2,
            to: 1,
            move_type: MoveType::Block,
        });

        assert_eq!(
            s.stacks,
            [
                ['A', 'C', 'B'].to_vec(),
                ['D'].to_vec(),
                ['E', 'F'].to_vec(),
                [].to_vec(),
                ['G'].to_vec(),
            ]
        );
    }
}
