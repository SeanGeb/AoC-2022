use std::collections::HashSet;
use std::fmt::Display;

use num::CheckedSub;

use crate::dt::*;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct State {
    pub ore_robots: u16,
    pub clay_robots: u16,
    pub obsidian_robots: u16,
    pub geode_robots: u16,
    pub resources: Resources,
}

impl State {
    /// Returns a copy of the current state, but adds in the provided resources.
    pub fn add_resources(&self, r: Resources) -> Self {
        Self {
            resources: self.resources + r,
            ..*self
        }
    }

    /// Determine what resources will be collected this turn.
    /// These resources will need to be added back on to self.resources.
    pub fn collect_resources(&self) -> Resources {
        Resources {
            ore: self.ore_robots,
            clay: self.clay_robots,
            obsidian: self.obsidian_robots,
            geodes: self.geode_robots,
        }
    }

    /// Returns a lower bound of the maximum number of geodes that this state
    /// can open in the provided number of minutes.
    pub fn get_lower_bound_geodes(&self, bp: &Blueprint, minutes: u16) -> u16 {
        // Assume we build as many geode robots as we have resources for, or
        // will mine resources for.
        let mut state = *self;

        // On each turn:
        // - If resources allow, start a new geode robot.
        // - Accumulate resources.
        for _ in 0..minutes {
            let resources_this_turn = state.collect_resources();
            if let Some(with_geode_robot) = state.try_build_geode_robot(bp) {
                state = with_geode_robot;
            }
            state = state.add_resources(resources_this_turn);
        }

        state.resources.geodes
    }

    /// Returns an upper bound of the maximu number of geodes that this state
    /// can open in the provided number of minutes.
    pub fn get_upper_bound_geodes(&self, bp: &Blueprint, minutes: u16) -> u16 {
        // Assume we can build a robot of each non-geode type on each turn
        // without consuming any resources to do so, creating an upper bound on
        // the resources we can accumulate each turn. Build a geode robot on any
        // turn on which resources allow.
        let mut state = *self;

        for _ in 0..minutes {
            let resources_this_turn = state.collect_resources();
            if let Some(with_geode_robot) = state.try_build_geode_robot(bp) {
                state = with_geode_robot;
            }
            state.ore_robots += 1;
            state.clay_robots += 1;
            state.obsidian_robots += 1;
            state = state.add_resources(resources_this_turn);
        }

        state.resources.geodes
    }

    pub fn try_build_ore_robot(&self, bp: &Blueprint) -> Option<Self> {
        Some(Self {
            ore_robots: self.ore_robots + 1,
            resources: self.resources.checked_sub(&bp.ore_robot_cost)?,
            ..*self
        })
    }

    pub fn try_build_clay_robot(&self, bp: &Blueprint) -> Option<Self> {
        Some(Self {
            clay_robots: self.clay_robots + 1,
            resources: self.resources.checked_sub(&bp.clay_robot_cost)?,
            ..*self
        })
    }

    pub fn try_build_obsidian_robot(&self, bp: &Blueprint) -> Option<Self> {
        Some(Self {
            obsidian_robots: self.obsidian_robots + 1,
            resources: self.resources.checked_sub(&bp.obsidian_robot_cost)?,
            ..*self
        })
    }

    pub fn try_build_geode_robot(&self, bp: &Blueprint) -> Option<Self> {
        Some(Self {
            geode_robots: self.geode_robots + 1,
            resources: self.resources.checked_sub(&bp.geode_robot_cost)?,
            ..*self
        })
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            resources: Default::default(),
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Resources:\n  {}", self.resources)?;
        writeln!(f, "There are:")?;
        writeln!(f, "  {:2} ore robots", self.ore_robots)?;
        writeln!(f, "  {:2} clay robots", self.clay_robots)?;
        writeln!(f, "  {:2} obsidian robots", self.obsidian_robots)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct StateSet {
    pub minute: u16,
    pub states: HashSet<State>,
}

impl Default for StateSet {
    fn default() -> Self {
        let mut states: HashSet<State> = HashSet::new();
        let new_state: State = Default::default();
        states.insert(new_state);
        Self { minute: 0, states }
    }
}

impl Display for StateSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "********************\nAfter minute {}\n********************",
            self.minute
        )?;

        for s in self.states.iter() {
            writeln!(f, "{s}\n\n")?;
        }

