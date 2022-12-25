use crate::common::Timer;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test_divisible_by: i64,
    throw_if_true: usize,
    throw_if_false: usize,
    times_inspected: i64,
}

#[derive(Debug, Clone)]
enum Operation {
    Add(i64),
    Subtract(i64),
    Multiply(i64),
    Square(),
}

impl Monkey {
    pub fn remove_first_item(&mut self) -> i64 {
        self.items.remove(0)
    }

    pub fn add_new_item(&mut self, item: i64) {
        self.items.push(item);
    }

    pub fn increment_times_inspected(&mut self) {
        self.times_inspected += 1;
    }
}

pub fn run(timer: &mut Timer) {
    let input = include_str!("./input.txt").lines().collect::<Vec<&str>>();

    let monkeys = parse_monkeys(&input);

    let reduction_factor = monkeys
        .iter()
        .fold(1, |acc, monkey| return acc * monkey.test_divisible_by);

    solve_part_1(&mut monkeys.to_vec(), reduction_factor, timer);
    solve_part_2(&mut monkeys.to_vec(), reduction_factor, timer);
}

fn solve_part_1(monkeys: &mut Vec<Monkey>, reduction_factor: i64, timer: &mut Timer) {
    run_for_number_of_rounds(20, monkeys, reduction_factor, true);

    let result = extract_two_biggest_and_multiply_them(monkeys);

    assert_eq!(result, 55944);
    timer.log("11.1", result);
}

fn solve_part_2(monkeys: &mut Vec<Monkey>, reduction_factor: i64, timer: &mut Timer) {
    run_for_number_of_rounds(10000, monkeys, reduction_factor, false);

    let result = extract_two_biggest_and_multiply_them(monkeys);

    assert_eq!(result, 15117269860);
    timer.log("11.2", result);
}

fn extract_two_biggest_and_multiply_them(monkeys: &mut Vec<Monkey>) -> i64 {
    let mut counts = monkeys
        .iter()
        .map(|m| m.times_inspected)
        .collect::<Vec<i64>>();

    counts.sort_unstable();

    let mut two_best = counts.iter().skip(monkeys.len() - 2);

    return two_best.next().unwrap() * two_best.next().unwrap();
}

fn run_for_number_of_rounds(
    number_of_rounds: i64,
    monkeys: &mut Vec<Monkey>,
    reduction_factor: i64,
    should_reduce_worry: bool,
) {
    for _ in 0..number_of_rounds {
        for monkey_index in 0..monkeys.len() {
            let monkey_items = &monkeys[monkey_index].items;

            for _ in 0..monkey_items.len() {
                monkeys[monkey_index].increment_times_inspected();

                let mut item_worry_level = monkeys[monkey_index].remove_first_item();

                item_worry_level =
                    execute_operation(item_worry_level, &monkeys[monkey_index].operation);

                if should_reduce_worry {
                    item_worry_level = item_worry_level / 3;
                }

                let other_monkey_index =
                    if item_worry_level % monkeys[monkey_index].test_divisible_by == 0 {
                        monkeys[monkey_index].throw_if_true
                    } else {
                        monkeys[monkey_index].throw_if_false
                    };

                // Reduce numbers to make calculations faster
                let item_worry_level = item_worry_level % reduction_factor;

                monkeys[other_monkey_index].add_new_item(item_worry_level);
            }
        }
    }
}

fn execute_operation(value: i64, operation: &Operation) -> i64 {
    return match operation {
        Operation::Add(num) => value + num,
        Operation::Subtract(num) => value - num,
        Operation::Multiply(num) => value * num,
        Operation::Square() => value * value,
    };
}

fn parse_monkeys(input: &Vec<&'static str>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];

    for i in (0..input.len()).step_by(7) {
        let items = input[i + 1]
            .split(": ")
            .into_iter()
            .skip(1)
            .next()
            .unwrap()
            .split(", ")
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut operation_equation = input[i + 2]
            .split("= ")
            .into_iter()
            .skip(1)
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .into_iter();

        let test = input[i + 3]
            .split("by ")
            .into_iter()
            .skip(1)
            .next()
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let throw_if_true = input[i + 4]
            .split("monkey ")
            .into_iter()
            .skip(1)
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let throw_if_false = input[i + 5]
            .split("monkey ")
            .into_iter()
            .skip(1)
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let operation = (
            operation_equation.next().unwrap(),
            operation_equation.next().unwrap(),
            operation_equation.next().unwrap(),
        );

        let monkey = Monkey {
            throw_if_false: throw_if_false,
            throw_if_true: throw_if_true,
            test_divisible_by: test,
            operation: parse_operation(&operation),
            items: items.to_vec(),
            times_inspected: 0,
        };

        monkeys.push(monkey);
    }

    monkeys
}

fn parse_operation(params: &(&str, &str, &str)) -> Operation {
    if params.0 == "old" && params.1 == "*" && params.2 == "old" {
        return Operation::Square();
    }

    if params.1 == "*" {
        let param = params.2.parse::<i64>().unwrap();
        return Operation::Multiply(param);
    }

    if params.1 == "+" {
        let param = params.2.parse::<i64>().unwrap();
        return Operation::Add(param);
    }

    if params.1 == "-" {
        let param = params.2.parse::<i64>().unwrap();
        return Operation::Subtract(param);
    }

    panic!("Shouldn't reach this");
}
