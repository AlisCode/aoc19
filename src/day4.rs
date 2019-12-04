#[aoc_generator(day4)]
fn generator_input(input: &str) -> (u32, u32) {
    let nbs = input
        .split("-")
        .map(|x| x.parse::<u32>().expect("Failed to parse u32"))
        .collect::<Vec<u32>>();
    (nbs[0], nbs[1])
}

fn pass_match(x: &str) -> bool {
    let mut nb_adjacent = 0;
    let mut decrease = false;
    x.chars().zip(x.chars().skip(1)).for_each(|(a, b)| {
        if a == b {
            nb_adjacent += 1;
        }
        let a = a.to_string().parse::<u32>().unwrap();
        let b = b.to_string().parse::<u32>().unwrap();
        if a > b {
            decrease = true;
        }
    });
    nb_adjacent >= 1 && !decrease
}

fn pass_match_two(x: &str) -> bool {
    let mut nb_adjacent = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut decrease = false;
    x.chars().zip(x.chars().skip(1)).for_each(|(a, b)| {
        if a == b {
            nb_adjacent[a.to_string().parse::<usize>().unwrap()] += 1;
        }
        let a = a.to_string().parse::<u32>().unwrap();
        let b = b.to_string().parse::<u32>().unwrap();
        if a > b {
            decrease = true;
        }
    });
    nb_adjacent.iter().filter(|&x| *x == 1).count() >= 1 && !decrease
}

#[aoc(day4, part1)]
fn part_one((a, b): &(u32, u32)) -> usize {
    (*a..=*b)
        .map(|x| format!("{}", x))
        .filter(|x| pass_match(x))
        .count()
}

#[aoc(day4, part2)]
fn part_two((a, b): &(u32, u32)) -> usize {
    (*a..=*b)
        .map(|x| format!("{}", x))
        .filter(|x| pass_match_two(x))
        .count()
}

#[cfg(test)]
pub mod tests {
    use super::{pass_match, pass_match_two};
    #[test]
    fn day4_part_one() {
        assert!(pass_match("111111"));
        assert!(!pass_match("223450"));
        assert!(!pass_match("123789"));
    }

    #[test]
    fn day4_part_two() {
        assert!(pass_match_two("112233"));
        assert!(!pass_match_two("123444"));
        assert!(pass_match_two("111122"));
    }
}
