fn part_one(input: &str) {
    let mut score: u32 = 0;
    for round in input.lines() {
        if round == "" {
            //end of file
            break;
        }
        let round_chars: Vec<char> = round.chars().collect();
        let their_shape: char = round_chars[0];
        let your_shape: char = round_chars[2];
        score += get_shape_score(your_shape);
        score += get_result(your_shape, their_shape);
    }
    println!("{}", score);
}

fn part_two(input: &str) {
    println!("answer");
}

fn get_shape_score(shape: char) -> u32 {
    match shape {
        'X' => 1, //ROCK
        'Y' => 2, //PAPER
        'Z' => 3, //SCISSORS
        _ => panic!("not a shape: {}", shape)
    }
}

fn get_result(you: char, them: char) -> u32 {
    match you {
        'X' => match them {//ROCK
            'B' => 0,//PAPER
            'A' => 3,
            'C' => 6,//SCISSORS
            _ => panic!("not a shape: {}", them)
        },
        'Y' => match them {//PAPER
            'C' => 0,//SCISSORS
            'B' => 3, 
            'A' => 6,//ROCK
            _ => panic!("not a shape: {}", them)
        },
        'Z' => match them {//SCISSORS
            'A' => 0,//ROCK
            'B' => 6,//PAPER
            'C' => 3,
            _ => panic!("not a shape: {}", them)
        }
        _ => panic!("not a shape: {}", you)
    }
}

pub fn day2() {
    let _example = include_str!("../files/day2_example.txt");
    let input = include_str!("../files/day2.txt");

    part_one(input);
    part_two(input);
}
