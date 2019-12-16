#[aoc_generator(day16)]
fn input_generator(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect()
}

const FFT_PATTERN: [i32; 4] = [0, 1, 0, -1];
fn fft_phase(input: Vec<i32>) -> Vec<i32> {
    input
        .iter()
        .enumerate()
        .map(|(idx, val)| {
            let curr_pattern: Vec<i32> = FFT_PATTERN
                .iter()
                .flat_map(|x| (0..idx + 1).map(move |_| *x))
                .collect();
            input
                .iter()
                .zip(curr_pattern.into_iter().cycle().skip(1))
                .map(|(a, b)| a * b)
                .sum::<i32>()
                .abs()
                % 10
        })
        .collect()
}

#[aoc(day16, part1)]
fn part_one(input: &[i32]) -> String {
    (0..100)
        .fold(input.to_vec(), |state, _| fft_phase(state))
        .into_iter()
        .map(|x| x.to_string())
        .take(8)
        .collect()
}

#[aoc(day16, part2)]
fn part_two(input: &[i32]) -> String {
    let offset: usize = input
        .iter()
        .map(|x| x.to_string())
        .take(7)
        .collect::<String>()
        .parse()
        .unwrap();
    let origin_len = input.len();
    let input_len = input.len() * 10000;
    let mut input: Vec<i32> = input
        .iter()
        .cloned()
        .cycle()
        .skip(offset % origin_len)
        .take(input_len - offset)
        .collect();
    input.reverse();
    (0..100).for_each(|_| {
        let mut state = 0;
        let len = input.len();
        (0..len).for_each(|x| {
            state += input[x];
            input[x] = state.abs() % 10;
        });
    });

    input.iter().rev().take(8).map(|x| x.to_string()).collect()
}

#[cfg(test)]
pub mod tests {
    use super::{input_generator, part_two};

    #[test]
    fn day16_part_two() {
        let input = "03036732577212944063491565474664";
        let input = input_generator(input);
        assert_eq!(part_two(&input), "84462026");
    }
}
