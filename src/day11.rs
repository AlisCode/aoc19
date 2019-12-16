use crate::intcode_computer::{parse_input, Computer};
use std::collections::{HashMap, HashSet};

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Vec<i64> {
    parse_input(input)
}

#[derive(Debug)]
pub struct PaintingRobot {
    pub computer: Computer,
    // 0 : up
    // 1 : right
    // 2 : down
    // 3 : left
    pub direction: i64,
    pub position: (i32, i32),
    pub painted: HashSet<(i32, i32)>,
}

impl PaintingRobot {
    pub fn execute(&mut self, map: &mut HashMap<(i32, i32), i64>) {
        loop {
            self.computer.execute();
            let paint_to = self.computer.get_next_output().unwrap();
            let dir = self.computer.get_next_output().unwrap();

            self.step(map, paint_to, dir);

            if self.computer.halted() {
                break;
            }

            let input = map.get(&self.position).unwrap_or(&0);
            self.computer.input(*input);
        }
    }

    pub fn step(&mut self, map: &mut HashMap<(i32, i32), i64>, paint_to: i64, dir: i64) {
        // handles painting
        if !map.contains_key(&self.position) {
            map.insert(self.position.clone(), paint_to);
            if paint_to == 1 {
                self.painted.insert(self.position.clone());
            }
        } else {
            let entry = map.get_mut(&self.position).unwrap();
            if *entry != paint_to {
                self.painted.insert(self.position.clone());
            }
            *entry = paint_to;
        }

        // handles rotating
        let dir = if dir == 0 { -1 } else { dir };
        self.direction = (self.direction + dir).rem_euclid(4);

        // handles moving
        match self.direction {
            0 => self.position.1 += 1,
            1 => self.position.0 += 1,
            2 => self.position.1 -= 1,
            3 => self.position.0 -= 1,
            _ => (),
        }
    }
}

#[aoc(day11, part1)]
fn part_one(input: &[i64]) -> usize {
    let mut map: HashMap<(i32, i32), i64> = Default::default();
    let mut robot = PaintingRobot {
        computer: Computer::new(input.to_vec())
            .halt_on_missing_input()
            .set_available_memory(1500),
        direction: 0,
        position: (0, 0),
        painted: Default::default(),
    };
    robot.computer.input(0);
    robot.execute(&mut map);
    robot.painted.len()
}

#[aoc(day11, part2)]
fn part_two(input: &[i64]) -> String {
    let mut map: HashMap<(i32, i32), i64> = Default::default();
    map.insert((0, 0), 1);
    let mut robot = PaintingRobot {
        computer: Computer::new(input.to_vec())
            .halt_on_missing_input()
            .set_available_memory(1500),
        direction: 0,
        position: (0, 0),
        painted: Default::default(),
    };
    robot.computer.input(1);
    robot.execute(&mut map);

    let bounds = map
        .iter()
        .filter(|(k, v)| **v == 1)
        .fold((0, 0, 0, 0), |mut state, (k, v)| {
            state.0 = state.0.min(k.0);
            state.1 = state.1.max(k.0);
            state.2 = state.2.min(k.1);
            state.3 = state.3.max(k.1);
            state
        });

    (bounds.0..=bounds.1)
        .map(move |x| {
            let mut line: String = (bounds.2..=bounds.3)
                .map(|y| {
                    let val = map.get(&(x, y)).unwrap_or(&0);
                    if *val == 1 {
                        '█'
                    } else {
                        ' '
                    }
                })
                .collect();
            line.push('\n');
            line
        })
        .collect()
}

#[cfg(test)]
pub mod tests {

    use super::PaintingRobot;
    use crate::intcode_computer::Computer;
    use std::collections::HashMap;

    #[test]
    fn day11_part_one() {
        let mut map: HashMap<(i32, i32), i64> = Default::default();
        let mut robot = PaintingRobot {
            computer: Computer::new(vec![]),
            direction: 0,
            position: (0, 0),
            painted: Default::default(),
        };

        robot.step(&mut map, 1, 0);
        assert_eq!(robot.painted.len(), 1);
        assert_eq!(robot.direction, 3);
        assert_eq!(robot.position, (-1, 0));
        robot.step(&mut map, 0, 0);
        assert_eq!(robot.painted.len(), 1);
        assert_eq!(robot.direction, 2);
        assert_eq!(robot.position, (-1, -1));
        robot.step(&mut map, 1, 0);
        assert_eq!(robot.painted.len(), 2);
        assert_eq!(robot.direction, 1);
        assert_eq!(robot.position, (0, -1));
        robot.step(&mut map, 1, 0);
        assert_eq!(robot.painted.len(), 3);
        assert_eq!(robot.direction, 0);
        assert_eq!(robot.position, (0, 0));
    }
}
