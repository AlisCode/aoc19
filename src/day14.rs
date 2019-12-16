use recap::Recap;
use serde::Deserialize;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Vec<Recipe> {
    input
        .lines()
        .map(|i| Recipe::from_str(i).unwrap())
        .collect()
}

#[derive(Debug, Deserialize, Eq, Clone, Recap)]
#[recap(regex = r#"(?P<quantity>\d+) (?P<ident>[A-Z]+)"#)]
pub struct Chemical {
    pub quantity: i64,
    pub ident: String,
}

impl Hash for Chemical {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ident.hash(state);
    }
}

impl std::cmp::PartialEq for Chemical {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}

#[derive(Debug)]
pub struct Recipe {
    pub input: Vec<Chemical>,
    pub output: Chemical,
}

impl std::str::FromStr for Recipe {
    type Err = recap::Error;
    fn from_str(input: &str) -> Result<Self, recap::Error> {
        let splitted: Vec<&str> = input.split(" => ").collect();
        let output = Chemical::from_str(splitted[1])?;
        let input: Vec<Chemical> = splitted[0]
            .split(", ")
            .map(|i| Chemical::from_str(i).unwrap())
            .collect();
        Ok(Recipe { input, output })
    }
}

fn ore_requirements(
    recipes: &HashMap<Chemical, Vec<Chemical>>,
    stock: &mut HashMap<String, i64>,
    chemical_ident: &str,
    chemical_quantity: i64,
) -> i64 {
    let available = stock.entry(chemical_ident.into()).or_insert(0);
    let needed = (chemical_quantity - *available).max(0);

    let provided = recipes
        .keys()
        .filter(|r| r.ident == *chemical_ident)
        .next()
        .unwrap()
        .quantity;
    let times = (needed / provided) + if needed % provided > 0 { 1 } else { 0 };
    *available += provided * times - chemical_quantity;

    let chem = Chemical {
        ident: chemical_ident.into(),
        quantity: 0,
    };
    recipes[&chem]
        .iter()
        .map(|c| {
            if c.ident == "ORE" {
                c.quantity * times
            } else {
                ore_requirements(recipes, stock, &c.ident, c.quantity * times)
            }
        })
        .sum()
}

#[aoc(day14, part1)]
fn part_one(recipes: &[Recipe]) -> i64 {
    let recipes_map: HashMap<Chemical, Vec<Chemical>> = recipes
        .iter()
        .map(|r| (r.output.clone(), r.input.clone()))
        .collect();
    ore_requirements(&recipes_map, &mut Default::default(), "FUEL", 1)
}

#[aoc(day14, part2)]
fn part_two(recipes: &[Recipe]) -> i64 {
    /*
    let ore_per_fuel = part_one(recipes);
    let mut extra = Default::default();
    let mut produced = 1;
    let recipes_map: HashMap<Chemical, Vec<Chemical>> = recipes
        .iter()
        .map(|r| (r.output.clone(), r.input.clone()))
        .collect();

    (low..)
        .find(|x| {
            ore_requirements(&recipes_map, &mut Default::default(), "FUEL", *x) >= 1_000_000_000
        })
        .unwrap()
        */
    0
}

#[cfg(test)]
pub mod tests {
    use super::{input_generator, part_one, part_two, Chemical, Recipe};
    use std::collections::HashMap;
    #[test]
    fn day14_parse_recipe() {
        let input = "10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL";
        let input = input_generator(input);
        assert_eq!(input.len(), 6);

        assert_eq!(input[0].output.ident, "A");
        assert_eq!(input[0].output.quantity, 10);
        assert_eq!(input[2].input.len(), 2);
    }

    #[test]
    fn day14_part_one() {
        let input = "10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL";
        let recipes = input_generator(input);
        assert_eq!(part_one(&recipes), 31);

        let input = "9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL";
        let recipes = input_generator(input);
        assert_eq!(part_one(&recipes), 165);

        let input = "157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let recipes = input_generator(input);
        assert_eq!(part_one(&recipes), 13312);

        let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n17 NVRVD, 3 JNWZP => 8 VPVL\n53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n22 VJHF, 37 MNCFX => 5 FWMGM\n139 ORE => 4 NVRVD\n144 ORE => 7 JNWZP\n5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n145 ORE => 6 MNCFX\n1 NVRVD => 8 CXFTF\n1 VJHF, 6 MNCFX => 4 RFSQX\n176 ORE => 6 VJHF";
        let recipes = input_generator(input);
        assert_eq!(part_one(&recipes), 180697);

        let input = "171 ORE => 8 CNZTR\n7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n114 ORE => 4 BHXH\n14 VRPVC => 6 BMBT\n6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n5 BMBT => 4 WPTQ\n189 ORE => 9 KTJDG\n1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n12 VRPVC, 27 CNZTR => 2 XDBXC\n15 KTJDG, 12 BHXH => 5 XCVML\n3 BHXH, 2 VRPVC => 7 MZWV\n121 ORE => 7 VRPVC\n7 XCVML => 6 RJRHP\n5 BHXH, 4 VRPVC => 5 LTCX";
        let recipes = input_generator(input);
        assert_eq!(part_one(&recipes), 2210736);

        let input = "1 ORE => 2 A\n1 A => 1 B\n1 A, 1 B => 1 FUEL";
        let recipes = input_generator(input);
        assert_eq!(part_one(&recipes), 1);
    }

    #[test]
    fn day14_part_two() {
        let input = "157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let recipes = input_generator(input);
        assert_eq!(part_two(&recipes), 82892753);

        let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n17 NVRVD, 3 JNWZP => 8 VPVL\n53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n22 VJHF, 37 MNCFX => 5 FWMGM\n139 ORE => 4 NVRVD\n144 ORE => 7 JNWZP\n5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n145 ORE => 6 MNCFX\n1 NVRVD => 8 CXFTF\n1 VJHF, 6 MNCFX => 4 RFSQX\n176 ORE => 6 VJHF";
        let recipes = input_generator(input);
        assert_eq!(part_two(&recipes), 5586022);

        let input = "171 ORE => 8 CNZTR\n7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n114 ORE => 4 BHXH\n14 VRPVC => 6 BMBT\n6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n5 BMBT => 4 WPTQ\n189 ORE => 9 KTJDG\n1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n12 VRPVC, 27 CNZTR => 2 XDBXC\n15 KTJDG, 12 BHXH => 5 XCVML\n3 BHXH, 2 VRPVC => 7 MZWV\n121 ORE => 7 VRPVC\n7 XCVML => 6 RJRHP\n5 BHXH, 4 VRPVC => 5 LTCX";
        let recipes = input_generator(input);
        assert_eq!(part_two(&recipes), 460664);
    }
}
