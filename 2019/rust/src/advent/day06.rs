use crate::utils::errors::Error;
use crate::utils::files::{problem_input_path, read_file_split_whitespace};

use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Add;
use std::str::FromStr;

struct Orbit {
    center: String,
    satellite: String,
}

impl FromStr for Orbit {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<&str> = s.split(")").collect();
        Ok(Orbit {
            center: (*items.get(0).unwrap()).into(),
            satellite: (*items.get(1).unwrap()).into(),
        })
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Path {
    direct: u32,
    indirect: u32,
    ancestors: u32,
}

impl Path {
    fn incr(&self) -> Self {
        Path {
            direct: self.direct + 1,
            indirect: self.indirect + self.ancestors,
            ancestors: self.ancestors + 1,
        }
    }
}

impl Add for Path {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            direct: self.direct + other.direct,
            indirect: self.indirect + other.indirect,
            ancestors: self.ancestors + other.ancestors,
        }
    }
}

pub fn part_one() -> Result<u32, Error> {
    let input_path = problem_input_path(6, None);
    let orbit_pairs: Vec<Orbit> = read_file_split_whitespace(&input_path)?;
    let mut orbits = HashMap::new(); // K orbits V
    let mut orbited_by: HashMap<&str, HashSet<&str>> = HashMap::new(); // K is orbited by Vs
    let mut nodes = HashSet::new();
    let mut has_orbits = HashSet::new();
    for orbit in orbit_pairs.iter() {
        orbits.insert(&orbit.satellite, &orbit.center);
        orbited_by
            .entry(&orbit.center)
            .or_default()
            .insert(&orbit.satellite);
        nodes.insert(&orbit.satellite);
        nodes.insert(&orbit.center);
        has_orbits.insert(&orbit.center);
    }
    let mut explored = HashSet::new();
    // Our initial frontier is all nodes who have no satellites
    let mut frontier: VecDeque<_> = nodes.difference(&has_orbits).collect();
    let mut paths: HashMap<&String, Path> = HashMap::new();
    while !frontier.is_empty() {
        let visit = frontier.pop_front().unwrap();
        if explored.contains(visit.as_str()) {
            // We allow duplicates in the frontier, so ignore them
            continue;
        }
        if let Some(satellites) = orbited_by.get(visit.as_str()) {
            if let Some(_) = satellites.difference(&explored).next() {
                // Our current visit has satellites we have not yet visited.
                // Add to the back of our frontier and we'll revisit later.
                frontier.push_back(visit);
                continue;
            }
        }
        // Our current tracking of how far many direct (and indirect) jumps
        // we've made
        let path = paths.remove(visit).unwrap_or_default();
        if let Some(destination) = orbits.get(visit) {
            let existing_path = *paths.entry(destination).or_default();
            paths.insert(destination, existing_path + path.incr());
            frontier.push_back(destination);
            explored.insert(visit);
        } else {
            // The only location without an orbit is the COM
            return Ok(path.direct + path.indirect);
        }
    }
    Err(Error::NoSolutionFound)
}
