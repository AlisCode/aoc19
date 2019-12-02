/// Parses each line to be an i32
#[aoc_generator(day1)]
fn generator_input(input: &str) -> Vec<i32> {
    input.lines().map(|a| a.parse::<i32>().unwrap()).collect()
}

/// Calculates the fuel necessary for a given mass
fn calc_mass(x: &i32) -> i32 {
    x / 3 - 2
}

/// Calculates the recursive fuel necessary for a given mass
fn calc_total_mass(x: &i32) -> i32 {
    (0..)
        .scan(*x, |xx, _| {
            *xx = calc_mass(xx);
            if *xx > 0 {
                return Some(*xx);
            }
            None
        })
        .sum()
}

#[aoc(day1, part1)]
/// Solves part one by applying the calc_mass computation
fn part_one(input: &[i32]) -> i32 {
    input.iter().map(calc_mass).sum()
}

#[aoc(day1, part2)]
/// Solves part two by applying the calc_total_mass computation
fn part_two(input: &[i32]) -> i32 {
    input.iter().map(calc_total_mass).sum()
}

#[cfg(test)]
pub mod tests {
    use super::{calc_mass, calc_total_mass};

    #[test]
    fn day1_mass() {
        assert_eq!(calc_mass(&12), 2);
        assert_eq!(calc_mass(&14), 2);
        assert_eq!(calc_mass(&1969), 654);
        assert_eq!(calc_mass(&100756), 33583);
    }

    #[test]
    fn day1_total_mass() {
        assert_eq!(calc_total_mass(&14), 2);
        assert_eq!(calc_total_mass(&1969), 966);
        assert_eq!(calc_total_mass(&100756), 50346);
    }
}
