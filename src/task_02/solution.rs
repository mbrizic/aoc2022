use std::collections::HashMap;

use crate::common::benchmark;

pub fn run() {
    benchmark("02.1", &solve_first_part);
    // benchmark("02.2", &solve_second_part);
}



fn solve_first_part() -> i64 {
    let lines = include_str!("./test.txt").lines().collect::<Vec<&str>>();

    let score = 0;

    let points: HashMap<&str, i32> = HashMap::from([
        ("A", 1), // ROCK
        ("B", 2), // PAPER
        ("C", 3) // SCISSORS
    ]);

    lines.iter().for_each(|value| {
        let you = value.get(..1).unwrap();
        let them = value.get(2..).unwrap();

        println!("{:?} {:?}", you, them);
    });

    assert_eq!(score, 69626);

    return score as i64;
}

fn solve_second_part() -> i64 {
    // let values = include_str!("./input.txt").lines().collect::<Vec<&str>>();

    // let mut sums: Vec<i32> = vec!(0, 0, 0);

    // let mut acc = 0;

    // values.iter().for_each(|value| {
    //     if value.len() == 0 {
    //         enter_value_if_bigger(&mut sums, acc, 0);
    //         acc = 0;
    //         return;
    //     }

    //     let calories = value.parse::<i32>().unwrap();

    //     acc += calories;
    // });

    // let max = sums[0] + sums[1] + sums[2];

    // assert_eq!(max, 206780);

    return 1 as i64;
}