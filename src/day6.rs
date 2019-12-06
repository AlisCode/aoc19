use std::str::FromStr;

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

#[aoc(day6, part1)]
fn part_one(input: &[OrbitRelation]) -> u32 {
    let mut map: std::collections::HashMap<String, Vec<String>> = Default::default();
    input.iter().for_each(|or| {
        let entry = map.entry(or.parent.clone()).or_insert(vec![]);
        entry.push(or.ident.clone());
    });

    calc_weight(&map, "COM", 0)
}

#[cfg(test)]
pub mod tests {
    use super::{input_generator, part_one};
    #[test]
    fn day6_part_one() {
        let input = input_generator("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
        assert_eq!(part_one(&input), 42);
    }
}
