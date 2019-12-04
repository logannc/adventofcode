use crate::utils::errors::Error;
use crate::utils::files::{problem_input_path, read_file_split_whitespace};

use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy)]
enum DirectionVector {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl DirectionVector {
    fn parse(s: &str) -> Result<Self, Error> {
        Ok(match s.trim().split_at(1) {
            ("U", n) => Self::Up(str::parse::<u32>(n)?),
            ("D", n) => Self::Down(str::parse::<u32>(n)?),
            ("L", n) => Self::Left(str::parse::<u32>(n)?),
            ("R", n) => Self::Right(str::parse::<u32>(n)?),
            _ => return Err(Error::DirectionParseError(s.to_owned())),
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_mag = self.from_origin();
        let other_mag = other.from_origin();
        self_mag.cmp(&other_mag)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Point {
    fn from_origin(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
    fn travel(&self, to: DirectionVector) -> Point {
        match to {
            DirectionVector::Up(n) => Point {
                x: self.x,
                y: self.y + n as i32,
            },
            DirectionVector::Down(n) => Point {
                x: self.x,
                y: self.y - n as i32,
            },
            DirectionVector::Left(n) => Point {
                x: self.x - n as i32,
                y: self.y,
            },
            DirectionVector::Right(n) => Point {
                x: self.x + n as i32,
                y: self.y,
            },
        }
    }
}

/// These LineSegments will be, by construction, vertical or horizontal
/// segments. We will make use of that fact in various implementations.
/// We also assume that given line segments will not overlap in parallel
/// except for at a single point. THIS MIGHT BE FALSE!
#[derive(Debug, Clone, Copy)]
struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    fn normalized(&self) -> Self {
        if self.start.x > self.end.x || self.start.y > self.end.y {
            LineSegment {
                start: self.end,
                end: self.start,
            }
        } else {
            *self
        }
    }
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
    fn is_horizontal(&self) -> bool {
        !self.is_vertical()
    }
    fn intersects(&self, other: &Self) -> Option<Point> {
        let (a, b) = (self.normalized(), other.normalized());
        if a.is_vertical() && b.is_vertical() {
            if a.start.x != b.start.x {
                // If they are both vertical and not on the same X value,
                // they are parallel and cannot overlap.
                return None;
            }
            if a.start == b.start || a.start == b.end {
                // adjacent segments at the 'start'
                return Some(a.start);
            }
            if a.end == b.start || a.end == b.end {
                // adjacent segments at the 'end'
                return Some(a.end);
            }
            // hopefully there are no 'overlapping' lines
            return None;
        }
        // same block as above but for horizontal lines
        if a.is_horizontal() && b.is_horizontal() {
            if a.start.y != b.start.y {
                return None;
            }
            if a.start == b.start || a.start == b.end {
                return Some(a.start);
            }
            if a.end == b.start || a.end == b.end {
                return Some(a.end);
            }
            return None;
        }
        // we know they are different orientations, lets normalize the pair so
        let (a, b) = if a.start.x == a.end.x { (a, b) } else { (b, a) };
        // a is vertical,
        // b is horizontal
        if b.start.x <= a.start.x
            && a.start.x <= b.end.x
            && a.start.y <= b.start.y
            && b.start.y <= a.end.y
        {
            Some(Point {
                x: a.start.x,
                y: b.start.y,
            })
        } else {
            None
        }
    }
    fn length(&self) -> u32 {
        ((self.end.y - self.start.y).abs() + (self.end.x - self.start.x).abs()) as u32
    }
}

#[derive(Default, Debug)]
struct SparseLineBoard {
    lines: Vec<LineSegment>,
}

impl SparseLineBoard {
    fn bulk_travel(&mut self, directions: Vec<DirectionVector>) {
        for vector in directions.into_iter() {
            self.travel(vector);
        }
    }
    fn travel(&mut self, to: DirectionVector) {
        let source = if let Some(p) = self.lines.last() {
            p.end
        } else {
            Point::default()
        };
        let destination = source.travel(to);
        self.lines.push(LineSegment {
            start: source,
            end: destination,
        });
    }
    fn intersections(&self, line: &LineSegment) -> Vec<Point> {
        self.lines
            .iter()
            .flat_map(|part| part.intersects(line))
            .collect()
    }
    fn first_intersection(&self, line: &LineSegment) -> Option<(u32, Point)> {
        let mut traveled = 0;
        for segment in self.lines.iter() {
            if let Some(i) = segment.intersects(line) {
                let partial = LineSegment{start: segment.start, end: i}.length();
                return Some((traveled + partial, i));
            }
            traveled += segment.length();
        }
        None
    }
}

fn get_wires() -> Result<(Vec<DirectionVector>, Vec<DirectionVector>), Error> {
    let input_path = problem_input_path(3, None);
    let wires: Vec<String> = read_file_split_whitespace(&input_path)?;
    let wire_one: Result<Vec<DirectionVector>, _> =
        wires[0].split(",").map(DirectionVector::parse).collect();
    let wire_two: Result<Vec<DirectionVector>, _> =
        wires[1].split(",").map(DirectionVector::parse).collect();
    Ok((wire_one?, wire_two?))
}

pub fn part_one() -> Result<u32, Error> {
    let (wire_one, wire_two) = get_wires()?;
    let mut line_board = SparseLineBoard::default();
    line_board.bulk_travel(wire_one);
    let mut cursor = Point::default();
    let mut intersections = BTreeSet::new();
    // walk wire_two, checking each segment for intersections
    for vector in wire_two.into_iter() {
        let destination = cursor.travel(vector);
        let line = LineSegment{ start:cursor, end: destination};
        cursor = destination;
        for point in line_board.intersections(&line) {
            intersections.insert(point);
        }
    }
    intersections.remove(&Point::default());
    // we've defined min on `Point` as distance from origin, so let's grab it
    match intersections.iter().next() {
        Some(point) => Ok(point.from_origin()),
        None => Err(Error::NoSolutionFound),
    }
}

pub fn part_two() -> Result<u32, Error> {
    let (wire_one, wire_two) = get_wires()?;
    let mut line_board = SparseLineBoard::default();
    line_board.bulk_travel(wire_one);
    let mut cursor = Point::default();
    let mut traveled = 0;
    let mut intersection_latencies = Vec::new();
    for vector in wire_two.into_iter() {
        let destination = cursor.travel(vector);
        let line = LineSegment{ start: cursor, end: destination };
        if let Some((other_traveled, i)) = line_board.first_intersection(&line) {
            if i != Point::default() {
                let partial = LineSegment{ start: cursor, end: i}.length();
                intersection_latencies.push(other_traveled + traveled + partial);
            }
        }
        cursor = destination;
        traveled += line.length();
    }
    match intersection_latencies.into_iter().min() {
        Some(latency) => Ok(latency),
        None => Err(Error::NoSolutionFound),
    }
}
