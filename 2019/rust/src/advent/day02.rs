use crate::utils::errors::Error;
use crate::utils::files::{problem_input_path, read_file_split_on};

fn emulate_computer(tape: &mut Vec<u32>) {
    let mut idx = 0;
    loop {
        let opcode = tape[idx];
        match opcode {
            1 => {
                let first = tape[idx + 1];
                let second = tape[idx + 2];
                let location = tape[idx + 3];
                tape[location as usize] = tape[first as usize] + tape[second as usize];
            }
            2 => {
                let first = tape[idx + 1];
                let second = tape[idx + 2];
                let location = tape[idx + 3];
                tape[location as usize] = tape[first as usize] * tape[second as usize];
            }
            99 => break,
            _ => panic!("got bad opcode"),
        }
        idx += 4;
    }
}

pub fn part_one() -> Result<u32, Error> {
    let input_path = problem_input_path(2, None);
    let mut tape = read_file_split_on(&input_path, ",")?;
    tape[1] = 12;
    tape[2] = 2;
    emulate_computer(&mut tape);
    Ok(tape[0])
}

pub fn part_two() -> Result<u32, Error> {
    let input_path = problem_input_path(2, None);
    let orig_tape = read_file_split_on(&input_path, ",")?;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut tape = orig_tape.clone();
            tape[1] = noun;
            tape[2] = verb;
            emulate_computer(&mut tape);
            if tape[0] == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err(Error::NoSolutionFound)
}