        Ok(())
    }
}

impl StateSet {
    /// Finds the StateSet for the given minute, from the given blueprint.
    pub fn get_at_time(bp: &Blueprint, minute: u16) -> Self {
        let mut states: Self = Default::default();
        while states.minute < minute {
            states = states.find_next_states(bp, minute);
            states.prune_futile_states(bp, minute);
        }
        states
    }

    /// Finds all the possible unique successor states from this set of states.
    pub fn find_next_states(&self, bp: &Blueprint, total_minutes: u16) -> Self {
        let minutes_remaining = (total_minutes - self.minute) - 1;

        // The process in the example is:
        // - Try to construct a robot, if desired.
        // - Collect resources.
        // - Finish construction.

        // We can simplify this to:
        // - Determine increase in resources and create an intermediate state
        //   with the new resources.
        // - From the old state, determine what actions are possible.
        // - Apply each of those actions to the intermediate state.

        // Possible actions are:
        // - Do nothing.
        // - Build a robot of each type.

        // Track the best lower bound so far for pruning.
        let mut next_states: HashSet<State> = HashSet::new();
        let mut best_lower_bound: u16 = 0;

        let mut add_next_state = |s: State| {
            let lb = s.get_lower_bound_geodes(bp, minutes_remaining);
            let ub = s.get_upper_bound_geodes(bp, minutes_remaining);

            // If we've found a new best lower bound, update that and remove
            // anything worse.
            // Possible enhancements: cache the upper/lower bounds for each
            // state and use a BTreeSet to prune results much faster.
            if lb > best_lower_bound {
                best_lower_bound = lb;
                next_states.retain(|s| {
                    s.get_upper_bound_geodes(bp, minutes_remaining)
                        > best_lower_bound
                        || s.get_upper_bound_geodes(bp, minutes_remaining)
                            == best_lower_bound
                            && s.get_lower_bound_geodes(bp, minutes_remaining)
                                == best_lower_bound
                });
            }

            if ub >= best_lower_bound
                || ub == best_lower_bound && lb == best_lower_bound
            {
                next_states.insert(s);
            }
        };

        for state in self.states.iter() {
            let resources_this_turn = state.collect_resources();

            // Provides the "do nothing" option.
            add_next_state(state.add_resources(resources_this_turn));

            if let Some(with_ore_robot) = state.try_build_ore_robot(bp) {
                add_next_state(
                    with_ore_robot.add_resources(resources_this_turn),
                );
            }

            if let Some(with_clay_robot) = state.try_build_clay_robot(bp) {
                add_next_state(
                    with_clay_robot.add_resources(resources_this_turn),
                );
            }

            if let Some(with_obsidian_robot) =
                state.try_build_obsidian_robot(bp)
            {
                add_next_state(
                    with_obsidian_robot.add_resources(resources_this_turn),
                );
            }

            if let Some(with_geode_robot) = state.try_build_geode_robot(bp) {
                add_next_state(
                    with_geode_robot.add_resources(resources_this_turn),
                );
            }
        }

        Self {
            minute: self.minute + 1,
            states: next_states,
        }
    }

    /// Removes all successor states that cannot possibly improve over an
    /// existing state.
    /// For each state, we determine the minimum final score (assume no more
    /// geode robots are made), and the maximum final score (assume every turn
    /// is used to produce a geode robot).
    /// A state cannot improve over another if they have non-overlapping ranges,
    /// so we drop all states with a maximum possible score that is strictly
    /// less than largest value of the minimum possible score.
    pub fn prune_futile_states(&mut self, bp: &Blueprint, until: u16) {
        let minutes_remaining = until - self.minute;
        if minutes_remaining == 0 {
            return;
        }

        let best_lower_bound = self
            .states
            .iter()
            .map(|s| s.get_lower_bound_geodes(bp, minutes_remaining))
            .max()
            .unwrap();

        // Only retain states with a strictly better upper bound, or an equal
        // lower bound and upper bound equal to the greatest lower bound.
        self.states.retain(|s| {
            s.get_upper_bound_geodes(bp, minutes_remaining) > best_lower_bound
                || s.get_upper_bound_geodes(bp, minutes_remaining)
                    == best_lower_bound
                    && s.get_lower_bound_geodes(bp, minutes_remaining)
                        == best_lower_bound
        });
    }

