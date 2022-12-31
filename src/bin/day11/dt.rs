use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;
use std::{cmp, fmt, io};

use num::integer::lcm;

#[derive(Debug)]
enum Op {
    Add(u32),
    Mul(u32),
    Square,
}

impl Op {
    fn apply(&self, worry_level: u128, lcm: u64) -> u128 {
        // Clever trick: we're only concerned with remainders. So use the LCM
        // of all the monkey's divisors (and the relief factor), and use that to
        // restrict the range of possible values.
        let x = match self {
            Self::Add(n) => worry_level.checked_add((*n).into()),
            Self::Mul(n) => worry_level.checked_mul((*n).into()),
            Self::Square => worry_level.checked_mul(worry_level),
        };
        if x.is_none() {
            panic!(
                "arithmetic overflow: working with {worry_level} and {self:?}"
            );
        }
        x.unwrap() % (lcm as u128)
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "new = old {} {}",
            match self {
                Self::Square | Self::Mul(_) => '*',
                Self::Add(_) => '+',
            },
            match self {
                Self::Add(n) | Self::Mul(n) => n.to_string(),
                Self::Square => "old".to_string(),
            }
        )
    }
}

#[derive(Debug)]
struct Test {
    divisor: u8,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn get_throw(&self, val: u128) -> usize {
        match val % (self.divisor as u128) {
            0 => self.if_true,
            _ => self.if_false,
        }
    }
}

#[derive(Debug)]
pub struct Monkey {
    items: VecDeque<u128>,
    op: Op,
    test: Test,
    num_inspected: u32,
}

impl Monkey {
    fn fmt_indent(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Items: {:?}", self.items)?;
        writeln!(f, "  Operation: {}", self.op)?;
        writeln!(f, "  Test: divisible by {}", self.test.divisor)?;
        writeln!(f, "    If true: throw to monkey {}", self.test.if_true)?;
        writeln!(f, "    If false: throw to monkey {}", self.test.if_false)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct State {
    monkeys: Vec<Monkey>,
    relief_factor: u64,
    lcm: u64,
}

impl State {
    pub fn try_parse_from(
        mut lines: impl Iterator<Item = Result<String, io::Error>>,
        relief_factor: u64,
    ) -> Result<Self, Box<dyn Error>> {
        let mut ms: Vec<Monkey> = Vec::new();

        while let Some(line) = lines.next() {
            let line = line?;
            // This line should be "Monkey N"
            let n = scan_fmt!(line.as_str(), "Monkey {d}", usize)
                .or(Err(parse_err(r#"expected "Monkey [\d+]"#)))?;

            if n != ms.len() {
                return Err("wrong monkey number".into());
            }

            let line = lines
                .next()
                .ok_or_else(|| parse_err(r#"expected "Starting items""#))??;

            // This line should be "  Starting items: ..."
            let items =
                line.strip_prefix("  Starting items: ").ok_or_else(|| {
                    parse_err(r#"couldn't parse "Starting items""#)
                })?;

            let items: VecDeque<u128> = items
                .split(", ")
                .map(|s| s.parse::<u128>().unwrap())
                .collect();

            let line = lines
                .next()
                .ok_or_else(|| parse_err(r#"expected "Operation: ...""#))??;
            let line =
                line.strip_prefix("  Operation: new = old ").ok_or_else(
                    || parse_err("unable to parse Operation: bad prefix"),
                )?;

            let op = if let Some(rhs) = line.strip_prefix("* ") {
                if rhs == "old" {
                    Op::Square
                } else {
                    Op::Mul(rhs.parse()?)
                }
            } else if let Some(rhs) = line.strip_prefix("+ ") {
                Op::Add(rhs.parse()?)
            } else {
                return Err(parse_err("unable to parse Operation: bad suffix"));
            };

            let line = lines
                .next()
                .ok_or_else(|| parse_err(r#"expected "Test""#))??;

            let divisor = scan_fmt!(&line, "Test: divisible by {d}", u8)
                .or(Err(parse_err("unable to parse Test line")))?;

            let line = lines
                .next()
                .ok_or_else(|| parse_err(r#"expected "If true""#))??;

            let if_true =
                scan_fmt!(&line, "If true: throw to monkey {d}", usize)
                    .or(Err(parse_err("unable to parse If true line")))?;

            let line = lines
                .next()
                .ok_or_else(|| parse_err(r#"expected "If false""#))??;

            let if_false =
                scan_fmt!(&line, "If false: throw to monkey {d}", usize)
                    .or(Err(parse_err("unable to parse If false line")))?;

            ms.push(Monkey {
                items,
                op,
                test: Test {
                    divisor,
                    if_true,
                    if_false,
                },
                num_inspected: 0,
            });

            if let Some(l) = lines.next() {
                if l?.is_empty() {
                    return Err(parse_err("expected newline"));
                }
            }
        }

        // Check the throw values are all in range.
        let valid_range = 0..ms.len();
        for m in ms.iter() {
            if !valid_range.contains(&m.test.if_true)
                || !valid_range.contains(&m.test.if_false)
            {
                return Err(parse_err(
                    "found a if true/false index that was out of range",
                ));
            }
        }

        // Calculate the LCM.
        let lcm = ms
            .iter()
            .map(|m| m.test.divisor)
            .fold(relief_factor, |a, b| lcm(a, b as u64));

        Ok(State {
            monkeys: ms,
            relief_factor,
            lcm,
        })
    }

    pub fn step(&mut self) {
        for i in 0..self.monkeys.len() {
            while let Some(item) = self.monkeys[i].items.pop_front() {
                let this = &mut self.monkeys[i];
                this.num_inspected += 1;

                let worry_level = this.op.apply(item, self.lcm)
                    / (self.relief_factor as u128);
                let throw_to = this.test.get_throw(worry_level);
                self.monkeys[throw_to].items.push_back(worry_level);
            }
        }
    }

    pub fn print_items_thrown(&self) {
        for (i, m) in self.monkeys.iter().enumerate() {
            println!("Monkey {i} inspected items {} times", m.num_inspected);
        }
    }

    pub fn monkey_business_value(&self) -> u64 {
        let mut x = self
            .monkeys
            .iter()
            .map(|m| m.num_inspected)
            .collect::<Vec<u32>>();
        x.sort_unstable_by_key(|x| cmp::Reverse(*x));
        x.drain(..2)
            .fold(1, |a, b| a.checked_mul(b.into()).unwrap())
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (n, m) in self.monkeys.iter().enumerate() {
            writeln!(f, "Monkey {n}")?;
            m.fmt_indent(f)?;
            f.write_char('\n')?;
        }
        write!(f, "Group LCM: {}", self.lcm)
    }
}

fn parse_err(msg: &'static str) -> Box<dyn Error> {
    use aoc2022::utils::error::parse_error;

    Box::new(parse_error(msg))
}
