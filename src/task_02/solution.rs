use std::collections::HashMap;

use crate::common::benchmark;

pub fn run() {
    benchmark("02.1", &solve_first_part);
    benchmark("02.2", &solve_second_part);
}

const DRAW: i32 = 3;
const WIN: i32 = 6;

fn solve_first_part() -> i64 {
    let lines = include_str!("./input.txt")
        .lines()
        .map(|value| value.split(' ').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut total_score = 0;

    let points: HashMap<&str, i32> = HashMap::from([
        ("A", 1), // ROCK
        ("B", 2), // PAPER
        ("C", 3), // SCISSORS
        ("X", 1), // ROCK
        ("Y", 2), // PAPER
        ("Z", 3), // SCISSORS
    ]);

    lines.iter().for_each(|value| {
        let their_call = value.get(0).unwrap();
        let your_call = value.get(1).unwrap();

        let their_points = *points.get(their_call).unwrap();
        let your_points = *points.get(your_call).unwrap();

        let points_difference = your_points - their_points;

        let score = your_points
            + if points_difference == 0 {
                DRAW
            } else if points_difference == 1 || points_difference == -2 {
                WIN
            } else {
                0
            };

        total_score += score;
    });

    assert_eq!(total_score, 13675);

    return total_score as i64;
}

fn solve_second_part() -> i64 {
    let lines = include_str!("./input.txt")
        .lines()
        .map(|value| value.split(' ').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut total_score = 0;

    let points: HashMap<&str, i32> = HashMap::from([
        ("A", 1), // ROCK
        ("B", 2), // PAPER
        ("C", 3), // SCISSORS
    ]);

    lines.iter().for_each(|value| {
        let their_call = value.get(0).unwrap();
        let your_call = *value.get(1).unwrap();

        let their_points = *points.get(their_call).unwrap();
        
        let score = if your_call == "Y" {
            their_points + DRAW
        } else if your_call == "Z" {
            get_points_for_hand_beating_this_one(their_points) + WIN
        } else {
            get_points_for_hand_losing_to_this_one(their_points)
        };

        total_score += score;
    });

    assert_eq!(total_score, 14184);

    return total_score as i64;
}

fn get_points_for_hand_beating_this_one(this_one: i32) -> i32 {
    if this_one == 3 {
        return 1;
    } else {
        return this_one + 1;
    }
}

fn get_points_for_hand_losing_to_this_one(this_one: i32) -> i32 {
    if this_one == 1 {
        return 3;
    } else {
        return this_one - 1;
    }
}