use crate::utils::errors::Error;
use crate::utils::files::{problem_input_path, read_file_split_on};
use crate::utils::tape_machine::{TapeMachine, TapeMachineState};

pub fn part_one() -> Result<i32, Error> {
    let input_path = problem_input_path(5, None);
    let tape = read_file_split_on(&input_path, ",")?;
    let mut tape_machine = TapeMachine::new(tape, false);
    tape_machine.add_input(1);
    match tape_machine.run()? {
        TapeMachineState::Halted => {
            let outputs = tape_machine.get_outputs_mut();
            let diagnostic_code = outputs.split_off(outputs.len() - 1)[0];
            if outputs.iter().all(|c| *c == 0) {
                return Ok(diagnostic_code);
            }
            dbg!(outputs, diagnostic_code);
            Err(Error::NoSolutionFound)
        }
        _ => Err(Error::Infallible),
    }
}

pub fn part_two() -> Result<i32, Error> {
    let input_path = problem_input_path(5, None);
    let tape = read_file_split_on(&input_path, ",")?;
    let mut tape_machine = TapeMachine::new(tape, false);
    tape_machine.add_input(5);
    match tape_machine.run()? {
        TapeMachineState::Halted => {
            let outputs = tape_machine.get_outputs_mut();
            let diagnostic_code = outputs.split_off(outputs.len() - 1)[0];
            if outputs.iter().all(|c| *c == 0) {
                return Ok(diagnostic_code);
            }
            dbg!(outputs, diagnostic_code);
            Err(Error::NoSolutionFound)
        }
        _ => Err(Error::Infallible),
    }
}
