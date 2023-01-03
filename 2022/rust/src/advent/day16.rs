use crate::utils::*;
use derivative::Derivative;
use eyre::{Report, Result};
use itertools::Itertools;
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
    str::FromStr,
};

pub fn part_one() -> Result<usize> {
    let input_path = problem_input_path(16, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_one_inner(&content)?;
    println!("{result}");
    Ok(result)
}

pub fn part_two() -> Result<usize> {
    let input_path = problem_input_path(16, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_two_inner(&content)?;
    println!("{result}");
    Ok(result)
}

// this computes the best single path using an A* like technique
// it is not appropriate for Part 2 because we expect to take a suboptimal route for part 2
// the optimal route for Part 2 would be more like taking every other next best step, disjointly
// it would be faster if we switched to bitsets to reduce memory pressure/copying
fn part_one_inner(input: &str) -> Result<usize> {
    let valve_system: ValveSystem = str::parse(input)?;
    let compact_paths = compact(&valve_system.connections, &valve_system.flow_rates);
    let mut frontier = BinaryHeap::new();
    let initial = Path {
        location: "AA".to_owned(),
        time_remaining: 30,
        unvisited: valve_system.flow_rates,
        pressure: 0,
        released: 0,
    };
    let mut best = initial.clone();
    frontier.push(initial);
    while let Some(mut path) = frontier.pop() {
        if path.unvisited.is_empty() {
            path.released += path.pressure * path.time_remaining;
            path.time_remaining = 0;
            if path.released > best.released {
                best = path;
            }
            continue;
        }
        if path.released + path.heuristic(&compact_paths) < best.released {
            continue;
        }
        for target in path.unvisited.iter() {
            let route = format!("{}{}", path.location, target.name);
            let cost = compact_paths[&route];
            let mut path_clone = path.clone();
            path_clone.released += path_clone.pressure * usize::min(path.time_remaining, cost);
            if cost < path.time_remaining {
                path_clone.time_remaining -= cost;
                path_clone.pressure += target.pressure;
                path_clone.location = target.name.clone();
                path_clone.unvisited.remove(target);
                frontier.push(path_clone.clone());
            }
            if path_clone.released > best.released {
                best = path_clone;
            }
        }
    }
    Ok(best.released)
}

// for part 2, we need to compute all routes (at least of a certain quality)
// then find the two disjoint routes that perform best
fn part_two_inner(input: &str) -> Result<usize> {
    let valve_system: ValveSystem = str::parse(input)?;
    let compact_paths = compact(&valve_system.connections, &valve_system.flow_rates);
    let mut complete_paths: HashMap<BTreeSet<Valve>, usize> = HashMap::new();
    let mut frontier = VecDeque::new();
    let initial = Path {
        location: "AA".to_owned(),
        time_remaining: 26,
        unvisited: valve_system.flow_rates.clone(),
        pressure: 0,
        released: 0,
    };
    frontier.push_back(initial);
    // TODO: ugh, we don't prune the I[u] & state case which omits combinations we've explored before
    // thats why they store the max for any combination they've found - they don't care the path
    // TODO: optimize
    while let Some(path) = frontier.pop_front() {
        // TODO: add "if empty" branch so it works for the test input
        for target in path.unvisited.iter() {
            let route = format!("{}{}", path.location, target.name);
            let cost = compact_paths[&route];
            let mut path_clone = path.clone();
            path_clone.released += path_clone.pressure * usize::min(path.time_remaining, cost);
            if cost >= path_clone.time_remaining {
                let combination_best = complete_paths
                    .entry(
                        valve_system
                            .flow_rates
                            .difference(&path_clone.unvisited)
                            .cloned()
                            .collect(),
                    )
                    .or_default();
                if *combination_best < path_clone.released {
                    *combination_best = path_clone.released;
                }
            } else {
                path_clone.time_remaining -= cost;
                path_clone.pressure += target.pressure;
                path_clone.location = target.name.clone();
                path_clone.unvisited.remove(target);
                frontier.push_back(path_clone);
            }
        }
    }
    let mut best = 0;
    for (human_path, human_value) in complete_paths.iter() {
        for (elephant_path, elephant_value) in complete_paths.iter() {
            let released = human_value + elephant_value;
            if released > best && human_path.is_disjoint(elephant_path) {
                best = released;
            }
        }
    }
    Ok(best)
}

fn compact(
    connections: &HashMap<String, Vec<String>>,
    targets: &BTreeSet<Valve>,
) -> BTreeMap<String, usize> {
    let mut compact = BTreeMap::new();
    let starting_location = "AA".to_owned();
    for target in targets.iter() {
        let best_path = best_path(&starting_location, &target.name, connections);
        compact.insert(
            format!("{}{}", starting_location, target.name),
            best_path.len(),
        );
    }
    for pair in targets.iter().permutations(2) {
        let [from, to]: [&Valve; 2] = pair.try_into().unwrap();
        let best_path = best_path(&from.name, &to.name, connections);
        compact.insert(format!("{}{}", from.name, to.name), best_path.len());
    }
    compact
}

fn best_path(
    from: &String,
    to: &String,
    connections: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut frontier = VecDeque::new();
    seen.insert(from);
    frontier.push_back(vec![from]);
    while let Some(mut path) = frontier.pop_front() {
        let last = path.last().unwrap();
        if let Some(edges) = connections.get(*last) {
            for neighbor in edges.iter() {
                if neighbor == to {
                    path.push(to);
                    return path.into_iter().cloned().collect();
                } else if seen.insert(neighbor) {
                    let mut path_clone = path.clone();
                    path_clone.push(neighbor);
                    frontier.push_back(path_clone);
                }
            }
        }
    }
    unreachable!()
}

#[allow(clippy::derive_hash_xor_eq)]
#[derive(Debug, Clone, Eq, Hash)]
struct Path {
    location: String,
    time_remaining: usize,
    unvisited: BTreeSet<Valve>,
    pressure: usize,
    released: usize,
}

impl Path {
    fn heuristic(&self, compact_paths: &BTreeMap<String, usize>) -> usize {
        let project_past = self.pressure * self.time_remaining;
        if self.unvisited.is_empty() {
            project_past + self.pressure * self.time_remaining
        } else {
            let realistic_start =
                format!("{}{}", self.location, self.unvisited.first().unwrap().name);
            let starting_cost = compact_paths[&realistic_start];
            let (time_used, optimistic_future): (usize, usize) = self
                .unvisited
                .iter()
                .rev()
                .enumerate()
                .map(|(idx, valve)| {
                    let time_used = if idx > 0 { idx } else { starting_cost };
                    let time = self
                        .time_remaining
                        .saturating_sub(2 * (idx + starting_cost));
                    if time > 0 {
                        (time_used, time * valve.pressure)
                    } else {
                        (0, 0)
                    }
                })
                .reduce(|(time_used, opt_future), (t, v)| (time_used + t, opt_future + v))
                .unwrap();
            project_past
                + optimistic_future
                + self.time_remaining.saturating_sub(time_used) * self.pressure
        }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.released == other.released
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.released.cmp(&other.released))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[allow(clippy::derive_hash_xor_eq)]
#[derive(Debug, Clone, Derivative, Hash)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
struct Valve {
    #[derivative(PartialEq = "ignore")]
    #[derivative(PartialOrd = "ignore")]
    #[derivative(Ord = "ignore")]
    name: String,
    pressure: usize,
}

#[derive(Debug)]
struct ValveSystem {
    flow_rates: BTreeSet<Valve>,
    connections: HashMap<String, Vec<String>>,
}

impl FromStr for ValveSystem {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut flow_rates = BTreeSet::new();
        let mut connections = HashMap::new();
        for line in s.trim().lines() {
            let (valve, line) = line
                .strip_prefix("Valve ")
                .unwrap()
                .split_once(' ')
                .unwrap();
            let (flow_rate, line) = line
                .strip_prefix("has flow rate=")
                .unwrap()
                .split_once(';')
                .unwrap();
            let (_, line) = line.split_once("valve").unwrap();
            let other: Vec<String> = line
                .trim_start_matches('s')
                .split(',')
                .map(|v| v.trim().to_owned())
                .collect();
            let flow_rate = str::parse(flow_rate)?;
            if flow_rate > 0 {
                flow_rates.insert(Valve {
                    name: valve.to_owned(),
                    pressure: flow_rate,
                });
            }
            connections.insert(valve.into(), other);
        }
        Ok(ValveSystem {
            flow_rates,
            connections,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one_inner(TEST_INPUT).unwrap(), 1651);
        assert_eq!(part_one().unwrap(), 2087);
    }

    #[test]
    fn part_two_works() {
        // See TODO above. The real input cannot visit all valves so I omitted the empty target case.
        // But the test input is easy to visit all so the solver just returns the best single route.
        assert_eq!(part_two_inner(TEST_INPUT).unwrap(), 1707);
        assert_eq!(part_two().unwrap(), 2591);
    }
}
