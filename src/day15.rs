use crate::intcode_computer::{parse_input, Computer};
use pathfinding::directed::astar;

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<i64> {
    parse_input(input)
}

type Node = ((i32, i32), bool);
struct Explorer {
    computer: Computer,
    pos: Node,
    map: HashSet<Node>,
}

impl Explorer {
    pub fn new(input: &[i64]) -> Self {
        Explorer {
            computer: Computer::new(input.to_vec()),
            pos: ((0, 0), false),
            map: Default::default(),
        }
    }

    pub fn goto(&mut self, node: &Node) {

    }

    pub fn add_neighbours(&mut self) -> bool {
        let mut found_oxygen = false;
        // Add in each direction
        (1..5).for_each(|direction| {
            computer.input(direction);
            computer.execute();
            let out = computer.get_next_output().unwrap();
            if out == 1 || out == 2 {
                // Rollback
                computer.input((direction + 1) % 4 + 1);
                computer.get_next_output().unwrap();
                match direction {
                    1 => self.map.push((((node.0).0, (node.0).1 + 1), out == 2)),
                    2 => self.map.push((((node.0).0 + 1, (node.0).1), out == 2)),
                    3 => self.map.push((((node.0).0, (node.0).1 - 1), out == 2)),
                    4 => self.map.push((((node.0).0 - 1, (node.0).1), out == 2)),
                };
            }
        });
        found_oxygen
    }

}

#[aoc(day15, part1)]
fn part_one(input: &[i64]) -> usize {
    let mut computer = Computer::new(input.to_vec()).halt_on_missing_input();
    let successors = |&node| {
        let mut successors = vec![];

        successors
    };

    let success = |node: &((i32, i32), bool))| node.1;

    0
}
