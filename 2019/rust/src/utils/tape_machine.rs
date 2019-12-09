use crate::utils::errors::Error;

use std::collections::VecDeque;
use std::convert::TryFrom;

type Address = usize;
type Value = i32;
type Tape = Vec<Value>;

enum Parameter {
    Position(Address),
    Immediate(Value),
}

impl Parameter {
    fn get_value(&self, tape: &Tape) -> Value {
        match self {
            Self::Position(p) => tape[*p],
            Self::Immediate(v) => *v,
        }
    }
}

fn param(tape: &Tape, pos: usize, mode: Option<&Address>) -> Result<Parameter, Error> {
    match mode.unwrap_or(&0) {
        0 => Ok(Parameter::Position(usize::try_from(tape[pos])?)),
        1 => Ok(Parameter::Immediate(tape[pos])),
        _ => panic!("unknown parameter mode"),
    }
}

enum OpCode {
    Add(Parameter, Parameter, Address),
    Mul(Parameter, Parameter, Address),
    Input(Address),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Address),
    Equal(Parameter, Parameter, Address),
    Halt,
}

impl OpCode {
    fn decode(tape: &Tape, ip: Address) -> Result<OpCode, Error> {
        let mut instruction = tape[ip].to_string();
        let split_loc = instruction.len().checked_sub(2).unwrap_or(0);
        let opcode = format!("{:0>2}", instruction.split_off(split_loc));
        let modes: Vec<Address> = instruction
            .chars()
            .flat_map(|c| c.to_digit(10))
            .map(|c| c as usize)
            .rev()
            .collect();
        match opcode.as_ref() {
            "01" => Ok(OpCode::Add(
                param(&tape, ip + 1, modes.get(0))?,
                param(&tape, ip + 2, modes.get(1))?,
                usize::try_from(tape[ip + 3])?,
            )),
            "02" => Ok(OpCode::Mul(
                param(&tape, ip + 1, modes.get(0))?,
                param(&tape, ip + 2, modes.get(1))?,
                usize::try_from(tape[ip + 3])?,
            )),
            "03" => Ok(OpCode::Input(usize::try_from(tape[ip + 1])?)),
            "04" => Ok(OpCode::Output(param(&tape, ip + 1, modes.get(0))?)),
            "05" => Ok(OpCode::JumpIfTrue(
                param(&tape, ip + 1, modes.get(0))?,
                param(&tape, ip + 2, modes.get(1))?,
            )),
            "06" => Ok(OpCode::JumpIfFalse(
                param(&tape, ip + 1, modes.get(0))?,
                param(&tape, ip + 2, modes.get(1))?,
            )),
            "07" => Ok(OpCode::LessThan(
                param(&tape, ip + 1, modes.get(0))?,
                param(&tape, ip + 2, modes.get(1))?,
                usize::try_from(tape[ip + 3])?,
            )),
            "08" => Ok(OpCode::Equal(
                param(&tape, ip + 1, modes.get(0))?,
                param(&tape, ip + 2, modes.get(1))?,
                usize::try_from(tape[ip + 3])?,
            )),
            "99" => Ok(OpCode::Halt),
            _ => Err(Error::BadOpcode(instruction)),
        }
    }
}

pub enum TapeMachineState {
    RequestingInput,
    YieldingOutput,
    Halted,
}

pub struct TapeMachine {
    tape: Vec<Value>,
    ip: Address,
    input: VecDeque<Value>,
    output: VecDeque<Value>,
    yield_on_input: bool,
}

impl TapeMachine {
    pub fn new(tape: Tape, yield_on_input: bool) -> Self {
        TapeMachine {
            tape: tape,
            ip: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            yield_on_input: yield_on_input,
        }
    }
    fn execute_opcode(&mut self, op: OpCode) -> Result<Option<TapeMachineState>, Error> {
        match op {
            OpCode::Add(p1, p2, a) => {
                self.tape[a] = p1.get_value(&self.tape) + p2.get_value(&self.tape);
                self.ip += 4;
            }
            OpCode::Mul(p1, p2, a) => {
                self.tape[a] = p1.get_value(&self.tape) * p2.get_value(&self.tape);
                self.ip += 4;
            }
            OpCode::Input(a) => {
                if let Some(val) = self.input.pop_front() {
                    self.tape[a] = val;
                    self.ip += 2;
                } else {
                    return Ok(Some(TapeMachineState::RequestingInput));
                }
            }
            OpCode::Output(p1) => {
                self.output.push_back(p1.get_value(&self.tape));
                self.ip += 2;
                if self.yield_on_input {
                    return Ok(Some(TapeMachineState::YieldingOutput));
                }
            }
            OpCode::JumpIfTrue(p1, p2) => {
                if p1.get_value(&self.tape) != 0 {
                    self.ip = usize::try_from(p2.get_value(&self.tape))?;
                } else {
                    self.ip += 3;
                }
            }
            OpCode::JumpIfFalse(p1, p2) => {
                if p1.get_value(&self.tape) == 0 {
                    self.ip = usize::try_from(p2.get_value(&self.tape))?;
                } else {
                    self.ip += 3;
                }
            }
            OpCode::LessThan(p1, p2, a) => {
                if p1.get_value(&self.tape) < p2.get_value(&self.tape) {
                    self.tape[a] = 1;
                } else {
                    self.tape[a] = 0;
                }
                self.ip += 4;
            }
            OpCode::Equal(p1, p2, a) => {
                if p1.get_value(&self.tape) == p2.get_value(&self.tape) {
                    self.tape[a] = 1;
                } else {
                    self.tape[a] = 0;
                }
                self.ip += 4;
            }
            OpCode::Halt => return Ok(Some(TapeMachineState::Halted)),
        };
        Ok(None)
    }
    pub fn run(&mut self) -> Result<TapeMachineState, Error> {
        loop {
            let opcode = OpCode::decode(&self.tape, self.ip)?;
            if let Some(state) = self.execute_opcode(opcode)? {
                return Ok(state);
            }
        }
    }
    pub fn get_value(&self, addr: Address) -> Value {
        self.tape[addr]
    }
    pub fn get_outputs(&self) -> &VecDeque<Value> {
        &self.output
    }
    pub fn get_outputs_mut(&mut self) -> &mut VecDeque<Value> {
        &mut self.output
    }
    pub fn add_input(&mut self, val: Value) {
        self.input.push_back(val)
    }
}