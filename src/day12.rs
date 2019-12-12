use recap::Recap;
use serde::Deserialize;
use std::str::FromStr;
use std::collections::HashSet;

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Vec<Vec3> {
    input.lines().map(|l| Vec3::from_str(l).unwrap()).collect()
}

#[derive(Deserialize, Recap, Debug, Clone, Hash, PartialEq, Eq)]
#[recap(regex = r#"<x=(?P<x>(-?)(\d*)), y=(?P<y>(-?)(\d*)), z=(?P<z>(-?)(\d*))>"#)]
pub struct Vec3 {
    pub x: i32,
    pub y: i32, 
    pub z: i32,
}

impl Vec3 {
    pub fn new() -> Self {
        Vec3 {
            x: 0,
            y: 0,
            z: 0,
        }
    }

    pub fn sum(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    pub fn add(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

pub struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    pub fn new_from_pos(pos: Vec3) -> Self {
        Moon {
            pos,
            vel: Vec3::new(),
        }
    }

    pub fn delta_vel(&self, moons: &[Moon]) -> Vec3 {
        moons.iter().fold(Vec3::new(), |mut vel, m| {
            if self.pos.x > m.pos.x { vel.x -= 1 } else if self.pos.x < m.pos.x { vel.x += 1 };
            if self.pos.y > m.pos.y { vel.y -= 1 } else if self.pos.y < m.pos.y { vel.y += 1 };
            if self.pos.z > m.pos.z { vel.z -= 1 } else if self.pos.z < m.pos.z { vel.z += 1 };
            vel
        })
    }

    pub fn move_moon(&mut self) {
        self.pos.add(&self.vel);
    }
}

#[aoc(day12, part1)]
fn part_one(input: &[Vec3]) -> i32 {
    let mut moons: Vec<Moon> = input.iter().map(|pos| Moon::new_from_pos(pos.clone())).collect();

    (0..1000)
    .for_each(|_| {
        let delta_vel: Vec<Vec3> = moons.iter().map(|m| m.delta_vel(&moons)).collect();
        moons.iter_mut().zip(delta_vel.iter()).for_each(|(mut m, dv)| {
            m.vel.add(dv);
            m.move_moon();
        });
    });

    moons.iter().map(|m| m.pos.sum() * m.vel.sum()).sum()
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}


#[aoc(day12, part2)]
fn part_two(input: &[Vec3]) -> i64 {

    let mut moons: Vec<Moon> = input.iter().map(|pos| Moon::new_from_pos(pos.clone())).collect();

    let mut x_cycle = -1;
    let mut y_cycle = -1;
    let mut z_cycle = -1;
    (1..)
    .find(|x| {
        let delta_vel: Vec<Vec3> = moons.iter().map(|m| m.delta_vel(&moons)).collect();
        moons.iter_mut().zip(delta_vel.iter()).for_each(|(mut m, dv)| {
            m.vel.add(dv);
            m.move_moon();
        });

        if moons.iter().all(|m| m.vel.x == 0) && x_cycle == -1 { x_cycle = *x };
        if moons.iter().all(|m| m.vel.y == 0) && y_cycle == -1 { y_cycle = *x };
        if moons.iter().all(|m| m.vel.z == 0) && z_cycle == -1 { z_cycle = *x };
        x_cycle != -1 && y_cycle != -1 && z_cycle != -1 
    });
    lcm(lcm(x_cycle, y_cycle), z_cycle) * 2 
}


#[cfg(test)]
pub mod tests {
    use super::{input_generator, part_two, Vec3};
    use std::str::FromStr;

    #[test]
    fn day12_parse_vec3() {
        let vec = Vec3::from_str("<x=5, y=13, z=-3>").expect("Failed to parse Vec3");
        assert_eq!(vec.x, 5);
        assert_eq!(vec.y, 13);
        assert_eq!(vec.z, -3);
    }

    #[test]
    fn day12_part_two() {
        let input = "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>";
        let pos = input_generator(input);
        assert_eq!(part_two(&pos), 2772);
    }
}