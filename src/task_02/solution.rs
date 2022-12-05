use std::collections::HashMap;

use crate::common::Timer;

pub fn run(timer: &mut Timer) {
    let lines = include_str!("./input.txt")
        .lines()
        .collect::<Vec<&str>>();

    solve_first_part(&lines, timer);
    solve_second_part(&lines, timer);
}

const LOSS: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

fn solve_first_part(lines: &Vec<&str>, timer: &mut Timer) {
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
    timer.log("02.1", total_score);

}

fn solve_second_part(lines: &Vec<&str>, timer: &mut Timer) {
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
    timer.log("02.2", total_score);
    
}