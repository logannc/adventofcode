use crate::utils::errors::Error;
use crate::utils::files::{problem_input_path, read_file_split_on};
use crate::utils::tape_machine::emulate_computer;

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

struct HeapPermutator {
    vec: Vec<i32>,
    kstack: Vec<usize>,
    stackpointer: usize,
}

impl HeapPermutator {
    fn new(source: &Vec<i32>) -> Self {
        HeapPermutator {
            vec: source.clone(),
            kstack: Vec::with_capacity(source.len()),
            stackpointer: 0,
        }
    }
}

impl Iterator for HeapPermutator {
    type Item = Vec<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        let k = self.kstack[self.stackpointer];
        if k == 1 {
            self.stackpointer -= 1;
            return Some(self.vec.clone());
        } else {
            self.stackpointer += 1;
            self.kstack[self.stackpointer] = k - 1;
            return self.next();
            // TODO: very, very wrong
        }
    }
}

fn permutations_iter(source: &Vec<i32>) -> impl Iterator<Item = Vec<i32>> {
    HeapPermutator::new(source)
}

pub fn part_one() -> Result<i32, Error> {
    let input_path = problem_input_path(7, None);
    let tape = read_file_split_on(&input_path, ",")?;
    let mut highest_signal = i32::min_value();
    for state in permutations(&vec![0, 1, 2, 3, 4]) {
        let mut previous_signal = 0;
        for amp_phase in state {
            let input = vec![amp_phase, previous_signal];
            let outputs = emulate_computer(&mut tape.clone(), &input)?;
            previous_signal = outputs[0];
            if previous_signal > highest_signal {
                highest_signal = previous_signal;
            }
        }
    }
    Ok(highest_signal)
}
