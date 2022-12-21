use std::fs;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref PATTERN: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
}

fn parse_stacking(text: &str) -> Vec<Vec<char>> {
    const MAX_LEN: usize = 9;
    let mut stacks = vec![vec![]; MAX_LEN];

    for line in text.lines() {

        for i in 0..MAX_LEN {
            let c = line.chars().nth(1 + i * 4).unwrap();
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    // Reverse the stacks so the top of the stack is sane
    for s in stacks.iter_mut() {
        s.reverse()
    }

    return stacks
}

fn run_instructions(text: &str, stacks: &mut Vec<Vec<char>>) {
    for line in text.lines() {
        // Parse instruction
        let cap = PATTERN.captures(line).unwrap();
        let n: usize = cap[1].parse().unwrap();
        let i: usize = cap[2].parse().unwrap();
        let j: usize = cap[3].parse().unwrap();

        // Apply operation
        let from_stack = &mut stacks[i-1];
        let mut substack = from_stack.split_off(from_stack.len() - n);
        let to_stack = &mut stacks[j-1];
        to_stack.append(&mut substack);
    }
}

fn print_answer(stacks: &Vec<Vec<char>>) {
    print!("answer = ");
    for s in stacks {
        print!("{}", s.last().unwrap());
    }
    print!("\n");
}

fn main() {
    let contents = &fs::read_to_string("input/input.txt").unwrap();
    let term = " 1   2   3   4   5   6   7   8   9 ";
    let i = contents.find(term).unwrap();
    let header = &contents[..i];
    let instructions_i = i + term.len() + 2;
    let instructions = &contents[instructions_i..];

    println!("Header:\n{}", header);

    // Parse header
    let mut stacks = parse_stacking(header);

    run_instructions(instructions, &mut stacks);

    println!("stacks after instructions: {:?}", stacks);

    print_answer(&stacks);
}
