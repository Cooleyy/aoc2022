use std::io;
mod day1;
mod day2;

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
        _ => println!("Invalid Day"),
    }
}