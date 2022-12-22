use std::fs;

fn parse_forest(text: &str) -> Vec<Vec<u8>> {
    let mut forest = vec![];
    for line in text.lines() {
        let mut v = vec![];
        for c in line.chars() {
            v.push(c as u8);
        }
        forest.push(v);
    }

    return forest;
}

fn is_tree_visible(forest: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    let tree_height = forest[row][col];

    let mut visible_east = true;
    for t in &forest[row][col+1..] {
        if *t >= tree_height {
            visible_east = false;
            break;
        }
    }

    let mut visible_west = true;
    for t in &forest[row][..col] {
        if *t >= tree_height {
            visible_west = false;
            break;
        }
    }

    let mut visible_south = true;
    for r in &forest[row+1..] {
        let t = r[col];
        if t >= tree_height {
            visible_south = false;
            break;
        }
    }

    let mut visible_north = true;
    for r in &forest[..row] {
        let t = r[col];
        if t >= tree_height {
            visible_north = false;
            break;
        }
    }
    
    return visible_east || visible_west || visible_south || visible_north;
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let forest = parse_forest(&contents);

    let max_row = forest.len();
    let max_col = forest[0].len();

    let mut count = 0;
    for i in 0..max_row {
        for j in 0..max_col {
            count += is_tree_visible(&forest, i, j) as u32;
        }
    }

    println!("answer = {}", count);
}
