use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Write};

pub enum Instruction {
    Noop,
    AddX(i32),
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "noop" {
            Ok(Self::Noop)
        } else if let Ok(add_value) = scan_fmt!(value, "addx {d}", i32) {
            Ok(Self::AddX(add_value))
        } else {
            Err("value provided not a noop or addx")
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Noop => f.write_str("noop"),
            Self::AddX(x) => write!(f, "addx {x}"),
        }
    }
}

#[derive(Debug)]
pub struct Machine {
    x: i32,
    time: u32,
    // A map of cycle number -> value after that cycle completes.
    val_by_time: BTreeMap<u32, i32>,
}

impl Machine {
    pub fn new() -> Self {
        let mut m = Machine {
            x: 1,
            time: 1,
            val_by_time: BTreeMap::new(),
        };
        m.record_time(); // ensures we record value during cycle
        m
    }

    pub fn exec(&mut self, instr: Instruction) {
        match instr {
            Instruction::Noop => self.noop(),
            Instruction::AddX(x) => self.addx(x),
        };
        self.record_time();
    }

    pub fn get_at(&self, when: u32) -> Option<i32> {
        self.val_by_time.get(&when).copied()
    }

    fn noop(&mut self) {
        self.time_tick(1);
    }

    fn addx(&mut self, by: i32) {
        self.time_tick(2);
        self.x += by;
    }

    fn time_tick(&mut self, ticks: u32) {
        for _ in 0..ticks {
            self.record_time();
            self.time += 1;
        }
    }

    fn record_time(&mut self) {
        self.val_by_time.insert(self.time, self.x);
    }

    pub fn get_part1_score(&self) -> Result<i32, Box<dyn Error>> {
        let mut s = 0;
        for cycle in [20, 60, 100, 140, 180, 220] {
            match self.get_at(cycle) {
                Some(v) => {
                    s += v
                        .checked_mul(cycle.try_into()?)
                        .ok_or::<&'static str>("mul out of range".into())?
                },
                None => return Err("simulation didn't run long enough".into()),
            };
        }
        Ok(s)
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (cycle, x) in self.val_by_time.iter() {
            let (cycle, x) = (cycle.to_owned(), x.to_owned());
            let pos: i32 = ((cycle.checked_sub(1).unwrap()) % 40).try_into().unwrap();

            f.write_char(if ((pos - 1)..=(pos + 1)).contains(&x) {
                '#'
            } else {
                '.'
            })?;

            if pos == 39 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use aoc2022::utils::file::get_input_lines;

    use crate::parse::parse_lines;

    use super::*;

    #[test]
    fn test_machine() {
        let mut machine = Machine::new();
        let lines = get_input_lines("example/day10").unwrap();
        parse_lines(lines).for_each(|instr| machine.exec(instr.unwrap()));
        println!("{machine:?}");

        assert_eq!(machine.get_at(20), Some(21));
        assert_eq!(machine.get_at(60), Some(19));
        assert_eq!(machine.get_at(100), Some(18));
        assert_eq!(machine.get_at(140), Some(21));
        assert_eq!(machine.get_at(180), Some(16));
        assert_eq!(machine.get_at(220), Some(18));
    }
}
