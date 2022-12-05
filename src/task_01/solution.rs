use crate::common::{Timer};

pub fn run(timer: &mut Timer) {
    let values = include_str!("./input.txt").lines().collect::<Vec<&str>>();

    solve(&values, timer);
}

fn solve(values: &Vec<&str>, timer: &mut Timer) {
    let mut sums: Vec<i32> = vec!(0, 0, 0);

    let mut acc = 0;

    values.iter().for_each(|value| {
        if value.len() == 0 {
            enter_value_if_bigger(&mut sums, acc, 0);
            acc = 0;
            return;
        }

        let calories = value.parse::<i32>().unwrap();

        acc += calories;
    });

    let biggest = sums[0];

    assert_eq!(biggest, 69626);
    timer.log("01.1", biggest);

    let three_biggest_sum = sums[0] + sums[1] + sums[2];

    assert_eq!(three_biggest_sum, 206780);
    timer.log("01.2", three_biggest_sum);
    
}

fn enter_value_if_bigger(sums: &mut Vec<i32>, new_value: i32, index_to_check: usize) {
    if index_to_check > 2 {
        return ();
    }

    let current_value = sums[index_to_check];

    if new_value > current_value {
        sums[index_to_check] = new_value;
        enter_value_if_bigger(sums, current_value, index_to_check + 1);
    } else {
        enter_value_if_bigger(sums, new_value, index_to_check + 1);
    }

    return ();
}