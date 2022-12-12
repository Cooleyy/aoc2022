use std::{iter::Rev, str::Lines};

fn build_stacks(mut input: Rev<Lines>) -> Vec<Vec<char>> {
    let line_length = input.next().unwrap().len();
    let num_stacks = line_length/3;
    
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(num_stacks);
    for _ in 0..num_stacks {
        stacks.push(Vec::new())
    }
    
    for line in input{
        let chars: Vec<char> = line.chars().collect();
        for (stack, cr8) in (1..line_length-1).step_by(4).enumerate() {
            if chars[cr8] != ' ' {
                stacks[stack].push(chars[cr8]);
            }
        }
    }
    
    stacks
}

fn print_answer(stacks: Vec<Vec<char>>) {
    let mut answer = String::new();
    for stack in stacks {
        if stack.len() == 0 {
            answer.push(' ');
        }
        else {
            answer.push(stack[stack.len() -1]);
        }
    }
    println!("{}", answer);
}

fn part_one(input: &str){
    let mut input_split = input.split("\n\n");
    let stacks_input = input_split.next().unwrap().lines().rev();
    let moves = input_split.next().unwrap();

    let mut stacks = build_stacks(stacks_input);
    
    for mve in moves.lines() {
        let move_split: Vec<&str> = mve.split(" ").collect();
        let amount= move_split[1].parse::<usize>().unwrap();
        let from = move_split[3].parse::<usize>().unwrap()-1;
        let to = move_split[5].parse::<usize>().unwrap()-1;

        for _ in 0..amount {
            let taken = stacks[from].pop().unwrap();
            stacks[to].push(taken);
        }
    }

    print_answer(stacks);
}

fn part_two(input: &str) {
    let mut input_split = input.split("\n\n");
    let stacks_input = input_split.next().unwrap().lines().rev();
    let moves = input_split.next().unwrap();

    let mut stacks = build_stacks(stacks_input);

    for mve in moves.lines() {
        let move_split: Vec<&str> = mve.split(" ").collect();
        let amount= move_split[1].parse::<usize>().unwrap();
        let from = move_split[3].parse::<usize>().unwrap()-1;
        let to = move_split[5].parse::<usize>().unwrap()-1;

        let new_length = stacks[from].len() - amount;
        let taken = stacks[from].split_off(new_length);
        for cr8 in taken {
            stacks[to].push(cr8);
        }
    }

    print_answer(stacks);
}

pub fn day5() {
    let _example  = include_str!("../files/day5_example.txt");
    let input = include_str!("../files/day5.txt");

    part_one(input.clone());
    part_two(input);
}