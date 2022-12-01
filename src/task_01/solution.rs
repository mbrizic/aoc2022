use crate::common::benchmark;

pub fn run() {
    benchmark("01.1", &solve_first_part);
    benchmark("01.2", &solve_second_part);
}

fn solve_first_part() -> i64 {
    let values = include_str!("./input.txt").lines().collect::<Vec<&str>>();

    let mut max = 0;
    let mut acc = 0;

    values.iter().for_each(|value| {
        if value.len() == 0 {
            if acc > max {
                max = acc;
            }
            acc = 0;
            return;
        }

        let calories = value.parse::<i32>().unwrap();

        acc += calories;
    });

    assert_eq!(max, 69626);

    return max as i64;
}

fn solve_second_part() -> i64 {
    let values = include_str!("./input.txt").lines().collect::<Vec<&str>>();

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

    let max = sums[0] + sums[1] + sums[2];

    assert_eq!(max, 206780);

    return max as i64;
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