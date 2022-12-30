use std::{fs, vec};
use std::collections::HashMap;

struct Region {
    num_rows: usize,
    num_cols: usize,
    start: (usize, usize),
    end: (usize, usize),
    memory: Vec<i8>
}

fn parse(text: &str) -> Region {
    let mut region = Region {
        num_rows: text.lines().count(),
        num_cols: text.lines().next().unwrap().chars().count(),
        start: (0, 0), 
        end: (0, 0),
        memory: vec![]};

    // Generate table mapping letter to height value
    let valid_vals = "abcdefghijklmnopqrstuvwxyz";
    let mut conversion_table: HashMap<char, i8> = valid_vals.chars().enumerate().map(|(h,l)| (l,h as i8)).collect();
    conversion_table.insert('S', 0);
    conversion_table.insert('E', 25);

    for (row_i, l) in text.lines().enumerate() {
        for (col_i, c) in l.chars().enumerate() {
            let height = *conversion_table.get(&c).unwrap();
            // Assign start and end
            match c {
                'S' => region.start = (row_i, col_i),
                'E' => region.end = (row_i, col_i),
                _ => ()
            }
            region.memory.push(height);
        }
    }

    return region;
}

fn get_height(region: &Region, row: usize, col: usize) -> i8 {
    region.memory[row * region.num_cols + col]
}

fn neighbours(region: &Region, node: (usize, usize)) -> Vec<(usize, usize)> {
    let (row, col) = node;
    let centre = get_height(&region, row, col);
    let row = row as i32;
    let col = col as i32;
    let num_cols = region.num_cols as i32;
    let num_rows = region.num_rows as i32;
    let candidates = [(row-1, col), (row+1, col), (row, col-1), (row, col+1)];

    candidates
        .iter()
        // Only consider candidates in grid bounds ...
        .filter(|x| match **x {
            (-1,_) => false,
            (_,-1) => false,
            (r,_) if r == num_rows => false,
            (_,c) if c == num_cols => false,
            _ => true
        })
        .map(|(r,c)| (*r as usize, *c as usize))
        // ... and matching allowed steps
        .filter(|(r,c)| {
            let h = get_height(&region, *r, *c);
            h >= 0 && h <= centre + 1 // allowed step range
        })
        .collect()
}

fn pop_next_shortest(region: &Region, unvisited: &mut Vec<(usize, usize)>, dist: &Vec<u32>) -> (usize, usize) {
    let num_cols = region.num_cols;
    let (node_i, node, _) = unvisited
        .iter()
        .enumerate()
        .map(|(i, (r,c))| (i, (*r, *c), dist[*r * num_cols + *c]))
        .min_by_key(|(_,_,d)| *d)
        .unwrap();
    unvisited.remove(node_i);
    return node;
}

fn distance(region: Region) -> Option<u32> {

    let num_rows = region.num_rows;
    let num_cols = region.num_cols;
    let mut dist = vec![u32::MAX; num_rows * num_cols];
    let mut unvisited = Vec::with_capacity(num_rows * num_cols);

    // Initialise unvisited
    for row in 0..num_rows {
        for col in 0..num_cols {
            unvisited.push((row, col));
        }
    }

    // Set starting node distance to 0
    let (start_row, start_col) = region.start;
    dist[start_row * num_cols + start_col] = 0;

    while !unvisited.is_empty() {
        let current = pop_next_shortest(&region, &mut unvisited, &dist);
        let (current_row, current_col) = current;
        let current_dist = dist[current_row * num_cols + current_col];

        // Check if we're done
        if current == region.end {
            return Some(current_dist);
        }
       
        // Remove current from unvisted
        for (nb_row, nb_col) in neighbours(&region, current) {
            let nb_dist = dist[nb_row * num_cols + nb_col];
            if current_dist.saturating_add(1) < nb_dist{
                dist[nb_row * num_cols + nb_col] = current_dist.saturating_add(1);
            }
        }
    }

    return None
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let region = parse(&contents);
    if let Some(min_dist) = distance(region) {
        println!("answer = {}", min_dist);
    } else {
        println!("not path found")
    }   
}
