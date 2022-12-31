use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Display;
use std::io;

use aoc2022::utils::parse::Parser;

#[derive(Debug)]
pub struct Valve {
    pub rate: u32,
    pub tunnels_to: Vec<String>,
}

// State combines Valves with the information necessary to retrieve the valve
// from its name.
pub struct State {
    pub valves: BTreeMap<String, Valve>,
}

impl State {
    pub fn parse(
        lines: impl Iterator<Item = Result<String, io::Error>>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut valves: BTreeMap<String, Valve> = BTreeMap::new();

        for line in lines {
            let line = line?;
            let mut p: Parser = line.as_str().into();

            p.str("Valve ");
            let name = p.read_n(2);

            p.str(" has flow rate=");
            let rate = p.u32();

            p.str("; tunnel");
            p.optional_char('s');
            p.str(" lead");
            p.optional_char('s');
            p.str(" to valve");
            p.optional_char('s');
            p.str(" ");

            let mut tunnels_to: Vec<String> = Vec::new();
            while !p.is_empty() {
                tunnels_to.push(p.read_n(2));
                p.optional_char(',').then(|| p.optional_char(' '));
            }

            valves.insert(name, Valve { rate, tunnels_to });
        }

        // Assert that tunnels are symmetric: if A is reachable from B, B must
        // be reachable from A.
        for (from_name, from_valve) in valves.iter() {
            for to_name in from_valve.tunnels_to.iter() {
                let to_valve = &valves[to_name];
                assert!(to_valve.tunnels_to.contains(from_name));
            }
        }

        Ok(Self { valves })
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, valve) in self.valves.iter() {
            write!(f, "{name}: {} -> ", valve.rate)?;
            for t in valve.tunnels_to.iter() {
                write!(f, "{t} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
