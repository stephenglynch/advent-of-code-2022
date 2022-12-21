use std::{fs, collections::HashSet};

const UNIQUE_SEQ: usize = 14;

fn find_start(s: &str) -> usize {

    let v: Vec<char> = s.chars().collect();
    let windows = v.windows(UNIQUE_SEQ);
    for (i, w) in windows.enumerate() {
        let set: HashSet<char> = w.iter().cloned().collect();
        if set.len() == UNIQUE_SEQ {
            return i + UNIQUE_SEQ;
        }
    }
    return 0;
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    println!("answer = {}", find_start(&contents));
}
