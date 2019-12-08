use crate::intcode_computer::{Computer, parse_input};
use itertools::Itertools;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    parse_input(input)
}

fn solve_sequence(input: &[i32], sequence: Vec<i32>) -> i32 {
    (0..5)
    .fold(0, |inp_signal, idx| {
        let mut amp = Computer::new(input.to_vec()); 
        amp.input(sequence[idx]);
        amp.input(inp_signal);
        amp.execute(); 
        let out = amp.output[0];
        out
    })
}

fn solve_sequence_feedback(input: &[i32], sequence: Vec<i32>) -> i32 {
    let mut amps: Vec<Computer> = 
    sequence 
    .iter()
    .map(|i| { 
        let mut c = Computer::new(input.to_vec()).halt_on_output().halt_on_missing_input(); 
        c.input(*i); 
        c 
    })
    .collect();

    (0..)
    .scan(0, |state, idx| {
        let idx = idx % 5;
        amps[idx].input(*state);
        amps[idx].execute();
        let out = amps[idx].output.pop_front();
        if let Some(o) = out {
            *state = o;
        }
        out
    })
    .last()
    .unwrap()
}


#[aoc(day7, part1)]
fn part_one(input: &[i32]) -> i32 {
    (0..5)
    .permutations(5)
    .map(|p| solve_sequence(input, p))
    .max()
    .expect("Failed to find max")
}

#[aoc(day7, part2)]
fn part_two(input: &[i32]) -> i32 {
    (5..10)
    .permutations(5)
    .map(|p| solve_sequence_feedback(input, p))
    .max()
    .expect("Failed to find max")
}

#[cfg(test)]
pub mod tests {
    use super::{solve_sequence, solve_sequence_feedback, input_generator, part_two};

    #[test]
    fn day7_part_one() {
        let input = input_generator("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        let sequence = vec![4,3,2,1,0];
        assert_eq!(solve_sequence(&input, sequence), 43210);

        let input = input_generator("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
        let sequence = vec![0,1,2,3,4];
        assert_eq!(solve_sequence(&input, sequence), 54321);

        let input = input_generator("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
        let sequence = vec![1,0,4,3,2];
        assert_eq!(solve_sequence(&input, sequence), 65210);
    }

    #[test]
    fn day7_part_two() {
        let input = input_generator("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
        let sequence = vec![9,8,7,6,5];
        assert_eq!(solve_sequence_feedback(&input, sequence), 139629729);
        
        let input = input_generator("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
        let sequence = vec![9,7,8,5,6];
        assert_eq!(solve_sequence_feedback(&input, sequence), 18216);
        
        let input = input_generator("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
        assert_eq!(part_two(&input), 139629729);
        
        let input = input_generator("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
        assert_eq!(part_two(&input), 18216);
    }
}