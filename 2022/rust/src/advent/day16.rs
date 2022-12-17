use crate::utils::*;
use eyre::{Report, Result};
use std::{
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
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

pub fn part_two() -> Result<isize> {
    let input_path = problem_input_path(16, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_two_inner(&content)?;
    println!("{}", result);
    Ok(result)
}

fn part_one_inner(input: &str) -> Result<usize> {
    let valve_system: ValveSystem = str::parse(input)?;
    // println!("{:?}", valve_system);
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
        // let debug_me = path.actions.starts_with(&vec![
        //     Action::Move("DD".into()),
        //     Action::Open,
        //     Action::Move("CC".into()),
        //     Action::Move("BB".into()),
        //     Action::Open,
        //     Action::Move("AA".into()),
        //     Action::Move("II".into()),
        //     Action::Move("JJ".into()),
        //     Action::Open,
        //     Action::Move("II".into()),
        //     Action::Move("AA".into()),
        //     Action::Move("DD".into()),
        //     Action::Move("EE".into()),
        //     Action::Move("FF".into()),
        //     Action::Move("GG".into()),
        //     Action::Move("HH".into()),
        //     Action::Open,
        // ]);
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
                    // if debug_me {
                    //     println!("DEBUG: checking out {} for {:?}", route, path);
                    // }
                    let best_route = best_path_cache
                        .entry(route.clone())
                        .or_insert_with(|| {
                            best_path(&path.location, &target.name, &valve_system.connections)
                        })
                        .clone();
                    let time_taken = best_route.len();
                    let mut path_clone = path.clone();
                    // if route == "BBJJ" {
                    //     println!("BBJJ - {:?}", best_route)
                    // }
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
                        // if debug_me {
                        //     dbg!(path_clone.clone());
                        // }
                        frontier.push(path_clone.clone());
                    } else {
                        // println!(
                        //     "discarding {:?} because {} < {}",
                        //     path_clone, time_taken, time_remaining
                        // );
                    }
                    if path_clone.cumulative_release > max_released_path.cumulative_release {
                        max_released_path = path_clone;
                    }
                }
            }
        } else {
            // println!(
            //     "discarding {:?} because {} < {}",
            //     path, max_possible, max_released_path.cumulative_release
            // );
        }
    }
    println!("{}", max_released_path.actions.len());
    Ok(dbg!(max_released_path).cumulative_release)
}

fn part_two_inner(_input: &str) -> Result<isize> {
    todo!()
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
