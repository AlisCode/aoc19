use std::collections::VecDeque;

pub fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|a| a.parse::<i32>().unwrap())
        .collect()
}

#[derive(Debug)]
pub enum Value {
    Immediate(i32),
    Position(usize),
}

impl Value {
    pub fn evaluate(self, code: &Vec<i32>) -> i32 {
        match self {
            Value::Immediate(x) => x,
            Value::Position(addr) => code[addr],
        }
    } 

    pub fn from(val: i32, param: i32) -> Self {
        if param == 0 {
            return Value::Position(val as usize);
        }
        Value::Immediate(val)
    }

}

#[derive(Debug)]
pub enum Instruction { 
    Add(Value, Value, usize),
    Mul(Value, Value, usize),
    Inp(usize),
    Out(Value),
    JumpIfTrue(Value, Value),
    JumpIfFalse(Value, Value),
    LessThan(Value, Value, usize),
    Equals(Value, Value, usize),
    Halt,
}

impl Instruction {
    fn parse_instr(code: &Vec<i32>, pointer: usize) -> Self {
        let i = code[pointer];
        let (opcode, p1, p2, p3) = match i {
            0..=99 => (i,0,0,0),
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
            1 => Instruction::Add(Value::from(code[pointer+1], p1), Value::from(code[pointer+2], p2), code[pointer+3] as usize),
            2 => Instruction::Mul(Value::from(code[pointer+1], p1), Value::from(code[pointer+2], p2), code[pointer+3] as usize),
            3 => Instruction::Inp(code[pointer+1] as usize),
            4 => Instruction::Out(Value::from(code[pointer+1], p1)),
            5 => Instruction::JumpIfTrue(Value::from(code[pointer+1], p1), Value::from(code[pointer+2], p2)),
            6 => Instruction::JumpIfFalse(Value::from(code[pointer+1], p1), Value::from(code[pointer+2], p2)),
            7 => Instruction::LessThan(Value::from(code[pointer+1], p1), Value::from(code[pointer+2], p2), code[pointer+3] as usize),
            8 => Instruction::Equals(Value::from(code[pointer+1], p1), Value::from(code[pointer+2], p2), code[pointer+3] as usize),
            99 => Instruction::Halt,
            x => { println!("Unhandled {}", x); unimplemented!() },
        }
    }

    pub fn args_count(&self) -> usize {
        match self {
            Instruction::Add(_,_,_,) => 3,
            Instruction::Mul(_,_,_,) => 3,
            Instruction::Inp(_) => 1,
            Instruction::Out(_) => 1,
            Instruction::JumpIfTrue(_, _) => 2,
            Instruction::JumpIfFalse(_, _) => 2,
            Instruction::LessThan(_, _,_)  => 3,
            Instruction::Equals(_, _,_)  => 3,
            Instruction::Halt => 0,
        }
    }
}

#[derive(Debug)]
pub struct Computer {
    /// Data of the program
    code: Vec<i32>, 
    /// Code pointer
    pointer: usize,
    /// Output of the computer 
    pub output: VecDeque<i32>,
    /// Input of the computer
    input: VecDeque<i32>,
    /// Whether or not the computer should halt execution on output or not
    halt_on_output: bool,
    /// Whether the computer should halt execution on missing input or not
    halt_on_missing_input: bool,
}

impl Computer {
    /// Creates a new instance of the Intcode Computer
    pub fn new(code: Vec<i32>) -> Self {
        Computer {
            code, 
            pointer: 0,
            output: VecDeque::new(),
            input: VecDeque::new(),
            halt_on_output: false,
            halt_on_missing_input: false,
        }
    }

    /// Manually sets an address to a value
    pub fn set(&mut self, addr: usize, value: i32) {
        self.code[addr] = value;
    }

    /// Gets the value at the given addr
    pub fn get(&self, addr: usize) -> i32 {
        self.code[addr]
    }

    /// Adds a value to the input of the computer
    pub fn input(&mut self, val: i32) {
        self.input.push_back(val);
    }

    pub fn get_next_output(&mut self) -> Option<i32> {
        self.output.pop_front()
    }

    pub fn get_all_output(&self) -> impl Iterator<Item = &i32> {
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
            Instruction::Add(a,b,c) => self.code[c] = a.evaluate(&self.code) + b.evaluate(&self.code),
            Instruction::Mul(a,b,c) => self.code[c] = a.evaluate(&self.code) * b.evaluate(&self.code),
            Instruction::Inp(a) => { let inp = self.input.pop_front(); if let Some(i) = inp { self.code[a] = i; } else { return !self.halt_on_missing_input; } },
            Instruction::Out(a) => { self.output.push_back(a.evaluate(&self.code)); self.pointer += forward + 1; return !self.halt_on_output; },
            Instruction::JumpIfTrue(a, b) => {
                if a.evaluate(&self.code) != 0 {
                    self.pointer = b.evaluate(&self.code) as usize;
                    change_pc = false;
                }
            },
            Instruction::JumpIfFalse(a, b) => {
                if a.evaluate(&self.code) == 0 {
                    self.pointer = b.evaluate(&self.code) as usize;
                    change_pc = false;
                }
            },
            Instruction::LessThan(a, b, c) => {
                let a = a.evaluate(&self.code);
                let b = b.evaluate(&self.code);
                self.code[c] = if a < b { 1 } else { 0 }
            },
            Instruction::Equals(a, b, c) => {
                let a = a.evaluate(&self.code);
                let b = b.evaluate(&self.code);
                self.code[c] = if a == b { 1 } else { 0 }
            },
            Instruction::Halt => { return false; }
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
        return self.code[self.pointer] == 99
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Computer, parse_input};

    #[test]
    pub fn computer_test_case() {
        let input = parse_input("1002,4,3,4,33");
        let mut computer = Computer::new(input);
        computer.step();
        assert_eq!(computer.get(4), 99);
        assert!(!computer.step());
    }

    // TODO: add day5 unit test
}