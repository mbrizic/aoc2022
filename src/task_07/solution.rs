use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::common::Timer;

#[derive(Debug)]
struct Node {
    name: &'static str,
    size: i32,
    children: HashMap<String, Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(name: &'static str, size: i32) -> Node {
        Node {
            name: name,
            size: size,
            children: HashMap::new(),
        }
    }

    pub fn set_size(&mut self, size: i32) {
        self.size = size;
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Node>>) {
        let path = child.borrow_mut().name.to_string();

        self.children.insert(path, child);
    }
}

pub fn run(timer: &mut Timer) {
    let input = include_str!("./input.txt");

    solve(&input, timer);
}

fn solve(input: &'static str, timer: &mut Timer) {
    let root = Rc::new(RefCell::new(Node::new("/", 0)));

    let mut current_dir = root.clone();
    let mut stack: Vec<Rc<RefCell<Node>>> = vec![current_dir.clone()];

    for line in input.lines().skip(1) {
        let cmd = line.split_ascii_whitespace().collect::<Vec<&str>>();

        if cmd[0] == "$" {
            if cmd[1] == "cd" {
                let dir_name = cmd[2];

                if dir_name == ".." {
                    stack.pop().unwrap();
                    current_dir = stack.last().unwrap().clone();
                } else {
                    let new_node = Rc::new(RefCell::new(Node::new(dir_name, 0)));

                    current_dir.borrow_mut().add_child(new_node.clone());

                    current_dir = new_node.clone();

                    stack.push(new_node.clone());
                }
            }
        } else {
            let size = cmd[0].parse::<i32>().unwrap_or(0);
            let name = cmd[1];

            let new_node = Rc::new(RefCell::new(Node::new(name, size)));

            current_dir.borrow_mut().add_child(new_node.clone());
        }
    }

    let mut directory_sizes_report: Vec<i32> = vec![];

    calculate_total_children_size(&root, &mut directory_sizes_report);

    let total_folders_size = directory_sizes_report
        .iter()
        .filter(|item| *item < &100_000)
        .fold(0, |acc, size| acc + size);

    assert_eq!(total_folders_size, 1306611);
    timer.log("07.1", total_folders_size);

    let free_disk_space = 70_000_000 - root.clone().borrow_mut().size;
    let space_to_free_up = 30_000_000 - free_disk_space;

    let smallest_dir_to_delete_size = *directory_sizes_report
        .iter()
        .filter(|&&size| size > space_to_free_up)
        .min_by(|a, b| a.cmp(b))
        .unwrap();

    assert_eq!(smallest_dir_to_delete_size, 13210366);
    timer.log("07.2", smallest_dir_to_delete_size);
}

fn calculate_total_children_size(rc: &Rc<RefCell<Node>>, report: &mut Vec<i32>) -> i32 {
    let mut node = rc.borrow_mut();

    if node.size > 0 {
        return node.size;
    }

    let size_of_all_children = node.children.iter().fold(0, |acc, (_, item)| {
        acc + calculate_total_children_size(&item.clone(), report)
    });

    node.set_size(size_of_all_children);

    report.push(node.size);

    size_of_all_children
}
