use crate::utils::*;

enum Heading {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl From<String> for Heading {
    fn from(s: String) -> Self {
        let mut splitter = s.split_whitespace();
        let direction = splitter.next().expect("expected direction");
        let magnitude = splitter
            .next()
            .expect(&format!("expected magnitude for {}", s))
            .parse::<i64>()
            .expect("failed to parse magnitude");
        match direction {
            "forward" => Self::Forward(magnitude),
            "down" => Self::Down(magnitude),
            "up" => Self::Up(magnitude),
            _ => panic!("Malformed input"),
        }
    }
}

pub fn part_one() {
    let ip = problem_input_path(2, Some(1));
    let data: Vec<String> = read_file_split_on(&ip, "\n").expect("failed to parse");
    let parsed: Vec<Heading> = data.into_iter().map(From::from).collect();
    let (x_pos, depth) = parsed.into_iter().fold((0, 0), |(x, d), h| match h {
        Heading::Forward(m) => (x + m, d),
        Heading::Down(m) => (x, d + m),
        Heading::Up(m) => (x, d - m),
    });
    println!("{} x {} = {}", x_pos, depth, x_pos * depth);
}

pub fn part_two() {
    let ip = problem_input_path(2, Some(1));
    let data: Vec<String> = read_file_split_on(&ip, "\n").expect("failed to parse");
    let parsed: Vec<Heading> = data.into_iter().map(From::from).collect();
    let (x_pos, depth, _) = parsed.into_iter().fold((0, 0, 0), |(x, d, a), h| match h {
        Heading::Forward(m) => (x + m, d + a * m, a),
        Heading::Down(m) => (x, d, a + m),
        Heading::Up(m) => (x, d, a - m),
    });
    println!("{} x {} = {}", x_pos, depth, x_pos * depth);
}
