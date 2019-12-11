use std::collections::VecDeque;

pub fn parse_input(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|a| a.parse::<i64>().unwrap())
        .collect()
}

#[derive(Debug)]
pub enum Value {
    Immediate(i64),
    Position(usize),
    Relative(i64),
}

impl Value {
    pub fn evaluate(self, code: &Vec<i64>, relative_base: i64, write_mode: bool) -> i64 {
        match self {
            Value::Immediate(x) => x,
            Value::Position(addr) if !write_mode => code[addr],
            Value::Position(addr) if write_mode => addr as i64,
            Value::Relative(addr) if !write_mode => code[(addr + relative_base) as usize], 
            Value::Relative(addr) if write_mode => addr + relative_base,
            _ => unreachable!()
        }
    }

    pub fn from(val: i64, param: i64) -> Self {
        match param {
            0 => Value::Position(val as usize),
            1 => Value::Immediate(val),
            2 => Value::Relative(val),
            _ => unimplemented!()
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Add(Value, Value, Value),
    Mul(Value, Value, Value),
    Inp(Value),
    Out(Value),
    JumpIfTrue(Value, Value),
    JumpIfFalse(Value, Value),
    LessThan(Value, Value, Value),
    Equals(Value, Value, Value),
    SetRelativeBase(Value),
    Halt,
}

impl Instruction {
    fn parse_instr(code: &Vec<i64>, pointer: usize) -> Self {
        let i = code[pointer];
        //println!("parse i {}", i);
        let (opcode, p1, p2, p3) = match i {
            0..=99 => (i, 0, 0, 0),
            _ => {
                let opcode = i % 100;
                let param = i / 100;
                let p3 = i / 10000;
                let p2 = (param % 100) / 10;
                let p1 = param % 10;
                (opcode, p1, p2, p3)
            }
        };
        match opcode {
            1 => Instruction::Add(
                Value::from(code[pointer + 1], p1),
                Value::from(code[pointer + 2], p2),
                Value::from(code[pointer + 3], p3),
            ),
            2 => Instruction::Mul(
                Value::from(code[pointer + 1], p1),
                Value::from(code[pointer + 2], p2),
                Value::from(code[pointer + 3], p3),
            ),
            3 => Instruction::Inp(Value::from(code[pointer + 1], p1)),
            4 => Instruction::Out(Value::from(code[pointer + 1], p1)),
            5 => Instruction::JumpIfTrue(
                Value::from(code[pointer + 1], p1),
                Value::from(code[pointer + 2], p2),
            ),
            6 => Instruction::JumpIfFalse(
                Value::from(code[pointer + 1], p1),
                Value::from(code[pointer + 2], p2),
            ),
            7 => Instruction::LessThan(
                Value::from(code[pointer + 1], p1),
                Value::from(code[pointer + 2], p2),
                Value::from(code[pointer + 3], p3),
            ),
            8 => Instruction::Equals(
                Value::from(code[pointer + 1], p1),
                Value::from(code[pointer + 2], p2),
                Value::from(code[pointer + 3], p3),
            ),
            9 => Instruction::SetRelativeBase(
                Value::from(code[pointer + 1], p1),
            ),
            99 => Instruction::Halt,
            x => {
                println!("Unhandled {}", x);
                unimplemented!()
            }
        }
    }

    pub fn args_count(&self) -> usize {
        match self {
            Instruction::Add(_, _, _) => 3,
            Instruction::Mul(_, _, _) => 3,
            Instruction::Inp(_) => 1,
            Instruction::Out(_) => 1,
            Instruction::JumpIfTrue(_, _) => 2,
            Instruction::JumpIfFalse(_, _) => 2,
            Instruction::LessThan(_, _, _) => 3,
            Instruction::Equals(_, _, _) => 3,
            Instruction::SetRelativeBase(_) => 1,
            Instruction::Halt => 0,
        }
    }
}

#[derive(Debug)]
pub struct Computer {
    /// Data of the program
    code: Vec<i64>,
    /// Code pointer
    pointer: usize,
    /// Output of the computer
    pub output: VecDeque<i64>,
    /// Input of the computer
    input: VecDeque<i64>,
    /// Whether or not the computer should halt execution on output or not
    halt_on_output: bool,
    /// Whether the computer should halt execution on missing input or not
    halt_on_missing_input: bool,
    /// The relative base (day9)
    relative_base: i64,
}

impl Computer {
    /// Creates a new instance of the Intcode Computer
    pub fn new(code: Vec<i64>) -> Self {
        Computer {
            code,
            pointer: 0,
            output: VecDeque::new(),
            input: VecDeque::new(),
            halt_on_output: false,
            halt_on_missing_input: false,
            relative_base: 0,
        }
    }

    /// Manually sets an address to a value
    pub fn set(&mut self, addr: usize, value: i64) {
        self.code[addr] = value;
    }

    /// Sets the size of the available memory
    pub fn set_available_memory(mut self, memory: usize) -> Self {
        self.code.resize(memory, 0);
        self
    }

    /// Gets the value at the given addr
    pub fn get(&self, addr: usize) -> i64 {
        self.code[addr]
    }

    /// Adds a value to the input of the computer
    pub fn input(&mut self, val: i64) {
        self.input.push_back(val);
    }

    pub fn get_next_output(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    pub fn get_all_output(&self) -> impl Iterator<Item = &i64> {
        self.output.iter()
    }

    /// Sets the halt_on_output flag on that computer instance
    pub fn halt_on_output(mut self) -> Self {
        self.halt_on_output = true;
        self
    }

    /// Sets the halt_on_output flag on that computer instance
    pub fn halt_on_missing_input(mut self) -> Self {
        self.halt_on_missing_input = true;
        self
    }

    /// Executes the next instruction. Returns false if the program has halted
    pub fn step(&mut self) -> bool {
        let instr = Instruction::parse_instr(&self.code, self.pointer);
        let forward = instr.args_count();
        let mut change_pc = true;
        match instr {
            Instruction::Add(a, b, c) => {
                let dst = c.evaluate(&self.code, self.relative_base, true) as usize; 
                self.code[dst] = a.evaluate(&self.code, self.relative_base, false) + b.evaluate(&self.code, self.relative_base, false)
            }
            Instruction::Mul(a, b, c) => {
                let dst = c.evaluate(&self.code, self.relative_base, true) as usize; 
                self.code[dst] = a.evaluate(&self.code, self.relative_base, false) * b.evaluate(&self.code, self.relative_base, false)
            }
            Instruction::Inp(a) => {
                let inp = self.input.pop_front();
                if let Some(i) = inp {
                    let dst = a.evaluate(&self.code, self.relative_base, true) as usize;
                    self.code[dst] = i;
                } else {
                    return !self.halt_on_missing_input;
                }
            }
            Instruction::Out(a) => {
                self.output.push_back(a.evaluate(&self.code, self.relative_base, false));
                self.pointer += forward + 1;
                return !self.halt_on_output;
            }
            Instruction::JumpIfTrue(a, b) => {
                if a.evaluate(&self.code, self.relative_base, false) != 0 {
                    self.pointer = b.evaluate(&self.code, self.relative_base, false) as usize;
                    change_pc = false;
                }
            }
            Instruction::JumpIfFalse(a, b) => {
                if a.evaluate(&self.code, self.relative_base, false) == 0 {
                    self.pointer = b.evaluate(&self.code, self.relative_base, false) as usize;
                    change_pc = false;
                }
            }
            Instruction::LessThan(a, b, c) => {
                let a = a.evaluate(&self.code, self.relative_base, false);
                let b = b.evaluate(&self.code, self.relative_base, false);
                let dst = c.evaluate(&self.code, self.relative_base, true) as usize; 
                self.code[dst] = if a < b { 1 } else { 0 }
            }
            Instruction::Equals(a, b, c) => {
                let a = a.evaluate(&self.code, self.relative_base, false);
                let b = b.evaluate(&self.code, self.relative_base, false);
                let dst = c.evaluate(&self.code, self.relative_base, true) as usize; 
                self.code[dst] = if a == b { 1 } else { 0 }
            }
            Instruction::SetRelativeBase(offset) => {
                self.relative_base += offset.evaluate(&self.code, self.relative_base, false);
            },
            Instruction::Halt => {
                return false;
            }
        }
        if change_pc {
            self.pointer += forward + 1;
        }
        true
    }

    /// Displays the current code
    pub fn debug(&self) {
        println!("{:?}", self.code);
    }

    pub fn debug_output(&self) {
        println!("{:?}", self.output);
    }

    /// Executes the whole program
    pub fn execute(&mut self) {
        loop {
            if !self.step() {
                break;
            }
        }
    }

    /// Tells whether the amp is halted for good or not
    pub fn halted(&self) -> bool {
        return self.code[self.pointer] == 99;
    }
}

#[cfg(test)]
pub mod tests {
    use super::{parse_input, Computer};

    #[test]
    pub fn computer_test_case() {
        let input = parse_input("1002,4,3,4,33");
        let mut computer = Computer::new(input);
        computer.step();
        assert_eq!(computer.get(4), 99);
        assert!(!computer.step());
    }

    // TODO: add day5 unit test

    #[test]
    pub fn computer_day9_tests() {
        let input = parse_input("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let mut computer = Computer::new(input).set_available_memory(150);
        computer.execute();
        let output: Vec<i64> = computer.get_all_output().cloned().collect();
        assert_eq!(output, vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
        
        let input = parse_input("1102,34915192,34915192,7,4,7,99,0");
        let mut computer = Computer::new(input).set_available_memory(150);
        computer.execute();
        let nb = computer.get_next_output().unwrap();
        assert_eq!(nb.to_string().len(), 16);

        let input = parse_input("104,1125899906842624,99");
        let mut computer = Computer::new(input);
        computer.execute();
        assert_eq!(computer.get_next_output().unwrap(), 1125899906842624);
    }
}
