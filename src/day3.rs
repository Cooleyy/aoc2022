#[derive(Debug)]
struct Backpack<'a> {
    total_contents: &'a str,
    compartment1: &'a str,
    compartment2: &'a str,
}

impl Backpack<'_> {
    fn find_duplicate (&self) -> Option<char> {
        for item in self.compartment1.chars() {
            if self.compartment2.contains(item){
                return Some(item)
            }
        }
        None
    }
}

fn build_backpack(contents: &str) -> Backpack {
    let compartment_size = contents.len()/2;
    return Backpack {
        total_contents: contents,
        compartment1: &contents[..compartment_size],
        compartment2: &contents[compartment_size..],
    };
}

struct Group<'a> (Backpack<'a>, Backpack<'a>, Backpack<'a>);

impl Group<'_> {
    fn find_common_item(&self) -> Option<char> {
        for item in self.0.total_contents.chars() {
            if self.1.total_contents.contains(item) && self.2.total_contents.contains(item){
                return Some(item)
            }
        }
        None
    }
}

fn item_to_priority(item: char) -> Option<u32> {
    if item.is_lowercase() {
        return Some(item as u32 - 96)
    }
    else if item.is_uppercase() {
        return Some(item as u32 - 38)
    }
    else {
        None
    }
}

fn part_one(input: &str) {
    let mut answer: u32 = 0u32;
    for pack in input.lines() {
        if pack == "" { break; }
        let backpack = build_backpack(&pack);
        
        match backpack.find_duplicate() {
            None => continue,
            Some(duplicate) => match item_to_priority(duplicate) {
                None => continue,
                Some(priority) => answer += priority
            },
        };
    }
    println!("{}", answer)
}

fn part_two(input: &str) {
    let mut lines = input.lines().peekable();
    let mut answer: u32 = 0;

    while lines.peek().is_some() {
        let group = Group {
            0: build_backpack(lines.next().unwrap()),
            1: build_backpack(lines.next().unwrap()),
            2: build_backpack(lines.next().unwrap()),
        };
        match group.find_common_item() {
            None => continue,
            Some(duplicate) => match item_to_priority(duplicate) {
                None => continue,
                Some(priority) => answer += priority
            },
        };
    }
    println!("{}", answer);
}

pub fn day3() {
    let _example = include_str!("../files/day3_example.txt");
    let input = include_str!("../files/day3.txt");

    part_one(input);
    part_two(input);
}
