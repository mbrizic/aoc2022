use std::collections::HashSet;

use crate::common::Timer;

#[derive(Debug)]
enum Direction {
    Up, Down, Left, Right,
}

const DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

pub fn run(timer: &mut Timer) {
    let input = include_str!("./input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    solve_part_1(&input, timer);
    solve_part_2(&input, timer);
}

fn solve_part_1(trees: &Vec<Vec<u32>>, timer: &mut Timer) {
    let mut found: HashSet<(usize, usize, u32)> = HashSet::new();

    for i in 0..trees.len() {
        let row = trees.get(0).unwrap();

        for j in 0..row.len() {
            if check_is_visible_from_edge(trees, i, j) {
                found.insert((i, j, trees[i][j]));
            }
        }
    }

    let result = found.len();

    assert_eq!(result, 1733);
    timer.log("08.1", result);
}

fn solve_part_2(trees: &Vec<Vec<u32>>, timer: &mut Timer) {
    let mut max: i32 = 0;

    for i in 1..trees.len() - 1 {
        let row = trees.get(0).unwrap();

        for j in 1..row.len() - 1 {
            let visible = count_visible_neighbours(trees, (i, j));
            if visible > max {
                max = visible;
            }
        }
    }

    let result = max;

    assert_eq!(result, 284648);
    timer.log("08.2", result);
}

fn count_visible_neighbours(trees: &Vec<Vec<u32>>, tree: (usize, usize)) -> i32 {
    return DIRECTIONS.iter().fold(1, |acc, direction| {
        let count = match get_neighbour_in_direction(&trees, &direction, tree) {
            Ok(indexes) => {
                count_bigger_than_neighbour_in_direction(trees, direction, tree, indexes, 1)
            }
            Err(_) => 1,
        };

        acc * count
    });
}

fn count_bigger_than_neighbour_in_direction(
    trees: &Vec<Vec<u32>>,
    direction: &Direction,
    tree_indexes: (usize, usize),
    neighbour_indexes: (usize, usize),
    acc: i32,
) -> i32 {
    let tree = trees[tree_indexes.0][tree_indexes.1];
    let neighbour = trees[neighbour_indexes.0][neighbour_indexes.1];

    if tree <= neighbour {
        return acc;
    }

    let acc = acc + 1;

    return match get_neighbour_in_direction(&trees, &direction, neighbour_indexes) {
        Ok(indexes) => {
            count_bigger_than_neighbour_in_direction(trees, direction, tree_indexes, indexes, acc)
        }
        Err(_) => acc - 1,
    };
}

fn check_is_visible_from_edge(trees: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {
    if i == 0 || j == 0 || i == trees.len() - 1 || j == trees.len() - 1 {
        return true;
    }

    return DIRECTIONS.iter().any(|direction| {
        return match get_neighbour_in_direction(&trees, &direction, (i, j)) {
            Ok(indexes) => is_bigger_than_neighbour_in_direction(trees, direction, (i, j), indexes),
            Err(_) => false,
        };
    });
}

fn is_bigger_than_neighbour_in_direction(
    trees: &Vec<Vec<u32>>,
    direction: &Direction,
    tree: (usize, usize),
    neighbour: (usize, usize),
) -> bool {
    let tree_height = trees[tree.0][tree.1];
    let neighbour_height = trees[neighbour.0][neighbour.1];

    if tree_height <= neighbour_height {
        return false;
    }

    return match get_neighbour_in_direction(&trees, &direction, neighbour) {
        Ok(indexes) => is_bigger_than_neighbour_in_direction(trees, direction, tree, indexes),
        Err(_) => true,
    };
}

fn get_neighbour_in_direction(
    trees: &Vec<Vec<u32>>,
    direction: &Direction,
    tree: (usize, usize),
) -> Result<(usize, usize), &'static str> {
    match direction {
        Direction::Up => {
            if tree.0 > 0 {
                Result::Ok((tree.0 - 1, tree.1))
            } else {
                Result::Err("Can't go further")
            }
        }
        Direction::Down => {
            if tree.0 < trees.len() - 1 {
                Result::Ok((tree.0 + 1, tree.1))
            } else {
                Result::Err("Can't go further")
            }
        }
        Direction::Left => {
            if tree.1 > 0 {
                Result::Ok((tree.0, tree.1 - 1))
            } else {
                Result::Err("Can't go further")
            }
        }
        Direction::Right => {
            if tree.1 < trees.len() - 1 {
                Result::Ok((tree.0, tree.1 + 1))
            } else {
                Result::Err("Can't go further")
            }
        }
    }
}
