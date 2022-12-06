// use std::io;
mod day1;
mod day2;

fn main() {
    // println!("Enter day:");
    // let mut day = String::new();
    // io::stdin()
    //     .read_line(&mut day)
    //     .expect("Failed to read input");

    // println!("Running day {} solution", day.trim());

    day1::day1();
    day2::day2();
}