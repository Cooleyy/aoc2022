use std::collections::HashMap;

#[derive(Debug)]
struct File<'a> {
    name: &'a str,
    size: u32
}

impl File<'_> {
    fn new(line: &str) -> Self {
        let mut line_split = line.split(" ");
        Self {
            name: line_split.next().unwrap(),
            size: line_split.next().unwrap().parse::<u32>().unwrap()
        }
    }
}

#[derive(Debug)]
struct Directory<'a> {
    name: String,
    files: Vec<File<'a>>,
    directories: HashMap<&'a str, Directory<'a>>,
    parent: Option<&'a Directory<'a>>
}

impl Directory<'_> {
    fn new(name: &str, parent: Option<&Directory>) -> Self {
        Self {
            name: String::from(name),
            files: Vec::new(),
            directories: HashMap::new(),
            parent,
        }
    }
}

fn move_directory<'a>(current_dir: &'a mut Directory<'a>, new_dir_name: &'a str) -> &'a mut Directory<'a> {
    if new_dir_name == ".." {
        &mut current_dir.parent.unwrap()
    }
    else {
        &mut current_dir.directories[new_dir_name]
    }
}

fn part_one(input: &str) {
    let mut top_level_dir = Directory::new("/", None);
    let mut current_dir = &top_level_dir;
    for io in input.lines() {
        match io[..4].as_ref() {
            "$ cd" => current_dir = move_directory(current_dir, &io[5..]),
            "$ ls" => (),
            "dir " => drop(current_dir.directories.insert(&io[4..], Directory::new(&io[4..], Some(&current_dir)))),
            _ => current_dir.files.push(File::new(&io)),
        }
    }
    dbg!(top_level_dir);
}

fn part_two(input: &str) {
}

pub fn day7() {
    let _example  = include_str!("../files/day7_example.txt");
    let input = include_str!("../files/day7.txt");

    part_one(_example);
    part_two(input);
}