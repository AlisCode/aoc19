use crate::intcode_computer::{parse_input, Computer};

#[aoc_generator(day5)]
fn generator_input(input: &str) -> Vec<i32> {
    parse_input(input)
}

#[aoc(day5, part1)]
fn part_one(input: &[i32]) -> i32 {
    let mut computer = Computer::new(input.to_vec());
    computer.input(1);
    computer.execute();
    *computer.output.iter().last().unwrap()
}

#[aoc(day5, part2)]
fn part_two(input: &[i32]) -> i32 {
    let mut computer = Computer::new(input.to_vec());
    computer.input(5);
    computer.execute();
    *computer.output.iter().last().unwrap()
}