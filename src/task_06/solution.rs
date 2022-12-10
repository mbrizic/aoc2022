use crate::common::Timer;

pub fn run(timer: &mut Timer) {
    let input = include_str!("./input.txt");

    solve_first_part(&input, timer);
    solve_second_part(&input, timer);
}

fn solve_first_part(input: &str, timer: &mut Timer) {
    let result = solve(&input, 4);

    assert_eq!(result, 1647);
    timer.log("06.1", result);
}

fn solve_second_part(input: &str, timer: &mut Timer) {
    let result = solve(&input, 14);

    assert_eq!(result, 2447);
    timer.log("06.2", result);
}

fn solve(input: &str, step_size: usize) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    let step_size: usize = step_size - 1;

    let mut result = 0;
    let mut counter: usize = step_size;

    'outer: while counter < chars.len() {
        for index in 1..=step_size {
            // first try to find items where we can skip through the array fast
            if chars.get(counter).unwrap() == chars.get(counter - index).unwrap() {
                let how_much_to_skip = step_size - index + 1;
                counter += how_much_to_skip;
                continue 'outer;
            }
        }

        for i in 1..step_size {
            for j in i+1..=step_size {
                if chars.get(counter - i).unwrap() == chars.get(counter - j).unwrap() {
                    counter += 1;
                    continue 'outer;
                }
            }
        }

        result = counter + 1;
        break;
    }

    return result;
}
