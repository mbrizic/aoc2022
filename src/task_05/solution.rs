use crate::common::{Timer};

const SPACE_KEY_CODE: u32 = 32;

pub fn run(timer: &mut Timer) {
    let segments: Vec<&str> = include_str!("./input.txt").split("\n\n").collect();

    let stacks: Vec<&str> = segments.get(0).unwrap().lines().collect();
    let stacks_height = stacks.len();

    let unformatted_stacks = stacks
        .into_iter()
        .rev()
        .skip(1)
        .map(|line| {
            return line
                .chars()
                .skip(1)
                .into_iter()
                .step_by(4)
                .collect::<Vec<char>>();
        })
        .enumerate()
        .map(|item| item.1)
        .collect::<Vec<Vec<char>>>();

    let mut stacks: Vec<Vec<char>> = vec!();

    for i in 0..stacks_height {
        let mapped: Vec<char> = unformatted_stacks.iter()
            .map(|stack| *stack.get(i).unwrap())
            .filter(|item| *item as u32 != SPACE_KEY_CODE)
            .collect();

        stacks.push(mapped);
    }

    let commands = segments
        .get(1)
        .unwrap()
        .lines()
        .map(|line| {
            line.split(" ")
                .skip(1)
                .step_by(2)
                .map(|item| item.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    solve_first_part(&mut stacks.clone(), &commands, timer);
    solve_second_part(&mut stacks, &commands, timer);
}

fn solve_first_part(stacks: &mut Vec<Vec<char>>, commands: &Vec<Vec<usize>>, timer: &mut Timer) {

    for command in commands {
        let how_many_to_move = *command.get(0).unwrap();
        let stack_to_move_from = *command.get(1).unwrap() - 1;
        let stack_to_move_to = *command.get(2).unwrap() - 1;

        for _ in 0..how_many_to_move { 
            let item_to_move = stacks[stack_to_move_from].pop().unwrap();

            stacks[stack_to_move_to].push(
                item_to_move
            );
        }
    }

    let mut result = String::from("");

    for stack in stacks {
        let first_item = stack.pop().unwrap();

        result += first_item.to_string().as_str();
    }

    assert_eq!(result, "QPJPLMNNR");
    timer.log("05.1", result);

}

fn solve_second_part(stacks: &mut Vec<Vec<char>>, commands: &Vec<Vec<usize>>, timer: &mut Timer) {

    for command in commands {
        let how_many_to_move = *command.get(0).unwrap();
        let stack_to_move_from = *command.get(1).unwrap() - 1;
        let stack_to_move_to = *command.get(2).unwrap() - 1;

        let mut items_to_move: Vec<char> = vec!();

        for _ in 0..how_many_to_move { 
            let item = stacks[stack_to_move_from].pop().unwrap();

            items_to_move.push(item);
        }

        for item in items_to_move.iter().rev() {
            stacks[stack_to_move_to].push(
                *item
            );
        }
    }

    let mut result = String::from("");

    for stack in stacks {
        let first_item = stack.pop().unwrap();

        result += first_item.to_string().as_str();
    }

    assert_eq!(result, "BQDNWJPVJ");
    timer.log("05.2", result);

}
