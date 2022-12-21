use std::fs;
use std::collections::HashSet;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref PATTERN: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
}

fn parse_line(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    let caps = PATTERN.captures(line).unwrap();
    let s1: u32 = caps[1].parse().unwrap();
    let e1: u32 = caps[2].parse().unwrap();
    let s2: u32 = caps[3].parse().unwrap();
    let e2: u32 = caps[4].parse().unwrap();

    ((s1..=e1).collect(), (s2..=e2).collect())
}

fn main() {
    let contents = &fs::read_to_string("input/input.txt").unwrap();    

    let mut total = 0;
    for l in contents.lines() {
        let (a, b) = parse_line(l);

        if !a.is_disjoint(&b) {
            total += 1;
        }
    }

    println!("answer = {}", total);
}
