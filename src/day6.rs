use std::collections::HashSet;

fn find_sequence(input: &str, size: usize) -> usize {
    for i in 0..input.len() {
        let sequence: HashSet<char> = input[i..i+size].chars().collect();
        if sequence.len() == size {
            return i+size
        }
    }
    0
}

fn part_one(input: &str) {
    println!("{}", find_sequence(input, 4));
}

fn part_two(input: &str) {
    println!("{}", find_sequence(input, 14));
}

pub fn day6() {
    let _example  = include_str!("../files/day6_example.txt");
    let input = include_str!("../files/day6.txt");

    part_one(input.clone());
    part_two(input);
}