/// Parses each line to be an i32
#[aoc_generator(day2)]
fn generator_input(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|a| a.parse::<i32>().unwrap())
        .collect()
}

pub struct Program {
    pub data: Vec<i32>,
    pointer: usize,
}

impl Program {
    pub fn new(data: Vec<i32>) -> Self {
        Program { data, pointer: 0 }
    }

    // Continues the execution of the program, returning
    // true if the program should continue, false if it should stop
    pub fn next(&mut self) -> bool {
        let opcode = self.data[self.pointer];
        let res = match opcode {
            1 => {
                let val = self.data[self.pointer + 1] + self.data[self.pointer + 2];
                let index_to = self.data[self.pointer + 3] as usize;
                self.data[index_to] = val;
                true
            }
            2 => {
                let val = self.data[self.pointer + 1] * self.data[self.pointer + 2];
                let index_to = self.data[self.pointer + 3] as usize;
                self.data[index_to] = val;
                true
            }
            99 => {
                println!("Opcode is 99");
                false
            }
            x => {
                println!("Something went wrong");
                false
            }
        };
        if res {
            self.pointer += 4;
        }
        res
    }
}

#[aoc(day2, part1)]
/// Solves part one by applying the calc_mass computation
fn part_one(input: &[i32]) -> i32 {
    let mut program = Program::new(input.to_vec());
    program.data[1] = 12;
    program.data[2] = 2;
    while program.next() {}
    program.data[0]
}

#[cfg(test)]
pub mod tests {
    use super::part_one;

    #[test]
    fn day2_part_one() {
        assert_eq!(
            part_one(&vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            3500
        );
    }
}
