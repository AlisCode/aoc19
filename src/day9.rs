use crate::intcode_computer::{parse_input, Computer};

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<i64> {
    parse_input(input)
}

#[aoc(day9, part1)]
pub fn part_one(input: &[i64]) -> i64 {
    let mut computer = Computer::new(input.to_vec()).set_available_memory(1500);
    computer.input(1);
    computer.execute();
    computer.get_next_output().unwrap()
}

#[aoc(day9, part2)]
pub fn part_two(input: &[i64]) -> i64 {
    let mut computer = Computer::new(input.to_vec()).set_available_memory(1500);
    computer.input(2);
    computer.execute();
    computer.get_next_output().unwrap()
}