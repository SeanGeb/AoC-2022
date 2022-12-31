use std::error::Error;
use std::fmt::Display;
use std::ops::*;

use aoc2022::min;
use aoc2022::utils::file::get_input_lines;
use num::CheckedSub;
use scan_fmt::parse::ScanError;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Resources {
    pub ore: u16,
    pub clay: u16,
    pub obsidian: u16,
    pub geodes: u16,
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geodes: self.geodes + rhs.geodes,
        }
    }
}

impl Sub for Resources {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore - rhs.ore,
            clay: self.ore - rhs.clay,
            obsidian: self.ore - rhs.obsidian,
            geodes: self.geodes - rhs.geodes,
        }
    }
}

impl CheckedSub for Resources {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        Some(Self {
            ore: self.ore.checked_sub(v.ore)?,
            clay: self.clay.checked_sub(v.clay)?,
            obsidian: self.obsidian.checked_sub(v.obsidian)?,
            geodes: self.geodes.checked_sub(v.geodes)?,
        })
    }
}

impl Div for Resources {
    type Output = u16;

    /// Div of two Resources returns the lowest common divisor, setting any
    /// division-by-zero results to the max value. Panics if it would return a
    /// division-by-zero result.
    fn div(self, rhs: Self) -> Self::Output {
        let val = min!(
            self.ore.checked_div(rhs.ore).unwrap_or(Self::Output::MAX),
            self.clay.checked_div(rhs.clay).unwrap_or(Self::Output::MAX),
            self.obsidian
                .checked_div(rhs.obsidian)
                .unwrap_or(Self::Output::MAX),
            self.geodes
                .checked_div(rhs.geodes)
                .unwrap_or(Self::Output::MAX)
        );
        assert_ne!(val, Self::Output::MAX);
        val
    }
}

impl Display for Resources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:2} ore, {:2} clay, {:2} obsidian",
            self.ore, self.clay, self.obsidian
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Blueprint {
    pub id: u16,
    pub ore_robot_cost: Resources,
    pub clay_robot_cost: Resources,
    pub obsidian_robot_cost: Resources,
    pub geode_robot_cost: Resources,
}

impl Display for Blueprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Blueprint {}:", self.id)?;
        writeln!(f, "  Ore robot:      {}", self.ore_robot_cost)?;
        writeln!(f, "  Clay robot:     {}", self.clay_robot_cost)?;
        writeln!(f, "  Obsidian robot: {}", self.obsidian_robot_cost)?;
        writeln!(f, "  Geode robot:    {}", self.geode_robot_cost)
    }
}

pub struct Blueprints(pub Vec<Blueprint>);

impl Display for Blueprints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bp in self.0.iter() {
            bp.fmt(f)?;
        }
        Ok(())
    }
}

impl TryFrom<&str> for Blueprint {
    type Error = ScanError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (
            id,
            ore_robot_ore,
            clay_robot_ore,
            obsidian_robot_ore,
            obisidan_robot_clay,
            geode_robot_ore,
            geode_robot_obsidian,
        ) = scan_fmt!(
            value,
            concat!(
                "Blueprint {d}: ",
                "Each ore robot costs {d} ore. ",
                "Each clay robot costs {d} ore. ",
                "Each obsidian robot costs {d} ore and {d} clay. ",
                "Each geode robot costs {d} ore and {d} obsidian"
            ),
            u16,
            u16,
            u16,
            u16,
            u16,
            u16,
            u16
        )?;

        Ok(Self {
            id,
            ore_robot_cost: Resources {
                ore: ore_robot_ore,
                ..Default::default()
            },
            clay_robot_cost: Resources {
                ore: clay_robot_ore,
                ..Default::default()
            },
            obsidian_robot_cost: Resources {
                clay: obisidan_robot_clay,
                ore: obsidian_robot_ore,
                ..Default::default()
            },
            geode_robot_cost: Resources {
                ore: geode_robot_ore,
                obsidian: geode_robot_obsidian,
                ..Default::default()
            },
        })
    }
}

pub fn get_blueprints(file: &str) -> Result<Blueprints, Box<dyn Error>> {
    let lines = get_input_lines(file)?;

    let mut bps: Vec<Blueprint> = Vec::new();
    for line in lines {
        let line = line?;
        bps.push(line.as_str().try_into()?);
    }

    Ok(Blueprints(bps))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_sub_for_resources() {
        let r1 = Resources {
            ore: 10,
            clay: 20,
            obsidian: 30,
            geodes: 0,
        };

        assert_eq!(
            r1.checked_sub(&Resources {
                ore: 1,
                clay: 2,
                obsidian: 3,
                geodes: 0,
            }),
            Some(Resources {
                ore: 9,
                clay: 18,
                obsidian: 27,
                geodes: 0,
            })
        );

        assert_eq!(
            r1.checked_sub(&Resources {
                ore: 10,
                clay: 20,
                obsidian: 30,
                geodes: 0,
            }),
            Some(Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geodes: 0,
            })
        );

        assert!(r1
            .checked_sub(&Resources {
                ore: 11,
                clay: 0,
                obsidian: 0,
                geodes: 0,
            })
            .is_none(),);
    }

    #[test]
    fn div_for_resources() {
        let l = Resources {
            ore: 10,
            clay: 5,
            obsidian: 3,
            geodes: 0,
        };

        assert_eq!(
            l / Resources {
                ore: 2,
                clay: 1,
                obsidian: 0,
                geodes: 0,
            },
            5
        );
    }
}
