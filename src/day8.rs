#[derive(Debug)]
struct Tree {
    height: u32,
    visible: bool,
    score: u32
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Tree>>,
    num_rows: usize,
    num_cols: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let rows = input.lines().rev();
        let mut num_rows: usize = 0;
        let mut num_cols: usize = 0;
        let mut grid: Vec<Vec<Tree>> = Vec::new();
        for (i, row) in rows.enumerate() {
            if num_cols == 0 {num_cols = row.len()}
            grid.push(Vec::new());
            for (j, t) in row.chars().enumerate() {
                grid[i].push(Tree{
                    height: t.to_digit(10).unwrap(),
                    visible: i == 0 || j == 0 || j == num_cols-1,
                    score: 0
                });
            }
            num_rows += 1;
        }
        // set last row to visible
        for t in grid.last_mut().unwrap(){
            t.visible = true;
        }
        Self {
            grid,
            num_rows,
            num_cols,
        }
    }

    fn check_all(&mut self) {
        self.check_rows();
        self.check_cols();
    }

    fn check_rows(&mut self) {
        for row in self.grid[1..self.num_rows-1].as_mut(){
            Grid::check_line(&mut row.into_iter());
            Grid::check_line(&mut row.into_iter().rev());
        }
    }

    fn check_cols(&mut self) {
        for c in 1..&self.num_cols-1 {
            //down
            let mut col: Vec<u32> = Vec::new();
            for row in &self.grid{
                col.push(row[c].height.clone());
            }
            let mut tallest: u32 = col[0].clone();
            for (r, tree) in col.into_iter().enumerate() {
                if &tree > &tallest{
                    self.grid[r][c].visible = true;
                    tallest = tree.clone();
                }
            }

            //up
            let mut rev_col: Vec<u32> = Vec::new();
            for row in &self.grid{
                rev_col.push(row[c].height.clone());
            }
            rev_col = rev_col.into_iter().rev().collect();
            let mut tallest: u32 = rev_col[0].clone();
            for (r, tree) in rev_col.into_iter().enumerate() {
                if &tree > &tallest{
                    self.grid[&self.num_rows-r-1][c].visible = true;
                    tallest = tree.clone();
                }
            }
        }
    }
    
    pub fn check_line<'a, I>(mut row: &'a mut I)
    where &'a mut I: Iterator<Item = &'a mut Tree>,{
        let mut tallest: u32 = row.next().unwrap().height;
        for tree in row {
            if tree.height > tallest{
                tree.visible = true;
                tallest = tree.height;
            }
        }
    }

    pub fn line_view_distance<'a, I>(line: I, target: &u32) -> u32
    where I: Iterator<Item = u32>,{
        let mut view_distance: u32 = 1;
        for tree in line {
            if &tree >= target{
                return view_distance;
            }
            view_distance += 1;
        }
        view_distance-1
    }

    fn score_all(&mut self) {
        for row in 0..self.num_rows {
            for col in 0..self.num_cols{
                self.grid[row][col].score = self.score(row, col);
            }
        }
    }

    fn score(&self, row: usize, col: usize)-> u32{
        let target: &u32 = &self.grid[row][col].height;
        let mut left_view: Vec<u32> = Vec::new();
        let mut right_view: Vec<u32> = Vec::new();
        let mut i: usize = 0;
        for t in &self.grid[row]{
            if i < col {
                left_view.push(t.height)
            }
            else if i > col {
                right_view.push(t.height)
            }
            i += 1
        }

        i = 0;
        let mut up_view: Vec<u32> = Vec::new();
        let mut down_view: Vec<u32> = Vec::new();
        for r in &self.grid {
            if i < row {
                down_view.push(r[col].height)
            }
            else if i > row {
                up_view.push(r[col].height)
            }
            i += 1
        }

        Grid::line_view_distance(left_view.into_iter().rev(), target) *
        Grid::line_view_distance(right_view.into_iter(), target) *
        Grid::line_view_distance(up_view.into_iter(), target) *
        Grid::line_view_distance(down_view.into_iter().rev(), target)
    }

    fn highest_score(&self) -> u32 {
        let mut highest: u32 = 0;
        for row in &self.grid {
            for t in row{
                if t.score > highest {highest = t.score}
            }
        }
        highest
    }

    fn total_visible(&self) -> u32 {
        let mut total: u32 = 0;
        for row in &self.grid {
            for t in row{
                if t.visible {total += 1}
            }
        }
        total
    }
}

fn part_one(input: &str){
    let mut grid = Grid::new(input);

    grid.check_all();
    println!("{}", grid.total_visible());
}

fn part_two(input: &str) {
    let mut grid = Grid::new(input);
    grid.score_all();
    println!("{}", grid.highest_score());
}

pub fn day8() {
    let _example  = include_str!("../files/day8_example.txt");
    let input = include_str!("../files/day8.txt");

    part_one(&input);
    part_two(&input);
}