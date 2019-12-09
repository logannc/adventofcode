use crate::utils::errors::Error;
use crate::utils::files::{problem_input_path, read_file_split_on};
use crate::utils::tape_machine::{TapeMachine, TapeMachineState};

use std::collections::VecDeque;

fn _generate(k: usize, source: &mut Vec<i32>) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    if k == 1 {
        results.push(source.clone());
    } else {
        results.append(&mut _generate(k - 1, source));
        for i in 0..(k - 1) {
            if k & 1 == 0 {
                let tmp = source[i];
                source[i] = source[k - 1];
                source[k - 1] = tmp;
            } else {
                let tmp = source[0];
                source[0] = source[k - 1];
                source[k - 1] = tmp;
            }
            results.append(&mut _generate(k - 1, source));
        }
    }
    results
}

/// Implementation of Heap's algorithm to generate all permutations
/// I'd like to turn this into an iterator, but the recursive implementation is more
/// easily understood.
fn permutations(source: &Vec<i32>) -> Vec<Vec<i32>> {
    _generate(source.len(), &mut source.clone())
}

pub fn part_one() -> Result<i32, Error> {
    let input_path = problem_input_path(7, None);
    let tape = read_file_split_on(&input_path, ",")?;
    let mut highest_signal = i32::min_value();
    for state in permutations(&vec![0, 1, 2, 3, 4]) {
        let mut previous_signal = 0;
        for amp_phase in state {
            let mut tape_machine = TapeMachine::new(tape.clone(), false);
            tape_machine.add_input(amp_phase);
            tape_machine.add_input(previous_signal);
            match tape_machine.run()? {
                TapeMachineState::Halted => {
                    let outputs = tape_machine.get_outputs();
                    previous_signal = outputs[0];
                    if previous_signal > highest_signal {
                        highest_signal = previous_signal;
                    }
                }
                _ => return Err(Error::Infallible),
            }
        }
    }
    Ok(highest_signal)
}

pub fn part_two() -> Result<i32, Error> {
    let input_path = problem_input_path(7, None);
    let tape = read_file_split_on(&input_path, ",")?;
    let mut amps = VecDeque::new();
    let mut highest_signal = i32::min_value();
    for phase_states in permutations(&vec![5, 6, 7, 8, 9]) {
        for phase in phase_states {
            let mut tape_machine = TapeMachine::new(tape.clone(), true);
            tape_machine.add_input(phase);
            amps.push_back(tape_machine);
        }

        let mut previous_signal = 0;
        while !amps.is_empty() {
            let mut amp = amps.pop_front().unwrap();
            amp.add_input(previous_signal);
            match amp.run()? {
                TapeMachineState::RequestingInput => {
                    amps.push_back(amp);
                }
                TapeMachineState::YieldingOutput => {
                    previous_signal = amp.get_outputs_mut().pop_front().unwrap();
                    amps.push_back(amp);
                }
                TapeMachineState::Halted => {}
            }
        }
        if previous_signal > highest_signal {
            highest_signal = previous_signal;
        }
    }
    Ok(highest_signal)
}
