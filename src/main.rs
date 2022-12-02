use std::time::Instant;

mod task_01;
mod task_02;

pub mod common;

fn main() {

    println!("| Task \t| Time \t\t| Answer");
    println!("|-------|---------------|---------------");

    let now = Instant::now();
    
    task_01::solution::run();
    task_02::solution::run();

    println!("\n> Total: {:.2?}", now.elapsed());
}
