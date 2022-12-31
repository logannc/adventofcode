use crate::utils::*;
use core::num;
use eyre::{Report, Result};
use rayon::prelude::*;
use std::{
    fs,
    ops::{Add, Mul},
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
        .map(|mut b| dbg!(b.quality_score()))
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
        .into_par_iter()
        .map(|mut b| dbg!(b.geodes_harvested(32)))
        .product())
}

#[derive(Default, Debug, Clone)]
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

struct State {
    materials: Resources,
    bots: Resources,
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore: Resources,
    clay: Resources,
    obsidian: Resources,
    geode: Resources,
    max: Resources,
}

// #[cached]
fn bound(
    time_remaining: usize,
    obsidian: usize,
    geode: usize,
    geode_cost: usize,
    num_obsidian: usize,
) -> usize {
    (0..time_remaining)
        .rev()
        .fold(
            (obsidian, num_obsidian, geode),
            |(obsidian, obsidian_rate, geodes), time_left| {
                if obsidian >= geode_cost {
                    (
                        obsidian - geode_cost + obsidian_rate,
                        obsidian_rate,
                        geodes + time_left,
                    )
                } else {
                    (obsidian + obsidian_rate, obsidian_rate + 1, geodes)
                }
            },
        )
        .2
}

impl Blueprint {
    fn quality_score(&mut self) -> usize {
        self.id * self.geodes_harvested(24)
    }

    fn geodes_harvested(&mut self, time: usize) -> usize {
        self.geodes_dfs(time, Resources::default(), 1, 0, 0, 0, 0)
            .geode
    }

    fn geodes_dfs(
        &mut self,
        time_remaining: usize,
        resources: Resources,
        num_ore: usize,
        num_clay: usize,
        num_obsidian: usize,
        num_geode: usize,
        mut best: usize,
    ) -> Resources {
        // TODO: is this too pessimistic because it doesn't use num_geode?
        if time_remaining == 0
            || bound(
                time_remaining,
                resources.obsidian,
                resources.geode,
                self.geode.obsidian,
                num_obsidian,
            ) < best
        {
            resources
        } else {
            let mut max = Resources::default();
            for (mut left_resources, ore_delta, clay_delta, obsidian_delta, geode_delta) in self
                .build_options(&resources, num_ore, num_clay, num_obsidian)
                .rev()
            {
                left_resources.ore += num_ore;
                left_resources.clay += num_clay;
                left_resources.obsidian += num_obsidian;
                left_resources.geode += geode_delta * (time_remaining - 1);
                // left_resources.geode += num_geode;
                let best_including_self = max.geode.max(left_resources.geode);
                let result = self.geodes_dfs(
                    time_remaining - 1,
                    left_resources,
                    num_ore + ore_delta,
                    num_clay + clay_delta,
                    num_obsidian + obsidian_delta,
                    num_geode + geode_delta,
                    best_including_self,
                );
                if result.geode > best {
                    best = result.geode;
                }
                if result.geode > max.geode {
                    max = result;
                }
            }
            max
        }
    }

    fn build_options(
        &self,
        resources: &Resources,
        num_ore: usize,
        num_clay: usize,
        num_obsidian: usize,
    ) -> impl DoubleEndedIterator<Item = (Resources, usize, usize, usize, usize)> {
        if let Some(remaining) = self.build_geode(resources) {
            return vec![(remaining, 0, 0, 0, 1)].into_iter();
        }
        let mut options = vec![(resources.clone(), 0, 0, 0, 0)];
        if let Some(remaining) = self.build_ore(resources, num_ore) {
            options.push((remaining, 1, 0, 0, 0));
        }
        if let Some(remaining) = self.build_clay(resources, num_clay) {
            options.push((remaining, 0, 1, 0, 0));
        }
        if let Some(remaining) = self.build_obsidian(resources, num_obsidian) {
            options.push((remaining, 0, 0, 1, 0));
        }
        options.into_iter()
    }

    fn build_ore(&self, resources: &Resources, num_ore: usize) -> Option<Resources> {
        // Option<(Resources, usize)> {
        if num_ore < self.max.ore {
            let time_to_accrue = self.ore.ore.saturating_sub(resources.ore).div_ceil(num_ore);
        }
        if resources.ore >= self.ore.ore && num_ore < self.max.ore {
            let mut remaining = resources.clone();
            remaining.ore -= self.ore.ore;
            Some(remaining)
        } else {
            None
        }
    }

    fn build_clay(&self, resources: &Resources, num_clay: usize) -> Option<Resources> {
        if resources.ore >= self.clay.ore && num_clay < self.max.clay {
            let mut remaining = resources.clone();
            remaining.ore -= self.clay.ore;
            Some(remaining)
        } else {
            None
        }
    }

    fn build_obsidian(&self, resources: &Resources, num_obsidian: usize) -> Option<Resources> {
        if resources.ore >= self.obsidian.ore
            && resources.clay >= self.obsidian.clay
            && num_obsidian < self.max.obsidian
        {
            let mut remaining = resources.clone();
            remaining.ore -= self.obsidian.ore;
            remaining.clay -= self.obsidian.clay;
            Some(remaining)
        } else {
            None
        }
    }

    fn build_geode(&self, resources: &Resources) -> Option<Resources> {
        if resources.ore >= self.geode.ore && resources.obsidian >= self.geode.obsidian {
            let mut remaining = resources.clone();
            remaining.ore -= self.geode.ore;
            remaining.obsidian -= self.geode.obsidian;
            Some(remaining)
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
            ore: Resources {
                ore: ore_ore,
                ..Default::default()
            },
            clay: Resources {
                ore: clay_ore,
                ..Default::default()
            },
            obsidian: Resources {
                ore: obsidian_ore,
                clay: obsidian_clay,
                ..Default::default()
            },
            geode: Resources {
                ore: geode_ore,
                obsidian: geode_obsidian,
                ..Default::default()
            },
            max: Resources {
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
        // assert_eq!(part_one().unwrap(), 1480);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two_inner(TEST_INPUT).unwrap(), 56 * 62);
        // assert_eq!(part_two().unwrap(), 0);
    }
}
