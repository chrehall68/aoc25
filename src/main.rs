use std::env;
mod day1;
mod day2;
mod day3;
mod day4;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("AOC 2025!");
    if args.len() < 2 {
        panic!("Expected the day to run!");
    }
    let day: i32 = args[1].parse().expect("Failed parsing day");
    match day {
        1 => day1::driver(),
        2 => day2::driver(),
        3 => day3::driver(),
        4 => day4::driver(),
        _ => println!("No match!"),
    }
}
