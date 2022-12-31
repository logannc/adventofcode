use crate::utils::*;
use eyre::{Report, Result};
use rayon::prelude::*;
use std::{
    fs,
    ops::{Add, Mul, Sub},
    str::FromStr,
};

pub fn part_one() -> Result<usize> {
    let input_path = problem_input_path(19, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_one_inner(&content)?;
    println!("{result}");
    Ok(result)
}

pub fn part_two() -> Result<usize> {
    let input_path = problem_input_path(19, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_two_inner(&content)?;
    println!("{result}");
    Ok(result)
}

fn part_one_inner(input: &str) -> Result<usize> {
    let blueprints: Result<Vec<Blueprint>, _> =
        input.trim().lines().map(str::parse::<Blueprint>).collect();
    Ok(blueprints?
        .into_par_iter()
        // .into_iter()
        .map(|b| b.quality_score())
        .sum())
}

fn part_two_inner(input: &str) -> Result<usize> {
    let blueprints: Result<Vec<Blueprint>, _> = input
        .trim()
        .lines()
        .map(str::parse::<Blueprint>)
        .take(3)
        .collect();
    Ok(blueprints?
        // .into_par_iter()
        .into_iter()
        .map(|b| b.geodes_harvested(32))
        .product())
}

#[derive(Default, Debug, Clone, Copy)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Add for Resources {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Resources {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Sub for Resources {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Resources {
            ore: self.ore.saturating_sub(rhs.ore),
            clay: self.clay.saturating_sub(rhs.clay),
            obsidian: self.obsidian.saturating_sub(rhs.obsidian),
            geode: self.geode.saturating_sub(rhs.geode),
        }
    }
}

impl Mul<usize> for Resources {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Resources {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
            geode: self.geode * rhs,
        }
    }
}

