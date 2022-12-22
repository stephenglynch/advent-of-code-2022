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

fn tree_scenic_score(forest: &Vec<Vec<u8>>, row: usize, col: usize) -> u32 {
    let tree_height = forest[row][col];
    let max_row = forest.len();
    let max_col = forest[0].len();

    // Edge trees score 0
    if row == 0 || row == max_row - 1 || col == 0 || col == max_col - 1 {
        return 0;
    }

    let mut score_east = 0;
    for t in &forest[row][col+1..] {
        score_east += 1;
        if *t >= tree_height {
            break;
        }
    }

    let mut score_west = 0;
    for t in forest[row][..col].iter().rev() {
        score_west += 1;
        if *t >= tree_height {
            break;
        }
    }

    let mut score_south = 0;
    for r in &forest[row+1..] {
        let t = r[col];
        score_south += 1;
        if t >= tree_height {
            break;
        }
    }

    let mut score_north = 0;
    for r in forest[..row].iter().rev() {
        let t = r[col];
        score_north += 1;
        if t >= tree_height {
            break;
        }
    }
    
    return score_east * score_west * score_south * score_north;
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let forest = parse_forest(&contents);

    let max_row = forest.len();
    let max_col = forest[0].len();

    let mut max_score = 0;
    for i in 0..max_row {
        for j in 0..max_col {
            let score = tree_scenic_score(&forest, i, j) as u32;
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("answer = {}", max_score);
}
