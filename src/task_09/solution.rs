use std::collections::HashSet;

use crate::common::Timer;

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
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    let mut head_x = 0;
    let mut head_y = 0;

    let mut tail_x = 0;
    let mut tail_y = 0;

    for (direction, steps) in input {
        let moves = match *direction {
            "D" => (0, -1),
            "U" => (0, 1),
            "R" => (1, 0),
            "L" => (-1, 0),
            _ => panic!("Unexpected input"),
        };

        for _ in 0..*steps {
            head_x += moves.0;
            head_y += moves.1;

            let mut x_diff = head_x - tail_x;
            let mut y_diff = head_y - tail_y;

            if (-1..2).contains(&x_diff) && (-1..2).contains(&y_diff) {
                continue;
            } else if x_diff != 0 && y_diff == 0 {
                tail_x += x_diff / 2;
            } else if y_diff != 0 && x_diff == 0 {
                tail_y += y_diff / 2;
            } else if y_diff != 0 && x_diff != 0 {
                if y_diff == 2 || y_diff == -2 {
                    y_diff = y_diff / 2;
                }
                if x_diff == 2 || x_diff == -2 {
                    x_diff = x_diff / 2;
                }

                tail_y += y_diff;
                tail_x += x_diff;
            } else {
                panic!("Shouldn't reach this: {x_diff}, {y_diff}");
            }

            visited.insert((tail_x, tail_y));
        }
    }

    let result = visited.len();

    assert_eq!(result, 5878);
    timer.log("09.1", result)
}
