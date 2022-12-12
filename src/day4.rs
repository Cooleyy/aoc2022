struct Assignment {
    lower: u32,
    upper: u32
}

impl Assignment {
    fn new(range: &str) -> Self {
        let mut split_range = range.split("-");
        Assignment {
            lower: split_range.next().unwrap().parse().unwrap(),
            upper: split_range.next().unwrap().parse().unwrap(),
        }
    }
}

struct Pair (Assignment, Assignment);

impl Pair {
    fn new(pair: &str) -> Self{
        let mut split_pair = pair.split(",");
        Pair (Assignment::new(split_pair.next().unwrap()), Assignment::new(split_pair.next().unwrap()))
    }

    fn is_fully_contained(&self) -> bool {
        self.0.lower <= self.1.lower && self.0.upper >= self.1.upper || 
        self.1.lower <= self.0.lower && self.1.upper >= self.0.upper
    }

    fn any_overlap(&self) -> bool {
        self.0.lower <= self.1.upper && self.1.lower <= self.0.upper
    }
}

fn part_one(input: &str){
    let mut answer = 0u32;
    for line in input.lines() {
        let pair = Pair::new(line);
        if pair.is_fully_contained() {
            answer += 1;
        }
    }
    println!("{}", answer);
}

fn part_two(input: &str) {
    let mut answer = 0u32;
    for line in input.lines() {
        let pair = Pair::new(line);
        if pair.any_overlap() {
            answer += 1;
        }
    }
    println!("{}", answer);
}

pub fn day4() {
    let _example = include_str!("../files/day4_example.txt");
    let input = include_str!("../files/day4.txt");

    part_one(input.clone());
    part_two(input);
}