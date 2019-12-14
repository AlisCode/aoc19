use crate::intcode_computer::{parse_input, Computer};

#[aoc_generator(day13)]
fn input_generator(input: &str) -> Vec<i64> {
    parse_input(input)
}

#[aoc(day13, part1)]
fn part_one(input: &[i64]) -> usize {
    let mut computer = Computer::new(input.to_vec()).set_available_memory(3000);
    computer.execute();

    let mut idx = 0;
    computer
        .get_all_output()
        .filter(|x| {
            let ret = idx == 2 && **x == 2;
            idx += 1;
            idx %= 3;
            ret
        })
        .count()
}

struct GameInfo {
    pub score: i64,
    pub ball: (i64, i64),
    pub paddle: (i64, i64),
    pub game_finished: bool,
}

impl GameInfo {
    fn from_frame(input: Vec<i64>) -> Self {
        let mut ball = (0, 0);
        let mut paddle = (0, 0);
        let mut score = 0;
        let mut game_finished = true;
        input
            .chunks(3)
            .inspect(|i| println!("{}", i[2]))
            .for_each(|i| match i[2] {
                0 | 1 => (),
                2 => {
                    game_finished = false;
                }
                3 => {
                    paddle = (i[0], i[1]);
                }
                4 => {
                    ball = (i[0], i[1]);
                }
                x => {
                    score = x;
                }
            });

        GameInfo {
            score,
            ball,
            paddle,
            game_finished,
        }
    }
}

#[aoc(day13, part2)]
fn part_two(input: &[i64]) -> i64 {
    let mut computer = Computer::new(input.to_vec())
        .set_available_memory(3000)
        .halt_on_missing_input();
    computer.set(0, 2);

    loop {
        computer.execute();
        let frame: Vec<i64> = computer.output.iter().map(|x| *x).collect();
        let game_info = GameInfo::from_frame(frame);
        computer.clear_output();

        if game_info.game_finished {
            return game_info.score;
        }

        let input = if game_info.paddle.0 < game_info.ball.0 {
            1
        } else if game_info.paddle.0 > game_info.ball.0 {
            -1
        } else {
            0
        };
        computer.input(input);
        println!("-----");
    }
}
