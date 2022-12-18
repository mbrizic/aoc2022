use std::collections::HashMap;

use crate::common::Timer;

pub fn run(timer: &mut Timer) {
    let input = include_str!("./input.txt")
        .lines()
        .map(|line| {
            let mut segments = line.split(" ");

            let command = segments.next().unwrap();
            let value = segments.next().unwrap_or("");

            return (command, value);
        })
        .collect::<Vec<(&str, &str)>>();

    solve(&input, timer);
}

fn solve(input: &Vec<(&str, &str)>, timer: &mut Timer) {
    let mut results: HashMap<i32, i32> = HashMap::new();

    let mut cycle = 0;
    let mut register = 1;

    results.insert(cycle, register);
    draw_pixel(cycle, register);

    for (command, value) in input {
        cycle += 1;

        draw_pixel(cycle, register);
        results.insert(cycle, register);

        if command.starts_with("addx") {
            let parsed = value.parse::<i32>().unwrap();
            cycle += 1;
            register += parsed;

            draw_pixel(cycle, register);
            results.insert(cycle, register);
        }
    }

    let result = [20, 60, 100, 140, 180, 220].iter().fold(0, |acc, index| {
        let value = results.get(&(index - 1)).unwrap();
        acc + (index * value)
    });

    assert_eq!(result, 14520);
    timer.log("10.1", result);
    timer.log("10.2", "PZBGZEJB");
}

fn draw_pixel(_cycle: i32, _value: i32) {
    // let diff = cycle % 40;

    // if diff == 0 {
    //     print!("\n");
    // }

    // if diff == value || diff == value - 1 || diff == value + 1 {
    //     print!("#");
    // } else {
    //     print!(".");
    // }
}
