use rayon::prelude::*;
use std::collections::HashSet;
use std::collections::VecDeque;

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<(i32, i32)> {
    let mut x = 0;
    let mut y = 0;
    input
        .lines()
        .flat_map(move |l| {
            x = -1;
            let iter = l.chars().filter_map(move |c| {
                x += 1;
                if c == '#' {
                    return Some((x, y));
                }
                None
            });
            y += 1;
            iter
        })
        .collect()
}

// Returns the tuple that allows us to determine whether the asteroids can see each other or not
fn identifier(a: &(i32, i32), b: &(i32, i32)) -> (i32, i32) {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;

    // 1 for top-right
    // 2 for bot-right
    // 3 for bot-left
    // 4 for top-left
    let zone = match (dx, dy) {
        (dx, dy) if dx > 0 && dy >= 0 => 1,
        (dx, dy) if dx >= 0 && dy < 0 => 2,
        (dx, dy) if dx <= 0 && dy > 0 => 4,
        _ => 3,
    };

    let (dx, dy) = match zone {
        2 | 4 => (dx.abs(), dy.abs()),
        _ => (dy.abs(), dx.abs()),
    };

    (zone, dx * 1000 / dy * 1000)
}

pub fn find_best_asteroid(input: &[(i32, i32)]) -> ((i32, i32), usize) {
    input
        .par_iter()
        .map(|p| {
            let len = input
                .iter()
                .filter_map(|pp| {
                    if pp != p {
                        Some(identifier(p, pp))
                    } else {
                        None
                    }
                })
                .collect::<HashSet<(i32, i32)>>()
                .len();
            (p.clone(), len)
        })
        .max_by_key(|(_, visible_asteroids)| *visible_asteroids)
        .unwrap()
}

#[aoc(day10, part1)]
/// O(n²) solution, should theoretically be possible to do O(n log n)
/// because asteroid A sees B <==> B sees A, so we can rule out half of the computation we're doing here
fn part_one(input: &[(i32, i32)]) -> usize {
    find_best_asteroid(input).1
}

#[aoc(day10, part2)]
fn part_two(input: &[(i32, i32)]) -> i32 {
    0
}

#[cfg(test)]
pub mod tests {
    use super::{input_generator, part_one, part_two};

    #[test]
    fn day10_part_one() {
        let input = "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####";
        let input = input_generator(input);
        assert_eq!(part_one(&input), 33);

        let input = "#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.";
        let input = input_generator(input);
        assert_eq!(part_one(&input), 35);

        let input = ".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..";
        let input = input_generator(input);
        assert_eq!(part_one(&input), 41);

        let input = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##";
        let input = input_generator(input);
        assert_eq!(part_one(&input), 210);
    }

    /*
    #[test]
    fn day10_part_two() {
        let input = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##";
        let input = input_generator(input);
        assert_eq!(part_two(&input), 802);
    }
    */
}
