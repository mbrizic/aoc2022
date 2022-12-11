use std::collections::HashSet;

use crate::common::Timer;

enum Direction { Top, Right, Bottom, Left }

pub fn run(timer: &mut Timer) {
    let input = include_str!("./input.txt")
        .lines()
        .map(|line| {
            let mut segments = line.split(" ");

            let direction = segments.next().unwrap();
            let steps = segments.next().unwrap().parse::<i32>().unwrap();

            return (direction, steps);
        })
        .collect::<Vec<(&str, i32)>>();

    solve_part_1(&input, timer);
    // solve_part_2(&input, timer);
}

fn solve_part_1(input: &Vec<(&str, i32)>, timer: &mut Timer) {

}
