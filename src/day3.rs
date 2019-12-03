use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub enum Instruction {
    Right(u32),
    Down(u32),
    Left(u32),
    Up(u32),
}

impl Instruction {
    pub fn get_distance(&self) -> u32 {
        match self {
            Instruction::Right(x) => x.clone(),
            Instruction::Down(x) => x.clone(),
            Instruction::Left(x) => x.clone(),
            Instruction::Up(x) => x.clone(),
        }
    }
}

#[derive(Debug)]
pub enum InstructionParseError {
    IntError(ParseIntError),
    WrongIdent(String),
}

impl From<ParseIntError> for InstructionParseError {
    fn from(pie: ParseIntError) -> Self {
        InstructionParseError::IntError(pie)
    }
}

impl FromStr for Instruction {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ident = &s[0..=0];
        let rest = s[1..].parse::<u32>()?;
        match ident {
            "R" => Ok(Instruction::Right(rest)),
            "D" => Ok(Instruction::Down(rest)),
            "L" => Ok(Instruction::Left(rest)),
            "U" => Ok(Instruction::Up(rest)),
            _ => Err(InstructionParseError::WrongIdent(ident.to_string())),
        }
    }
}

impl std::fmt::Display for InstructionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InstructionParseError::IntError(pie) => write!(f, "Error parsing integer {}", pie),
            InstructionParseError::WrongIdent(what) => write!(f, "Wrong identifier {}", what),
        }
    }
}
impl std::error::Error for InstructionParseError {}

pub struct Node {
    position: (i32, i32),
    steps: u32,
}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}
impl Eq for Node {}

impl Node {
    pub fn new(position: (i32, i32), steps: u32) -> Self {
        Node { position, steps }
    }
}

#[aoc_generator(day3)]
fn generator_input(input: &str) -> Result<Vec<Vec<Instruction>>, InstructionParseError> {
    input
        .lines()
        .map(|a| {
            a.split(",")
                .map(|i| Instruction::from_str(i))
                .collect::<Result<Vec<Instruction>, InstructionParseError>>()
        })
        .collect()
}

struct CollisionChecker {
    line_1: std::collections::HashSet<Node>,
    line_2: std::collections::HashSet<Node>,
    steps: u32,
    current: (i32, i32),
}

impl CollisionChecker {
    pub fn new() -> Self {
        CollisionChecker {
            line_1: Default::default(),
            line_2: Default::default(),
            steps: 0,
            current: (0, 0),
        }
    }

    pub fn follow_line(&mut self, instruction: &Instruction, line: usize) {
        let dist = instruction.get_distance();
        (0..dist).for_each(|_| {
            match instruction {
                &Instruction::Right(_) => self.current.0 += 1,
                &Instruction::Left(_) => self.current.0 -= 1,
                &Instruction::Down(_) => self.current.1 -= 1,
                &Instruction::Up(_) => self.current.1 += 1,
            }
            let position = self.current.clone();
            self.steps += 1;
            let to_add = Node::new(position, self.steps);
            match line {
                1 => self.line_1.insert(to_add),
                2 => self.line_2.insert(to_add),
                _ => unreachable!(),
            };
        })
    }

    pub fn collisions(&self) -> impl Iterator<Item = &Node> {
        self.line_1.intersection(&self.line_2)
    }

    pub fn restart(&mut self) {
        self.current = (0, 0);
        self.steps = 0;
    }
}

fn populate_line(cc: &mut CollisionChecker, instructions: &Vec<Instruction>, line: usize) {
    cc.restart();
    instructions.iter().for_each(|i| cc.follow_line(i, line))
}

#[aoc(day3, part1)]
fn part_one(input: &[Vec<Instruction>]) -> i32 {
    let mut cc = CollisionChecker::new();
    populate_line(&mut cc, &input[0], 1);
    populate_line(&mut cc, &input[1], 2);
    cc.collisions()
        .map(|c| c.position.0.abs() + c.position.1.abs())
        .min()
        .expect("Failed to find min")
}

#[aoc(day3, part2)]
fn part_two(input: &[Vec<Instruction>]) -> u32 {
    let mut cc = CollisionChecker::new();
    populate_line(&mut cc, &input[0], 1);
    populate_line(&mut cc, &input[1], 2);
    cc.collisions()
        .map(|c| {
            let d1 = cc.line_1.get(c).unwrap().steps;
            let d2 = cc.line_2.get(c).unwrap().steps;
            d1 + d2
        })
        .min()
        .expect("Failed to find min")
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part_one, part_two};

    #[test]
    fn day3_part_one() {
        let generated = generator_input("R8,U5,L5,D3\nU7,R6,D4,L4").expect("Failed to parse");
        assert_eq!(part_one(&generated), 6);

        let generated =
            generator_input("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")
                .expect("Failed to parse");
        assert_eq!(part_one(&generated), 159);

        let generated = generator_input(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        )
        .expect("Failed to parse");
        assert_eq!(part_one(&generated), 135);
    }

    #[test]
    fn day3_part_two() {
        let generated = generator_input("R8,U5,L5,D3\nU7,R6,D4,L4").expect("Failed to parse");
        assert_eq!(part_two(&generated), 30);

        let generated =
            generator_input("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")
                .expect("Failed to parse");
        assert_eq!(part_two(&generated), 610);

        let generated = generator_input(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        )
        .expect("Failed to parse");
        assert_eq!(part_two(&generated), 410);
    }
}
