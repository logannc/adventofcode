use crate::utils::errors::Error;
use crate::utils::files::{problem_input_path, read_file_split_on};
use crate::utils::tape_machine::emulate_computer;

pub fn part_one() -> Result<i32, Error> {
    let input_path = problem_input_path(5, None);
    let mut tape = read_file_split_on(&input_path, ",")?;
    let mut outputs = emulate_computer(&mut tape, &vec![1])?;
    let diagnostic_code = outputs.split_off(outputs.len()-1)[0];
    if outputs.iter().all(|c| *c == 0) {
        return Ok(diagnostic_code);
    }
    dbg!(outputs, diagnostic_code);
    Err(Error::NoSolutionFound)
}

pub fn part_two() -> Result<i32, Error> {
    let input_path = problem_input_path(5, None);
    let mut tape = read_file_split_on(&input_path, ",")?;
    let mut outputs = emulate_computer(&mut tape, &vec![5])?;
    let diagnostic_code = outputs.split_off(outputs.len()-1)[0];
    if outputs.iter().all(|c| *c == 0) {
        return Ok(diagnostic_code);
    }
    dbg!(outputs, diagnostic_code);
    Err(Error::NoSolutionFound)
}