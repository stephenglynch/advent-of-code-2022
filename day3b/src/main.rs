use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;

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
    for (a, b, c) in contents.lines().tuples() {
        let a: HashSet<_> = a.chars().collect();
        let b: HashSet<_> = b.chars().collect();
        let c: HashSet<_> = c.chars().collect();

        let shared: HashSet<_> = a.intersection(&b).cloned().collect();
        let shared: HashSet<_> = shared.intersection(&c).collect();

        for l in shared {
            total += priority_map[l];
        }
    }
    
    println!("total = {}", total);
}
