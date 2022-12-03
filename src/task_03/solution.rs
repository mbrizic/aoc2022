use std::{collections::{HashMap}};

use crate::common::benchmark;

pub fn run() {
    benchmark("03.1", &solve_first_part);
    benchmark("03.2", &solve_second_part);
}

fn solve_first_part() -> i64 {
    let lines = include_str!("./input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut found_items: Vec<char> = vec![];

    for line in lines {
        let halves = line.split_at(line.len() / 2);

        for char in halves.1 {
            if halves.0.contains(char) {
                found_items.push(*char);
                break;
            }
        }
    }

    let sum = calculate_score_from_chars(found_items);

    assert_eq!(sum, 7917);

    return sum as i64;
}

fn solve_second_part() -> i64 {
    let lines = include_str!("./input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut found: Vec<char> = vec!();

    for group in lines.chunks(3) {
        let line_1 = group.get(0).unwrap();
        let line_2 = group.get(1).unwrap();
        let line_3 = group.get(2).unwrap();

        for char in line_3 {
            if line_1.contains(char) && line_2.contains(char) {
                found.push(*char);
                break;
            }
        }
    }

    let sum = calculate_score_from_chars(found);

    assert_eq!(sum, 2585);

    return sum as i64;
}

fn calculate_score_from_chars(chars: Vec<char>) -> u32 {
    chars.iter().fold(0, |acc, value| {
        let offset = if value.is_uppercase() {
            38
        } else {
            96
        };
        acc + (*value as u32) - offset
    })
}