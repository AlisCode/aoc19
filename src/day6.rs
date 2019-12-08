use std::str::FromStr;
use pathfinding::directed::bfs::bfs;

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<OrbitRelation> {
    input
        .lines()
        .map(|s| OrbitRelation::from_str(s).unwrap())
        .collect()
}

pub struct OrbitRelation {
    pub ident: String,
    pub parent: String,
}

impl FromStr for OrbitRelation {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<OrbitRelation, Self::Err> {
        let parts: Vec<&str> = s.split(")").collect();
        Ok(OrbitRelation {
            ident: parts[1].into(),
            parent: parts[0].into(),
        })
    }
}

fn calc_weight(map: &std::collections::HashMap<String, Vec<String>>, ident: &str, i: u32) -> u32 {
    let node = map.get(ident);
    match node {
        Some(n) => n.iter().map(|n| calc_weight(map, n, i + 1)).sum::<u32>() + i,
        None => i,
    }
}

fn calc_dist(map: &std::collections::HashMap<String, (Vec<String>, Vec<String>)>, start: String, end: String) -> usize {
    let neighbors = |p: &String| -> Vec<String> {
        let s = map.get(p).unwrap();
        s.0.iter().chain(s.1.iter()).cloned().collect()
    }; 
    let success = |s: &String| -> bool { s == &end };
    bfs(&start, neighbors, success).unwrap().len()
}

#[aoc(day6, part1)]
fn part_one(input: &[OrbitRelation]) -> u32 {
    let mut map: std::collections::HashMap<String, Vec<String>> = Default::default();
    input.iter().for_each(|or| {
        let entry = map.entry(or.parent.clone()).or_insert(vec![]);
        entry.push(or.ident.clone());
    });
    
    calc_weight(&map, "COM", 0)
}

#[aoc(day6, part2)]
fn part_two(input: &[OrbitRelation]) -> usize {
    let mut map: std::collections::HashMap<String, (Vec<String>, Vec<String>)> = Default::default();
    input.iter().for_each(|or| {
        let entry = map.entry(or.parent.clone()).or_insert((vec![], vec![]));
        entry.0.push(or.ident.clone());
        let entry = map.entry(or.ident.clone()).or_insert((vec![], vec![]));
        entry.1.push(or.parent.clone());
    });
    
    calc_dist(&map, "YOU".into(), "SAN".into()) - 3
}

#[cfg(test)]
pub mod tests {
    use super::{input_generator, part_one, part_two};
    #[test]
    fn day6_part_one() {
        let input = input_generator("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
        assert_eq!(part_one(&input), 42);
    }

    #[test]
    fn day6_part_two() {
        let input = input_generator("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN");
        assert_eq!(part_two(&input), 4);
    }
}
