use crate::utils::*;
use eyre::{ContextCompat, Report, Result};
use std::{fs, str::FromStr};

// solution for part 2 is checking just outside the borders.
// Since we are told there is exactly one undetected, we know it must be adjacent to a detected square or there would be more than one.
// TODO: switch part one
// TODO: Another solution I saw was to rotate 45 degrees and then the ranges are squares so you can do easier intersections to find the missing interval (of one point).

pub fn part_one() -> Result<usize> {
    let input_path = problem_input_path(15, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_one_inner::<2000000>(&content)?;
    println!("{}", result);
    Ok(result)
}

pub fn part_two() -> Result<isize> {
    let input_path = problem_input_path(15, Some(1));
    let content = fs::read_to_string(input_path)?;
    let result = part_two_inner::<4000000>(&content)?;
    println!("{}", result);
    Ok(result)
}

fn part_one_inner<const Y: isize>(input: &str) -> Result<usize> {
    let readings: SensorReadings = str::parse(input)?;
    let min = readings
        .0
        .iter()
        .map(|reading| reading.sensor.x - reading.radius as isize)
        .min()
        .unwrap();
    let max = readings
        .0
        .iter()
        .map(|reading| reading.sensor.x + reading.radius as isize)
        .max()
        .unwrap();
    Ok((min..=max)
        .map(|x| {
            let beacon = Point { x, y: Y };
            let disqualified = readings.disqualifies_beacon(&beacon, true);
            disqualified
        })
        .filter(|b| *b)
        .count())
}

fn part_two_inner<const MAX: isize>(input: &str) -> Result<isize> {
    let readings: SensorReadings = str::parse(input)?;
    let Point { x, y } = readings
        .0
        .iter()
        .map(|reading| reading.border())
        .flatten()
        .filter(|point| {
            0 <= point.x
                && point.x <= MAX
                && 0 <= point.y
                && point.y <= MAX
                && !readings.disqualifies_beacon(point, false)
        })
        .next()
        .unwrap();
    Ok(x * 4000000 + y)
}

fn _parse_xy(s: &str) -> Result<(isize, isize), Report> {
    let (x, y) = s
        .split_once(',')
        .wrap_err_with(|| format!("couldn't split [{}]", s))?;
    let x = str::parse(
        x.strip_prefix("x=")
            .wrap_err_with(|| format!("failed to strip x= from [{}]", x))?,
    )?;
    let y = str::parse(
        y.trim()
            .strip_prefix("y=")
            .wrap_err_with(|| format!("failed to strip y= from [{}]", y))?,
    )?;
    Ok((x, y))
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Ord, PartialOrd)]
struct Point {
    x: isize,
    y: isize,
}

impl FromStr for Point {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = _parse_xy(s)?;
        Ok(Point { x, y })
    }
}

fn distance(sensor: &Point, beacon: &Point) -> usize {
    sensor.x.abs_diff(beacon.x) + sensor.y.abs_diff(beacon.y)
}

struct SensorReading {
    sensor: Point,
    beacon: Point,
    radius: usize,
}

impl FromStr for SensorReading {
    type Err = Report;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = line
            .strip_prefix("Sensor at ")
            .wrap_err_with(|| format!("failed to strip sensor prefix for [{}]", line))?;
        let (sensor_coords, line) = line
            .split_once(':')
            .wrap_err_with(|| format!("failed to split on ':' for [{}]", line))?;
        let beacon_coords = line
            .strip_prefix(" closest beacon is at ")
            .wrap_err_with(|| format!("failed to strip beacon prefix for [{}]", line))?;
        let sensor = str::parse(sensor_coords)?;
        let beacon = str::parse(beacon_coords)?;
        Ok(SensorReading { sensor, beacon, radius: distance(&sensor, &beacon) })
    }
}

impl SensorReading {
    fn disqualifies_beacon(&self, beacon: &Point, dont_disqualify_self: bool) -> bool {
        if dont_disqualify_self && *beacon == self.beacon {
            return false;
        }
        let beacon_distance = distance(&self.sensor, beacon);
        beacon_distance <= self.radius
    }

    fn border(&self) -> impl Iterator<Item = Point> {
        let d = self.radius + 1;
        let mut points = Vec::with_capacity(d * 4);
        let top_point = Point {
            x: self.sensor.x,
            y: self.sensor.y - d as isize,
        };
        let bottom_point = Point {
            x: self.sensor.x,
            y: self.sensor.y + d as isize,
        };
        points.push(top_point);
        let mut point = Point {
            x: top_point.x + 1,
            y: top_point.y + 1,
        };
        for _ in 0..d {
            points.push(point);
            point = Point {
                x: point.x + 1,
                y: point.y + 1,
            };
        }
        point = Point {
            x: top_point.x - 1,
            y: top_point.y + 1,
        };
        for _ in 0..d {
            points.push(point);
            point = Point {
                x: point.x - 1,
                y: point.y + 1,
            };
        }
        point = Point {
            x: bottom_point.x + 1,
            y: bottom_point.y - 1,
        };
        for _ in 0..d {
            points.push(point);
            point = Point {
                x: point.x + 1,
                y: point.y - 1,
            };
        }
        point = Point {
            x: bottom_point.x - 1,
            y: bottom_point.y - 1,
        };
        for _ in 0..d {
            points.push(point);
            point = Point {
                x: point.x - 1,
                y: point.y - 1,
            };
        }
        points.into_iter()
    }
}

struct SensorReadings(Vec<SensorReading>);

impl FromStr for SensorReadings {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let readings: Result<Vec<SensorReading>, _> =
            s.trim().lines().map(str::parse::<SensorReading>).collect();
        Ok(SensorReadings(readings?))
    }
}

impl SensorReadings {
    fn disqualifies_beacon(&self, beacon: &Point, dont_disqualify_self: bool) -> bool {
        self.0
            .iter()
            .map(|reading| {
                let disqualifies = reading.disqualifies_beacon(beacon, dont_disqualify_self);
                disqualifies
            })
            .any(|b| b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one_inner::<10>(TEST_INPUT).unwrap(), 26);
        assert_eq!(part_one().unwrap(), 4951427);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two_inner::<20>(TEST_INPUT).unwrap(), 56000011);
        assert_eq!(part_two().unwrap(), 13029714573243);
    }
}
