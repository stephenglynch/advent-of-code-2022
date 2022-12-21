use std::{fs, collections::HashSet};

fn find_start(s: &str) -> usize {

    let v: Vec<char> = s.chars().collect();
    let windows = v.windows(4);
    for (i, w) in windows.enumerate() {
        let set: HashSet<char> = w.iter().cloned().collect();
        if set.len() == 4 {
            return i + 4;
        }
    }
    return 0;
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    println!("answer = {}", find_start(&contents));
}