    /// Returns the maximum number of geodes that have been collected.
    pub fn get_max_geodes(&self) -> u16 {
        self.states
            .iter()
            .map(|s| s.resources.geodes)
            .max()
            .unwrap()
    }

    /// Returns the quality score.
    pub fn get_quality_score(&self, bp: &Blueprint) -> u16 {
        self.get_max_geodes() * bp.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_states_should_follow_example() {
        let bp = Blueprint {
            id: 1,
            ore_robot_cost: Resources {
                ore: 4,
                ..Default::default()
            },
            clay_robot_cost: Resources {
                ore: 2,
                ..Default::default()
            },
            obsidian_robot_cost: Resources {
                ore: 3,
                clay: 14,
                ..Default::default()
            },
            geode_robot_cost: Resources {
                ore: 2,
                obsidian: 7,
                ..Default::default()
            },
        };

        println!("{bp}");

        let mut states: StateSet = Default::default();

        const MINS: u16 = 24;

        while states.minute < 5 {
            states = states.find_next_states(&bp, MINS);
            println!(
                "minute {}: {} states",
                states.minute,
                states.states.len()
            );
            states.prune_futile_states(&bp, MINS);
            println!("  pruned down to {} states", states.states.len());
        }

        assert_eq!(states.minute, 5);
        assert!(states.states.contains(&State {
            ore_robots: 1,
            clay_robots: 2,
            obsidian_robots: 0,
            geode_robots: 0,
            resources: Resources {
                ore: 1,
                clay: 2,
                obsidian: 0,
                geodes: 0,
            },
        }));

        while states.minute < 10 {
            states = states.find_next_states(&bp, MINS);
            println!(
                "minute {}: {} states",
                states.minute,
                states.states.len()
            );
            states.prune_futile_states(&bp, MINS);
            println!("  pruned down to {} states", states.states.len());
        }

        assert_eq!(states.minute, 10);
        assert!(states.states.contains(&State {
            ore_robots: 1,
            clay_robots: 3,
            obsidian_robots: 0,
            geode_robots: 0,
            resources: Resources {
                ore: 4,
                clay: 15,
                obsidian: 0,
                geodes: 0,
            },
        }));

        while states.minute < MINS {
            states = states.find_next_states(&bp, MINS);
            println!(
                "minute {}: {} states",
                states.minute,
                states.states.len()
            );
            states.prune_futile_states(&bp, MINS);
            println!("  pruned down to {} states", states.states.len());
        }

        assert_eq!(states.minute, MINS);
        assert!(states.states.contains(&State {
            ore_robots: 1,
            clay_robots: 4,
            obsidian_robots: 2,
            geode_robots: 2,
            resources: Resources {
                ore: 6,
                clay: 41,
                obsidian: 8,
                geodes: 9,
            },
        }));
        assert_eq!(states.get_max_geodes(), 9);
        assert_eq!(states.get_quality_score(&bp), 9);
    }

    #[test]
    fn next_states_should_return_correct_score() {
        let bp1 = Blueprint {
            id: 1,
            ore_robot_cost: Resources {
                ore: 4,
                ..Default::default()
            },
            clay_robot_cost: Resources {
                ore: 2,
                ..Default::default()
            },
            obsidian_robot_cost: Resources {
                ore: 3,
                clay: 14,
                ..Default::default()
            },
            geode_robot_cost: Resources {
                ore: 2,
                obsidian: 7,
                ..Default::default()
            },
        };

        let bp1_state_set = StateSet::get_at_time(&bp1, 24);
        assert_eq!(bp1_state_set.get_max_geodes(), 9);
        assert_eq!(bp1_state_set.get_quality_score(&bp1), 9);

        let bp2 = Blueprint {
            id: 2,
            ore_robot_cost: Resources {
                ore: 2,
                ..Default::default()
            },
            clay_robot_cost: Resources {
                ore: 3,
                ..Default::default()
            },
            obsidian_robot_cost: Resources {
                ore: 3,
                clay: 8,
                ..Default::default()
            },
            geode_robot_cost: Resources {
                ore: 3,
                obsidian: 12,
                ..Default::default()
            },
        };

        let bp2_state_set = StateSet::get_at_time(&bp2, 24);
        assert_eq!(bp2_state_set.get_max_geodes(), 12);
        assert_eq!(bp2_state_set.get_quality_score(&bp2), 24);
    }
}
