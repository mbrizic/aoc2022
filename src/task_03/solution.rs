use crate::common::Timer;

pub fn run(timer: &mut Timer) {
    let lines = include_str!("./input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    solve_first_part(&lines, timer);
    solve_second_part(&lines, timer);
}

fn solve_first_part(lines: &Vec<Vec<char>>, timer: &mut Timer) {
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
    timer.log("03.1", sum);

}

fn solve_second_part(lines: &Vec<Vec<char>>, timer: &mut Timer) {
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
    timer.log("03.2", sum);

}

fn calculate_score_from_chars(chars: Vec<char>) -> i32 {
    chars.iter().fold(0, |acc, value| {
        let offset = if value.is_uppercase() {
            38
        } else {
            96
        };
        acc + (*value as i32) - offset
    })
}