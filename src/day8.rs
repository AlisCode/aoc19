#[aoc_generator(day8)]
fn generator_input(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).expect("Failed to parse u32")).collect()
} 

#[derive(Debug)]
struct LayerInfo {
    zeroes: usize,
    pub ones: usize,
    pub twos: usize,
} 

impl LayerInfo {
    pub fn compare(a: &Self, b: &Self) -> std::cmp::Ordering {
        a.zeroes.cmp(&b.zeroes)
    }

    pub fn checksum(self) -> usize {
        self.ones * self.twos
    }
}

impl<'a, T: Iterator<Item=&'a u32>> From<T> for LayerInfo {
    fn from(t: T) -> Self {
        let mut zeroes = 0;
        let mut ones = 0;
        let mut twos = 0;
        t.for_each(|x| {
            match x {
                0 => zeroes += 1,
                1 => ones += 1,
                2 => twos += 1,
                _ => unimplemented!(),
            }
        });
        LayerInfo {
            zeroes,
            ones,
            twos
        }
    }
}


fn solve_part_one(input: &[u32], wide: usize, tall: usize) -> usize {
    input
    .chunks(wide*tall)
    .map(|w| LayerInfo::from(w.iter()))
    .min_by(LayerInfo::compare)
    .expect("Failed to find layer_info")
    .checksum()
}

fn solve_part_two(input: &[u32], wide: usize, tall: usize) -> String {
    let mut img: Vec<u32> = (0..wide*tall).map(|_| 2).collect();
    input.chunks(wide*tall).for_each(|x| {
        x.iter().enumerate().for_each(|(idx, data)| if img[idx] == 2 { img[idx] = *data; })
    });

    // quick & dirty way of displaying the image
    img.chunks(wide).fold("\n".to_string(), |mut i, val | {
        val.iter().for_each(|x| {
            if *x == 1 {
                i.push_str("*");
            } else {
                i.push_str(" ");
            } 
        });
        i.push_str("\n");
        i
    })
}

const IMG_WIDE: usize = 25;
const IMG_TALL: usize = 6;

#[aoc(day8, part1)]
fn part_one(input: &[u32]) -> usize {
    solve_part_one(input, IMG_WIDE, IMG_TALL)
}

#[aoc(day8, part2)]
fn part_two(input: &[u32]) -> String {
    solve_part_two(input, IMG_WIDE, IMG_TALL)
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, solve_part_one, solve_part_two};
    #[test]
    fn day8_part_one() {
       assert_eq!(solve_part_one(&[1,2,1,0,0,0,0,0,0,0,1,2], 3, 2), 2);
    }

    fn day8_part_two() {
        assert_eq!(solve_part_two(&[0,2,2,2,1,1,2,2,2,2,1,2,0,0,0,0], 3, 2), "\n *\n* ");
    }
}