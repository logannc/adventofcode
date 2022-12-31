use crate::utils::*;
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

struct BetterPath {
    location: String,
    time_remaining: usize,
    unvisited: BTreeSet<Valve>,
    pressure: usize,
    released: usize,
}

impl BetterPath {
    fn heuristic(&self, paths: &BTreeMap<String, usize>) -> usize {
        let project_past = self.pressure * self.time_remaining;
        todo!()
    }
}

fn _score_path(mut time: isize, actions: &Vec<String>, costs: &BTreeMap<String, usize>) -> isize {
    let mut pressure = 0;
    let mut released = 0;
    for action in actions.iter() {
        let cost = costs[action];
        if (time - cost as isize) < 0 {
            break;
        }
        released += pressure * cost as isize;
        pressure += 
    }
    released += pressure * time;
    released
}

fn part_one_inner(input: &str) -> Result<usize> {
    let valve_system: ValveSystem = str::parse(input)?;
    let compact_paths = compact(&valve_system.connections, &valve_system.flow_rates);
    // let mut frontier = BinaryHeap::new();
    todo!()
}

fn _part_one_inner(input: &str) -> Result<usize> {
    let valve_system: ValveSystem = str::parse(input)?;
    let mut best_path_cache: HashMap<String, Vec<String>> = HashMap::new();
    let mut frontier = BinaryHeap::new();
    let mut max_released_path = Path {
        location: "AA".to_owned(),
        actions: vec![],
        targets: valve_system.flow_rates.clone(),
        pressure: 0,
        cumulative_release: 0,
    };
    frontier.push(max_released_path.clone());
    while let Some(mut path) = frontier.pop() {
        let time_remaining = 30 - path.actions.len();
        let max_possible =
            path.cumulative_release + heuristic(path.pressure, time_remaining, &path.targets);
        if max_possible > max_released_path.cumulative_release {
            if path.targets.is_empty() {
                path.cumulative_release += path.pressure * time_remaining;
                if path.cumulative_release > max_released_path.cumulative_release {
                    max_released_path = path;
                }
            } else {
                for target in path.targets.iter().rev() {
                    let route = format!("{}{}", path.location, target.name);
                    let best_route = best_path_cache
                        .entry(route.clone())
                        .or_insert_with(|| {
                            best_path(&path.location, &target.name, &valve_system.connections)
                        })
                        .clone();
                    let time_taken = best_route.len();
                    let mut path_clone = path.clone();
                    path_clone.cumulative_release +=
                        path_clone.pressure * usize::min(time_remaining, time_taken);
                    if time_taken <= time_remaining {
                        for stop in best_route.into_iter().skip(1) {
                            path_clone.actions.push(Action::Move(stop));
                        }
                        path_clone.actions.push(Action::Open);
                        path_clone.pressure += target.pressure;
                        path_clone.location = target.name.clone();
                        path_clone.targets.remove(target);

                        frontier.push(path_clone.clone());
                    }
                    if path_clone.cumulative_release > max_released_path.cumulative_release {
                        max_released_path = path_clone;
                    }
                }
            }
        }
    }
    Ok(max_released_path.cumulative_release)
}

#[derive(Debug, Clone, Eq)]
struct PathPair {
    first: Path,
    second: Path,
}

impl PathPair {
    fn pressure(&self) -> usize {
        self.first.pressure + self.second.pressure
    }
}

impl PartialEq for PathPair {
    fn eq(&self, other: &Self) -> bool {
        self.pressure() == other.pressure()
    }
}

impl PartialOrd for PathPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.pressure().cmp(&other.pressure()))
    }
}

