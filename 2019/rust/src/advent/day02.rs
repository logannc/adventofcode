use crate::utils::errors::Error;
use crate::utils::files::{problem_input_path, read_file_split_on};
use crate::utils::tape_machine::{TapeMachine, TapeMachineState};

pub fn part_one() -> Result<i32, Error> {
    let input_path = problem_input_path(2, None);
    let mut tape = read_file_split_on(&input_path, ",")?;
    tape[1] = 12;
    tape[2] = 2;
    let mut tape_machine = TapeMachine::new(tape, false);
    match tape_machine.run()? {
        TapeMachineState::Halted => Ok(tape_machine.get_value(0)),
        _ => Err(Error::NoSolutionFound),
    }
}

pub fn part_two() -> Result<i32, Error> {
    let input_path = problem_input_path(2, None);
    let orig_tape = read_file_split_on(&input_path, ",")?;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut tape = orig_tape.clone();
            tape[1] = noun;
            tape[2] = verb;
            let mut tape_machine = TapeMachine::new(tape, false);
            match tape_machine.run()? {
                TapeMachineState::Halted => {
                    if tape_machine.get_value(0) == 19690720 {
                        return Ok(100 * noun + verb);
                    }
                },
                _ => return Err(Error::NoSolutionFound),
            }
        }
    }
    Err(Error::NoSolutionFound)
}
