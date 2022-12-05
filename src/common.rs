use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::time::Duration;
use std::time::Instant;

pub fn read_file(path: &str) -> std::io::Result<Vec<i32>> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let lines = contents
        .lines()
        .map(|a| a.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    return Ok(lines);
}

pub fn print_results(task_number: &str, result: &String, time: Duration) {
    println!("| {}\t| {:.2?}\t| {}", task_number, time, result);
}

pub struct Timer {
    pub start_time: Instant,
    pub last_snapshot: Instant
}

impl Timer {
    pub fn new() -> Timer {
        let start_time = Instant::now();

        return Timer{ start_time: start_time, last_snapshot: start_time };
    }

    pub fn log(&mut self, task_number: &str, result: impl Into<i64>) {
        let result: i64 = result.into();

        println!("| {}\t| {:.2?}\t| {}", task_number, self.last_snapshot.elapsed(), result);

        self.last_snapshot = Instant::now();
    } 
}