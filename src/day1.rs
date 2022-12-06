use std::str::Lines;

fn part_one(input: Lines){
    let mut current_elf = 0;
    let mut biggest_elf = 0;
    let mut elves: Vec<i32> = Vec::new();
    for line in input {
        if line == ""{
            elves.push(current_elf);
            if biggest_elf < current_elf {
                biggest_elf = current_elf;
            }
            current_elf = 0;
        }
        else {
            match &line.parse::<i32>() {
                Ok(n) => current_elf += n,
                Err(_e) => println!("{}", &line),
            }
        }
    }
    println!("{}", biggest_elf);
}

fn part_two(input: Lines) {
    let mut current_elf = 0;
    let mut elves: Vec<i32> = Vec::new();
    for line in input {
        if line == ""{
            elves.push(current_elf);
            current_elf = 0;
        }
        else {
            //current_elf += &line.parse().unwrap();
            match &line.parse::<i32>() {
                Ok(n) => current_elf += n,
                Err(_e) => println!("{}", &line),
            }
        }
    }

    elves.sort();
    let answer: i32 = elves.iter().rev().take(3).sum();
    println!("{}", answer);
}

pub fn day1() {
    let _example  = include_str!("../files/day1_example.txt").lines();
    let input = include_str!("../files/day1.txt").lines();

    part_one(input.clone());
    part_two(input);
}