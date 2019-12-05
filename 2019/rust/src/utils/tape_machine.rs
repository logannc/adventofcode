use crate::utils::errors::Error;

type Address = usize;
type Value = i32;

enum Parameter {
    Position(Address),
    Immediate(Value),
}

impl Parameter {
    fn get_value(&self, tape: &Vec<Value>) -> Value {
        match self {
            Self::Position(p) => tape[*p],
            Self::Immediate(v) => *v,
        }
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

fn param(tape: &Vec<Value>, pos: usize, mode: Option<&Address>) -> Parameter {
    match mode.unwrap_or(&0) {
        0 => Parameter::Position(tape[pos] as usize),
        1 => Parameter::Immediate(tape[pos]),
        _ => panic!("unknown parameter mode"),
    }
}

fn decode_opcode(tape: &Vec<Value>, ip: Address) -> Result<OpCode, Error> {
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
            param(&tape, ip + 1, modes.get(0)),
            param(&tape, ip + 2, modes.get(1)),
            tape[ip + 3] as usize,
        )),
        "02" => Ok(OpCode::Mul(
            param(&tape, ip + 1, modes.get(0)),
            param(&tape, ip + 2, modes.get(1)),
            tape[ip + 3] as usize,
        )),
        "03" => Ok(OpCode::Input(tape[ip + 1] as usize)),
        "04" => Ok(OpCode::Output(param(&tape, ip + 1, modes.get(0)))),
        "05" => Ok(OpCode::JumpIfTrue(
            param(&tape, ip + 1, modes.get(0)),
            param(&tape, ip + 2, modes.get(1)),
        )),
        "06" => Ok(OpCode::JumpIfFalse(
            param(&tape, ip + 1, modes.get(0)),
            param(&tape, ip + 2, modes.get(1)),
        )),
        "07" => Ok(OpCode::LessThan(
            param(&tape, ip + 1, modes.get(0)),
            param(&tape, ip + 2, modes.get(1)),
            tape[ip + 3] as usize,
        )),
        "08" => Ok(OpCode::Equal(
            param(&tape, ip + 1, modes.get(0)),
            param(&tape, ip + 2, modes.get(1)),
            tape[ip + 3] as usize,
        )),
        "99" => Ok(OpCode::Halt),
        _ => Err(Error::BadOpcode(instruction)),
    }
}

pub fn emulate_computer(tape: &mut Vec<i32>, inputs: &Vec<i32>) -> Result<Vec<Value>, Error> {
    let mut inputs = inputs.into_iter();
    let mut outputs = Vec::new();
    let mut ip = 0;
    loop {
        let opcode = decode_opcode(&tape, ip)?;
        match opcode {
            OpCode::Add(p1, p2, a) => {
                tape[a] = p1.get_value(&tape) + p2.get_value(&tape);
                ip += 4;
            }
            OpCode::Mul(p1, p2, a) => {
                tape[a] = p1.get_value(&tape) * p2.get_value(&tape);
                ip += 4;
            }
            OpCode::Input(a) => {
                tape[a] = *inputs.next().unwrap();
                ip += 2;
            }
            OpCode::Output(p1) => {
                outputs.push(p1.get_value(&tape));
                ip += 2;
            }
            OpCode::JumpIfTrue(p1, p2) => {
                if p1.get_value(&tape) != 0 {
                    ip = p2.get_value(&tape) as usize;
                } else {
                    ip += 3
                }
            }
            OpCode::JumpIfFalse(p1, p2) => {
                if p1.get_value(&tape) == 0 {
                    ip = p2.get_value(&tape) as usize;
                } else {
                    ip += 3
                }
            }
            OpCode::LessThan(p1, p2, a) => {
                if p1.get_value(&tape) < p2.get_value(&tape) {
                    tape[a] = 1
                } else {
                    tape[a] = 0
                }
                ip += 4;
            }
            OpCode::Equal(p1, p2, a) => {
                if p1.get_value(&tape) == p2.get_value(&tape) {
                    tape[a] = 1
                } else {
                    tape[a] = 0
                }
                ip += 4;
            }
            OpCode::Halt => break,
        }
    }
    Ok(outputs)
}
