use std::cmp::max;

pub struct Turn {
    red: u32,
    green: u32,
    blue: u32,
}

pub struct Game {
    id: u32,
    turns: Vec<Turn>,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let Some((head, tail)) = line.split_once(':') else {
                panic!("unexpected format for line {line}", line = line)
            };
            let game_id = head.split(' ').nth(1).unwrap().parse().unwrap();
            let turns = tail
                .split(';')
                .map(|turn_str| {
                    let mut turn = Turn {
                        red: 0,
                        green: 0,
                        blue: 0,
                    };
                    turn_str.trim().split(',').for_each(|turn_part| {
                        let Some((n, color)) = turn_part.trim().split_once(' ') else {
                            panic!(
                                "unexpected format for turn part {turn_part}",
                                turn_part = turn_part
                            )
                        };
                        let val: u32 = n.parse().unwrap();
                        match color {
                            "red" => turn.red = val,
                            "green" => turn.green = val,
                            "blue" => turn.blue = val,
                            _ => panic!(
                                "unexpected color {color} (not red/green/blue)",
                                color = color
                            ),
                        }
                    });
                    turn
                })
                .collect();

            Game { id: game_id, turns }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Game]) -> u32 {
    input
        .iter()
        .filter(|g| {
            !g.turns
                .iter()
                .any(|turn| turn.red > 12 || turn.green > 13 || turn.blue > 14)
        })
        .map(|g| g.id)
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|g| {
            let mut maxes = Turn {
                red: 0,
                green: 0,
                blue: 0,
            };
            g.turns.iter().for_each(|turn| {
                maxes.red = max(maxes.red, turn.red);
                maxes.green = max(maxes.green, turn.green);
                maxes.blue = max(maxes.blue, turn.blue);
            });
            maxes.red * maxes.blue * maxes.green
        })
        .sum()
}