#[derive(Default, Debug, Clone)]
struct State {
    time_remaining: usize,
    materials: Resources,
    bots: Resources,
}
impl State {
    fn max(self, other: State) -> State {
        if self.materials.geode > other.materials.geode {
            self
        } else {
            other
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_bot_cost: Resources,
    clay_bot_cost: Resources,
    obsidian_bot_cost: Resources,
    geode_bot_cost: Resources,
    max_resource_cost: Resources,
}

impl Blueprint {
    fn quality_score(&self) -> usize {
        self.id * self.geodes_harvested(24)
    }

    fn geodes_harvested(&self, time: usize) -> usize {
        self.dfs(
            State {
                time_remaining: time,
                materials: Resources::default(),
                bots: Resources {
                    ore: 1,
                    ..Default::default()
                },
            },
            0,
        )
        .materials
        .geode
    }

    fn dfs(&self, state: State, mut best: usize) -> State {
        if state.time_remaining == 0 {
            state
        } else {
            let mut max = State::default();
            for choice in self.choices(&state) {
                if self.bound(&choice) > best {
                    let result = self.dfs(choice, best);
                    best = best.max(result.materials.geode);
                    max = max.max(result);
                }
            }
            max
        }
    }

    fn bound(&self, state: &State) -> usize {
        // imperative loop *doubles* the time!? (~750us to 1.5ms)
        // let (mut obsidian, mut geodes, mut obsidian_bots) = (
        //     state.materials.obsidian,
        //     state.materials.geode + state.bots.geode * state.time_remaining,
        //     state.bots.obsidian,
        // );
        // for time_left in (0..state.time_remaining).rev() {
        //     if obsidian >= self.geode_bot_cost.obsidian {
        //         obsidian += self.geode_bot_cost.obsidian + obsidian_bots;
        //         geodes += time_left;
        //     } else {
        //         obsidian += obsidian_bots;
        //         obsidian_bots += 1;
        //     }
        // }
        state.bots.geode * state.time_remaining
            + (0..state.time_remaining)
                .rev()
                .fold(
                    (
                        state.materials.obsidian,
                        state.bots.obsidian,
                        state.materials.geode,
                    ),
                    |(obsidian, obsidian_rate, geodes), time_left| {
                        if obsidian >= self.geode_bot_cost.obsidian {
                            (
                                obsidian - self.geode_bot_cost.obsidian + obsidian_rate,
                                obsidian_rate,
                                geodes + time_left,
                            )
                        } else {
                            (obsidian + obsidian_rate, obsidian_rate + 1, geodes)
                        }
                    },
                )
                .2
        // geodes
    }

    fn choices(&self, state: &State) -> impl Iterator<Item = State> {
        let states = vec![
            self.build_ore(state),
            self.build_clay(state),
            self.build_obsidian(state),
            self.build_geode(state),
        ];
        states.into_iter().flatten()
    }

    fn build_ore(&self, state: &State) -> Option<State> {
        if state.bots.ore < self.max_resource_cost.ore {
            let time_to_build = self
                .ore_bot_cost
                .ore
                .saturating_sub(state.materials.ore)
                .div_ceil(state.bots.ore)
                + 1;
            let materials = state.materials + state.bots * time_to_build.min(state.time_remaining)
                - self.ore_bot_cost;
            let mut bots = state.bots;
            bots.ore += 1;
            Some(State {
                time_remaining: state.time_remaining.saturating_sub(time_to_build),
                materials,
                bots,
            })
        } else {
            None
        }
    }

    fn build_clay(&self, state: &State) -> Option<State> {
        if state.bots.clay < self.max_resource_cost.clay {
            let time_to_build = self
                .clay_bot_cost
                .ore
                .saturating_sub(state.materials.ore)
                .div_ceil(state.bots.ore)
                + 1;
            let materials = state.materials + state.bots * time_to_build.min(state.time_remaining)
                - self.clay_bot_cost;
            let mut bots = state.bots;
            bots.clay += 1;
            Some(State {
                time_remaining: state.time_remaining.saturating_sub(time_to_build),
                materials,
                bots,
            })
        } else {
            None
        }
    }

    fn build_obsidian(&self, state: &State) -> Option<State> {
        if 0 < state.bots.clay && state.bots.obsidian < self.max_resource_cost.obsidian {
            let time_to_gather_ore = self
                .obsidian_bot_cost
                .ore
                .saturating_sub(state.materials.ore)
                .div_ceil(state.bots.ore);
            let time_to_gather_clay = self
                .obsidian_bot_cost
                .clay
                .saturating_sub(state.materials.clay)
                .div_ceil(state.bots.clay);
            let time_to_build = time_to_gather_ore.max(time_to_gather_clay) + 1;
            let materials = state.materials + state.bots * time_to_build.min(state.time_remaining)
                - self.obsidian_bot_cost;
            let mut bots = state.bots;
            bots.obsidian += 1;
            Some(State {
                time_remaining: state.time_remaining.saturating_sub(time_to_build),
                materials,
                bots,
            })
        } else {
            None
        }
    }

    fn build_geode(&self, state: &State) -> Option<State> {
        if 0 < state.bots.obsidian {
            let time_to_gather_ore = self
                .geode_bot_cost
                .ore
                .saturating_sub(state.materials.ore)
                .div_ceil(state.bots.ore);
            let time_to_gather_obsidian = self
                .geode_bot_cost
                .obsidian
                .saturating_sub(state.materials.obsidian)
                .div_ceil(state.bots.obsidian);
            let time_to_build = time_to_gather_ore.max(time_to_gather_obsidian) + 1;
            let materials = state.materials + state.bots * time_to_build.min(state.time_remaining)
                - self.geode_bot_cost;
            let mut bots = state.bots;
            bots.geode += 1;
            Some(State {
                time_remaining: state.time_remaining.saturating_sub(time_to_build),
                materials,
                bots,
            })
        } else {
            None
        }
    }
}

impl FromStr for Blueprint {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id_str, rest) = s.trim().split_once(':').unwrap();
        let (_, id_str) = id_str.split_once(' ').unwrap();
        let id = str::parse(id_str).unwrap();
        let values: Vec<usize> = rest
            .trim()
            .split_ascii_whitespace()
            .filter_map(|maybe_number| str::parse::<usize>(maybe_number).ok())
            .collect();
        let [ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian]: [usize;
            6] = values.try_into().unwrap();
        Ok(Blueprint {
            id,
            ore_bot_cost: Resources {
                ore: ore_ore,
                ..Default::default()
            },
            clay_bot_cost: Resources {
                ore: clay_ore,
                ..Default::default()
            },
            obsidian_bot_cost: Resources {
                ore: obsidian_ore,
                clay: obsidian_clay,
                ..Default::default()
            },
            geode_bot_cost: Resources {
                ore: geode_ore,
                obsidian: geode_obsidian,
                ..Default::default()
            },
            max_resource_cost: Resources {
                ore: clay_ore.max(obsidian_ore).max(geode_ore),
                clay: obsidian_clay,
                obsidian: geode_obsidian,
                geode: 0,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
"#;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one_inner(TEST_INPUT).unwrap(), 33);
        assert_eq!(part_one().unwrap(), 1480);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two_inner(TEST_INPUT).unwrap(), 56 * 62);
        assert_eq!(part_two().unwrap(), 3168);
    }
}
