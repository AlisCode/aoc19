/// Parses each line to be an i32
#[aoc_generator(day2)]
fn generator_input(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|a| a.parse::<i32>().unwrap())
        .collect()
}

#[derive(Clone)]
pub struct Program {
    /// Data of the program (parsed input)
    data: Vec<i32>,
    /// Code pointer
    pointer: usize,
}

impl Program {
    pub fn new(data: Vec<i32>) -> Self {
        Program { data, pointer: 0 }
    }
    
    /// Solves the program for given verb and noun
    pub fn solve_for(&mut self, verb: i32, noun: i32) -> i32 {
        self.data[1] = verb;
        self.data[2] = noun;
        while self.next() {}
        self.data[0]
    }

    // Continues the execution of the program, returning
    // true if the program should continue, false if it should stop
    pub fn next(&mut self) -> bool {
        let opcode = self.data[self.pointer];
        let idx_a = self.data[self.pointer + 1] as usize;
        let idx_b = self.data[self.pointer + 2] as usize;
        let idx_c = self.data[self.pointer + 3] as usize;
        let res = match opcode {
            1 => {
                let val = self.data[idx_a] + self.data[idx_b];
                self.data[idx_c] = val;
                true
            }
            2 => {
                let val = self.data[idx_a] * self.data[idx_b];
                self.data[idx_c] = val;
                true
            }
            _ => false,
        };
        if res {
            self.pointer += 4;
        }
        res
    }
}

#[aoc(day2, part1)]
/// Solves part one by solving for verb = 12 and noun = 2  
fn part_one(input: &[i32]) -> i32 {
    let mut program = Program::new(input.to_vec());
    program.solve_for(12, 2)
}

#[aoc(day2, part2)]
/// Solves part two : 
/// On the Reddit thread for solutions (https://www.reddit.com/r/adventofcode/comments/e4u0rw/2019_day_2_solutions/)
/// we can see that a constraint solver (z3) simplifies this problem as solving 19690720 = c1 + c2*v + n
/// where c1 and c2 will vary (since they depend on your input).
fn part_two(input: &[i32]) -> i32 {
    // The first step is to find c1 and c2. In order to do this, we need to solve the program for known verb and noun. 
    // Here, we'll solve for two cases: 
    // * v = 12 and n = 2, 
    // * v = 13 and n = 2
    //
    // This will give us two results: r1 and r2. We then have the system such as : 
    // { r1 = c1 + c2 * 12 + 2  
    // { r2 = c1 + c2 * 13 + 2
    let mut program = Program::new(input.to_vec());
    let mut program_copy = program.clone(); 
    let r1 = program.solve_for(12, 2); 
    let r2 = program_copy.solve_for(13, 2); 

    // Simplyfing this system gives us 
    // { c1 = 13 * r1 - 12 * r2 - 2
    // { c2 = r2 - r1
    let c1 = 13 * r1 - 12 * r2 - 2;
    let c2 = r2 - r1;

    // We then need to solve the following equation : 
    // 19690720 = c1 + c2 * v + n
    // ==> 196960720 - c1 = c2 * v + n
    // The most straightforward solution is then to use the euclidean division.
    let val = 19690720 - c1; 
    let (v, n) = (val / c2, val % c2);

    // And return the formatted result :)
    100 * v + n
}

#[cfg(test)]
pub mod tests {
    use super::{part_one, Program};

    #[test]
    fn day2_part_one() {
        let mut program = Program::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert!(program.next());
        assert_eq!(program.pointer, 4);
        assert_eq!(
            program.data,
            vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        assert!(program.next());
        assert_eq!(
            program.data,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        assert!(!program.next());
    }
}
