#![feature(iter_array_chunks)]

use std::io;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
// mod day7;

fn main() {
    println!("Enter day:");
    let mut day = String::new();
    io::stdin()
        .read_line(&mut day)
        .expect("Failed to read input");

    day = day.trim().to_string();
    println!("Running day {} solution", day);

    match day.as_str() {
        "1" => day1::day1(),
        "2" => day2::day2(),
        "3" => day3::day3(),
        "4" => day4::day4(),
        "5" => day5::day5(),
        "6" => day6::day6(),
        //"7" => day7::day7(),
        _ => println!("Invalid Day"),
    }
}