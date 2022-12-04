use crate::common::benchmark;

pub fn run() {
    benchmark("04.1", &solve_first_part);
    benchmark("04.2", &solve_second_part);
}

fn solve_first_part() -> i64 {
    let lines = include_str!("./input.txt")
        .lines()
        .map(|line| line.split(",").collect::<Vec<&str>>())
        .map(|assignments| {
            return assignments.iter().map(|assignment| {
                return assignment.split("-")
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
            }).collect::<Vec<Vec<u32>>>();
        }).collect::<Vec<Vec<Vec<u32>>>>();

    let mut counter = 0;

    for line in lines {
        let first_pair = line.get(0).unwrap();
        let second_pair = line.get(1).unwrap();

        let first_start = first_pair.get(0).unwrap();
        let first_end = first_pair.get(1).unwrap();
        let second_start = second_pair.get(0).unwrap();
        let second_end = second_pair.get(1).unwrap();

        if first_start <= second_start && first_end >= second_end {
            counter += 1;
        } else if first_start >= second_start && first_end <= second_end {
            counter += 1;
        }
    }

    assert_eq!(counter, 540);

    return counter as i64;
}


fn solve_second_part() -> i64 {
    let lines = include_str!("./input.txt")
        .lines()
        .map(|line| line.split(",").collect::<Vec<&str>>())
        .map(|assignments| {
            return assignments.iter().map(|assignment| {
                return assignment.split("-")
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
            }).collect::<Vec<Vec<u32>>>();
        }).collect::<Vec<Vec<Vec<u32>>>>();

    let mut counter = 0;

    for line in lines {
        let first_pair = line.get(0).unwrap();
        let second_pair = line.get(1).unwrap();

        let first_start = *first_pair.get(0).unwrap();
        let first_end = *first_pair.get(1).unwrap();
        let second_start = *second_pair.get(0).unwrap();
        let second_end = *second_pair.get(1).unwrap();

        let first_range = first_start..=first_end;
        let second_range = second_start..=second_end;

        if first_range.contains(&second_start) || first_range.contains(&second_end) {
            counter += 1;
            continue;
        }

        if second_range.contains(&first_start) || second_range.contains(&first_end) {
            counter += 1;
        }
    }

    assert_eq!(counter, 872);

    return counter as i64;
}