impl Ord for PathPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn part_two_inner(input: &str) -> Result<usize> {
    let valve_system: ValveSystem = str::parse(input)?;
    let mut best_path_cache: HashMap<String, Vec<String>> = HashMap::new();
    let mut frontier = BinaryHeap::new();
    let max_path = Path {
        location: "AA".to_owned(),
        actions: vec![],
        targets: valve_system.flow_rates.clone(),
        pressure: 0,
        cumulative_release: 0,
    };
    let mut max_path_pair = PathPair {
        first: max_path.clone(),
        second: max_path,
    };
    frontier.push(max_path_pair.clone());
    while let Some(path_pair) = frontier.pop() {
        let PathPair {
            mut first,
            mut second,
        } = path_pair;
        let first_remaining = 26 - first.actions.len();
        let first_max_possible =
            first.cumulative_release + heuristic(first.pressure, first_remaining, &first.targets);
        let second_remaining = 26 - second.actions.len();
        let second_max_possible = second.cumulative_release
            + heuristic(second.pressure, second_remaining, &second.targets);
        let max_possible = first_max_possible + second_max_possible;
        let max_so_far =
            max_path_pair.first.cumulative_release + max_path_pair.second.cumulative_release;
        if max_possible < max_so_far {
            continue;
        }
        println!(
            "{}, {}, {}",
            first.targets.len(),
            second.targets.len(),
            frontier.len()
        );
        if first.targets.is_empty() && second.targets.is_empty() {
            first.cumulative_release += first.pressure * first_remaining;
            second.cumulative_release += second.pressure * second_remaining;
            if first.cumulative_release + second.cumulative_release > max_so_far {
                max_path_pair = PathPair { first, second }
            }
        } else {
            for target in first.targets.iter().rev() {
                let route = format!("{}{}", first.location, target.name);
                let first_best_route = best_path_cache
                    .entry(route.clone())
                    .or_insert_with(|| {
                        best_path(&first.location, &target.name, &valve_system.connections)
                    })
                    .clone();
                let first_time_taken = first_best_route.len();
                let mut first_clone = first.clone();
                first_clone.cumulative_release +=
                    first_clone.pressure * usize::min(first_remaining, first_time_taken);
                if first_time_taken <= first_remaining {
                    for stop in first_best_route.into_iter().skip(1) {
                        first_clone.actions.push(Action::Move(stop));
                    }
                    first_clone.actions.push(Action::Open);
                    first_clone.pressure += target.pressure;
                    first_clone.location = target.name.clone();
                    first_clone.targets.remove(target);
                    let mut second_clone = second.clone();
                    second_clone.targets.remove(target);
                    if second_clone.targets.is_empty() {
                        frontier.push(PathPair {
                            first: first_clone,
                            second: second_clone,
                        });
                    } else {
                        for target in second_clone.targets.iter().rev() {
                            let route = format!("{}{}", second_clone.location, target.name);
                            let second_best_route = best_path_cache
                                .entry(route.clone())
                                .or_insert_with(|| {
                                    best_path(
                                        &second_clone.location,
                                        &target.name,
                                        &valve_system.connections,
                                    )
                                })
                                .clone();
                            let second_time_taken = second_best_route.len();
                            let mut first_second_clone = first_clone.clone();
                            let mut second_second_clone = second_clone.clone();
                            second_second_clone.cumulative_release += second_clone.pressure
                                * usize::min(second_remaining, second_time_taken);
                            if second_time_taken <= second_remaining {
                                for stop in second_best_route.into_iter().skip(1) {
                                    second_second_clone.actions.push(Action::Move(stop));
                                }
                                second_second_clone.actions.push(Action::Open);
                                second_second_clone.pressure += target.pressure;
                                second_second_clone.location = target.name.clone();
                                second_second_clone.targets.remove(target);
                                first_second_clone.targets.remove(target);
                                frontier.push(PathPair {
                                    first: first_second_clone.clone(),
                                    second: second_second_clone.clone(),
                                });
                            }
                            if first_second_clone.cumulative_release
                                + second_second_clone.cumulative_release
                                > max_path_pair.first.cumulative_release
                                    + max_path_pair.second.cumulative_release
                            {
                                max_path_pair = PathPair {
                                    first: first_second_clone,
                                    second: second_second_clone,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", max_path_pair);
    Ok(max_path_pair.first.cumulative_release + max_path_pair.second.cumulative_release)
}

fn heuristic(
    current_pressure: usize,
    time_remaining: usize,
    remaining_targets: &BTreeSet<Valve>,
) -> usize {
    let project_past_targets = current_pressure * time_remaining;
    let optimistic_future_targets: usize = remaining_targets
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, valve)| {
            let time = time_remaining as isize - 2 * (idx as isize + 1);
            if time >= 0 {
                time as usize * valve.pressure
            } else {
                0
            }
        })
        .sum();
    project_past_targets + optimistic_future_targets
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Action {
    Move(String),
    Open,
}

#[derive(Eq, Debug, Clone)]
struct Path {
    location: String,
    actions: Vec<Action>,
    targets: BTreeSet<Valve>,
    pressure: usize,
    cumulative_release: usize,
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cumulative_release
            .partial_cmp(&other.cumulative_release)
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.cumulative_release == other.cumulative_release
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Eq, Debug, Clone)]
struct Valve {
    name: String,
    pressure: usize,
}

impl PartialOrd for Valve {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Ord::cmp(&self.pressure, &other.pressure))
    }
}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.pressure == other.pressure
    }
}

impl Ord for Valve {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
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
        assert_eq!(part_two().unwrap(), 0);
    }
}
