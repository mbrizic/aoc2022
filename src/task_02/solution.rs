use std::collections::HashMap;

use crate::common::benchmark;

pub fn run() {
    benchmark("02.1", &solve_first_part);
    benchmark("02.2", &solve_second_part);
}

const LOSS: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

fn solve_first_part() -> i64 {
    let lines = include_str!("./input.txt")
        .lines()
        .collect::<Vec<&str>>();

    let mut total_score = 0;

    let combination_points_map: HashMap<&str, i32> = HashMap::from([
        ("A X", DRAW + ROCK),
        ("A Y", WIN + PAPER),
        ("A Z", LOSS + SCISSORS),
        ("B X", LOSS + ROCK),
        ("B Y", DRAW + PAPER),
        ("B Z", WIN + SCISSORS),
        ("C X", WIN + ROCK),
        ("C Y", LOSS + PAPER),
        ("C Z", DRAW + SCISSORS),
    ]);

    lines.iter().for_each(|value| {
        total_score += combination_points_map.get(value).unwrap();
    });

    assert_eq!(total_score, 13675);

    return total_score as i64;
}

fn solve_second_part() -> i64 {
    let lines = include_str!("./input.txt")
        .lines()
        .collect::<Vec<&str>>();

    let mut total_score = 0;

    let combination_points_map: HashMap<&str, i32> = HashMap::from([
        ("A X", LOSS + SCISSORS),
        ("A Y", DRAW + ROCK),
        ("A Z", WIN + PAPER),
        ("B X", LOSS + ROCK),
        ("B Y", DRAW + PAPER),
        ("B Z", WIN + SCISSORS),
        ("C X", LOSS + PAPER),
        ("C Y", DRAW + SCISSORS),
        ("C Z", WIN + ROCK),
    ]);

    lines.iter().for_each(|value| {
        total_score += combination_points_map.get(value).unwrap();
    });

    assert_eq!(total_score, 14184);

    return total_score as i64;
}