mod task_01;
mod task_02;
mod task_03;
mod task_04;
mod task_05;
mod task_06;
mod task_07;
mod task_08;

pub mod common;

fn main() {

    println!("| Task \t| Time \t\t| Answer");
    println!("|-------|---------------|---------------");

    let mut timer = common::Timer::new();
    
    task_01::solution::run(&mut timer);
    task_02::solution::run(&mut timer);
    task_03::solution::run(&mut timer);
    task_04::solution::run(&mut timer);
    task_05::solution::run(&mut timer);
    task_06::solution::run(&mut timer);
    task_07::solution::run(&mut timer);
    task_08::solution::run(&mut timer);

    println!("\n> Total: {:.2?}", timer.start_time.elapsed());
}
