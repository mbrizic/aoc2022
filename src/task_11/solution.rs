use crate::common::Timer;

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    test_divisible_by: i32,
    throw_if_true: usize,
    throw_if_false: usize,
    times_inspected: i32,
}

#[derive(Debug)]
enum Operation {
    Add(i32),
    Subtract(i32),
    Multiply(i32),
    Square(),
}

impl Monkey {
    pub fn remove_first_item(&mut self) -> i32 {
        self.items.remove(0)
    }

    pub fn add_new_item(&mut self, item: i32) {
        self.items.push(item);
    }

    pub fn increment_times_inspected(&mut self) {
        self.times_inspected += 1;
    }
}

pub fn run(timer: &mut Timer) {
    let input = include_str!("./input.txt").lines().collect::<Vec<&str>>();

    solve_part_1(&input, timer);
}

fn solve_part_1(input: &Vec<&'static str>, timer: &mut Timer) {
    let mut monkeys = parse_monkeys(&input);
    let number_of_rounds = 20;

    for round_index in 0..number_of_rounds {
        for monkey_index in 0..monkeys.len() {
            let monkey_items = &monkeys[monkey_index].items;

            for _ in 0..monkey_items.len() {
                monkeys[monkey_index].increment_times_inspected();

                let item_worry_level = monkeys[monkey_index].remove_first_item();

                let item_worry_level = solve(item_worry_level, &monkeys[monkey_index].operation);
                // needed for first part
                let item_worry_level = item_worry_level / 3;

                let other_monkey_index =
                    if item_worry_level % monkeys[monkey_index].test_divisible_by == 0 {
                        monkeys[monkey_index].throw_if_true
                    } else {
                        monkeys[monkey_index].throw_if_false
                    };

                // // reduce to make calculations faster
                // let item_worry_level = item_worry_level % monkeys[other_monkey_index].test_divisible_by;

                monkeys[other_monkey_index].add_new_item(item_worry_level);
            }
        }

        // println!("--- Round {}", round_index + 1);
        // for monkey in &monkeys {
        //     println!("{}", monkey.times_inspected);
        // }
    }

    let mut counts = monkeys
        .iter()
        .map(|m| m.times_inspected)
        .collect::<Vec<i32>>();

    counts.sort_unstable();

    let mut two_best = counts.iter().skip(monkeys.len() - 2);

    let result = two_best.next().unwrap() * two_best.next().unwrap();

    assert_eq!(result, 55944);
    timer.log("11.1", result);
}

fn solve(value: i32, operation: &Operation) -> i32 {
    return match operation {
        Operation::Add(num) => value + num,
        Operation::Subtract(num) => value - num,
        Operation::Multiply(num) => value * num,
        Operation::Square() => value * value,
        _ => panic!("Unsupported operation"),
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
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

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
            .parse::<i32>()
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

        let mut monkey = Monkey {
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
        let param = params.2.parse::<i32>().unwrap();
        return Operation::Multiply(param);
    }

    if params.1 == "+" {
        let param = params.2.parse::<i32>().unwrap();
        return Operation::Add(param);
    }

    if params.1 == "-" {
        let param = params.2.parse::<i32>().unwrap();
        return Operation::Subtract(param);
    }

    panic!("Shouldn't reach this");
}
