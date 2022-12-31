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
    println!("{}", result);
    Ok(result)
}

pub fn part_two() -> Result<usize> {
    let input_path = problem_input_path(16, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_two_inner(&content)?;
    println!("{}", result);
    Ok(result)
}

fn part_one_inner(input: &str) -> Result<usize> {
    todo!()
}

fn part_two_inner(input: &str) -> Result<usize> {
    todo!()
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

#[derive(Debug, Clone, Eq)]
struct PathPair {
    first: Path,
    second: Path,
}

impl PathPair {
    fn released(&self) -> usize {
        self.first.released + self.second.released
    }
    fn heuristic(&self, compact_paths: &BTreeMap<String, usize>) -> usize {
        self.first.heuristic(compact_paths) + self.second.heuristic(compact_paths)
    }
}

impl PartialEq for PathPair {
    fn eq(&self, other: &Self) -> bool {
        self.released() == other.released()
    }
}

impl PartialOrd for PathPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.released().cmp(&other.released()))
    }
}

impl Ord for PathPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, Eq)]
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

#[derive(Debug, Clone, Derivative)]
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
        assert_eq!(part_two_inner(TEST_INPUT).unwrap(), 1707);
        assert_eq!(part_two().unwrap(), 2591);
    }
}
