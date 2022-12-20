use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

static LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let contents = &fs::read_to_string("input/input.txt").unwrap();
    let pairs: Vec<_> = LETTERS.chars().enumerate().collect();

    // Build priority map
    let mut priority_map = HashMap::new();
    for (i, l) in pairs {
        priority_map.insert(l, i+1);
    }

    let mut total = 0;
    for line in contents.lines() {
        let line_len = line.len() / 2;
        let left: HashSet<_> = line[..line_len].chars().collect();
        let right: HashSet<_> = line[line_len..].chars().collect();
        let shared = left.intersection(&right);

        for l in shared {
            total += priority_map[l];
        }
    }
    
    println!("total = {}", total);
}
