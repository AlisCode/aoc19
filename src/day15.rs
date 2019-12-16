use crate::intcode_computer::{parse_input, Computer};
use std::collections::HashSet;

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<i64> {
    parse_input(input)
}

type Node = (i32, i32);
struct Explorer {
    computer: Computer,
    pos: Node,
    map: HashSet<Node>,
    end: Option<Node>,
}

fn neighbour_node(node: &Node, direction: i64) -> Node {
    match direction {
        1 => (node.0, node.1 + 1),
        2 => (node.0, node.1 - 1),
        3 => (node.0 - 1, node.1),
        4 => (node.0 + 1, node.1),
        _ => unimplemented!(),
    }
}

fn inv(dir: i64) -> i64 {
    match dir {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => unreachable!(),
    }
}

impl Explorer {
    pub fn new(input: &[i64]) -> Self {
        Explorer {
            computer: Computer::new(input.to_vec()).halt_on_missing_input(),
            pos: (0, 0),
            map: Default::default(),
            end: None,
        }
    }

    pub fn explore_all(&mut self) {
        // When we add a new point to the map, we add the invert to the backtrace.
        let mut back = vec![];
        while let Some(dir) = (1..5)
            .filter(|dir| !self.map.contains(&neighbour_node(&self.pos, *dir)))
            .next()
            .or_else(|| back.pop())
        {
            let new_node = neighbour_node(&self.pos, dir);
            let new = self.map.insert(new_node);
            if new {
                back.push(inv(dir));
            }
            // Try to move
            self.computer.input(dir);
            self.computer.execute();
            let out = self.computer.get_next_output().unwrap();
            if out == 2 {
                self.end = Some(new_node);
                break;
            }
            if out == 1 {
                match dir {
                    1 => self.pos.1 += 1,
                    2 => self.pos.1 -= 1,
                    3 => self.pos.0 -= 1,
                    4 => self.pos.0 += 1,
                    _ => (),
                }
            }
        }
    }

    pub fn step_to_oxygen(&self) -> usize {
        let success = |node: &Node| -> bool { self.end.unwrap() == *node };
        let successors = |node: &Node| -> Vec<Node> {
            (1..5)
                .filter_map(|dir| {
                    let n = neighbour_node(node, dir);
                    if self.map.contains(&n) {
                        Some(n)
                    } else {
                        None
                    }
                })
                .collect()
        };
        pathfinding::directed::bfs::bfs(&(0, 0), successors, success)
            .unwrap()
            .len()
    }

    pub fn viz(&self) -> String {
        let bounds = self.map.iter().fold((0, 0, 0, 0), |mut state, c| {
            state.0 = state.0.min(c.0);
            state.1 = state.1.max(c.0);
            state.2 = state.2.min(c.1);
            state.3 = state.3.max(c.1);
            state
        });

        (bounds.0..=bounds.1)
            .map(move |x| {
                let mut line: String = (bounds.2..=bounds.3)
                    .map(|y| if self.map.contains(&(x, y)) { '*' } else { ' ' })
                    .collect();
                line.push('\n');
                line
            })
            .collect()
    }
}

#[aoc(day15, part1)]
fn part_one(input: &[i64]) -> usize {
    let mut explorer = Explorer::new(input);
    explorer.explore_all();
    //explorer.viz()
    explorer.step_to_oxygen()
}
