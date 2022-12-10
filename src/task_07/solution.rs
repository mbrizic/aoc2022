use std::collections::{BTreeMap, HashMap};

use crate::common::Timer;

#[derive(Debug)]
struct Node {
    name: &'static str,
    path: String,
    size: i32,
    depth: usize,
}

impl Node {
    pub fn new(name: &'static str, size: i32, path: &Vec<&str>) -> Node {
        Node {
            name: name,
            size: size,
            path: path.join("/"),
            depth: path.len(),
        }
    }

    pub fn full_path(&self) -> String {
        format!("{}/{}", self.path, self.name)
    }

    pub fn is_dir(&self) -> bool {
        self.size == 0
    }
}

const COMMAND_CD: usize = 1;
const COMMAND_LS: usize = 2;

pub fn run(timer: &mut Timer) {
    let input = include_str!("./input.txt");

    solve(&input, timer);
}

fn solve(input: &'static str, timer: &mut Timer) {
    let mut file_tree: Vec<Node> = vec![];
    let mut active_path: Vec<&str> = vec![""];

    for line in input.lines().skip(1) {
        if line.starts_with("$") {
            let (cmd, param) = parse_command(line);

            match cmd {
                COMMAND_CD => {
                    if param == ".." {
                        active_path.pop();
                    } else {
                        let new_node = Node::new(param, 0, &active_path);

                        file_tree.push(new_node);

                        active_path.push(param);
                    }
                }
                _ => (),
            }
        } else {
            let (size, name) = parse_line_from_output(line);

            let node = Node::new(name, size, &active_path);

            file_tree.push(node);
        }
    }

    file_tree.sort_unstable_by(|a, b| a.full_path().cmp(&b.full_path()));
    file_tree.dedup_by_key(|a| a.full_path());

    let mut groups: BTreeMap<usize, Vec<&Node>> = BTreeMap::new();

    for node in &file_tree {
        if groups.contains_key(&node.depth) {
            groups.get_mut(&node.depth).unwrap().push(node.clone());
        } else {
            groups.insert(node.depth, vec![node.clone()]);
        }
    }

    let mut directory_sizes: HashMap<String, i32> = HashMap::new();

    for (_depth, group) in groups.iter().rev() {
        for item in group {
            if item.is_dir() {
                directory_sizes.insert(
                    item.full_path(),
                    calculate_dir_size(&file_tree, &directory_sizes, &item),
                );
            }
        }
    }

    let root_folder_size = groups.get(&1).unwrap().iter().fold(0, |acc, node| {
        if node.is_dir() {
            let size = directory_sizes.get(&node.full_path()).unwrap();
            acc + size
        } else {
            acc + node.size
        }
    });

    let directory_sizes_below_threshold = directory_sizes.iter().filter(|size| size.1 < &100_000);

    let total_folder_size = &directory_sizes_below_threshold
        .fold(0, |acc, item| acc + directory_sizes.get(item.0).unwrap());

    assert_eq!(*total_folder_size, 1306611);
    timer.log("07.1", *total_folder_size);

    let free_disk_space = 70_000_000 - root_folder_size;
    let space_to_free_up = 30_000_000 - free_disk_space;

    let mut potential_directories = directory_sizes
        .values()
        .filter(|&&size| size > space_to_free_up)
        .collect::<Vec<&i32>>();

    potential_directories.sort_unstable();

    let smallest_dir_to_delete = **potential_directories.get(0).unwrap();

    assert_eq!(smallest_dir_to_delete, 13210366);
    timer.log("07.2", smallest_dir_to_delete);
}

fn calculate_dir_size(
    file_tree: &Vec<Node>,
    directory_sizes: &HashMap<String, i32>,
    dir: &Node,
) -> i32 {
    if dir.size != 0 {
        return dir.size;
    }

    let children = file_tree
        .iter()
        .filter(|node| node.depth == dir.depth + 1 && node.full_path().contains(&dir.full_path()));

    let size = children.fold(0, |acc, node| {
        if node.is_dir() {
            let size = directory_sizes.get(&node.full_path()).unwrap();
            acc + size
        } else {
            acc + node.size
        }
    });

    size
}

fn parse_command(cmd: &str) -> (usize, &str) {
    let mut segments = cmd.split(" ").skip(1); // Skip the "$" terminal input prefix

    let _cmd = segments.next().unwrap();
    let param = segments.next().unwrap_or("");

    if param.is_empty() {
        (COMMAND_LS, "")
    } else {
        (COMMAND_CD, param)
    }
}

fn parse_line_from_output(output: &str) -> (i32, &str) {
    let mut segments = output.split(" ").into_iter();

    let size = segments.next().unwrap().parse::<i32>().unwrap_or(0);
    let name = segments.next().unwrap();

    (size, name)
}
