use core::panic;
use std::fs;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref CMD_PATTERN: Regex = Regex::new(r"^\$ cd (?P<cd>.+)|\$ ls$").unwrap();
    static ref NODE_PATTERN: Regex = Regex::new(r"^dir (?P<dir>.+)|(?P<size>\d+) (?P<file>.+)$").unwrap();
}

#[derive(Debug, Clone)]
enum Node {
    Dir (Rc<RefCell<HashMap<String, Node>>>),
    File (u32)
}

fn new_dir(parent: Rc<RefCell<HashMap<String, Node>>>) -> Node {
    let mut directories = HashMap::new();
    directories.insert("..".to_string(), Node::Dir(parent));
    let directories = Rc::new(RefCell::new(directories));
    return Node::Dir(directories);
}

fn new_file(size: u32) -> Node {
    Node::File(size)
}

fn parse_dir_tree(lines: &[&str]) -> Node {

    let root_dir: Rc<RefCell<HashMap<String, Node>>> = Rc::new(RefCell::new(HashMap::new()));
    let mut current = root_dir.clone();

    for line in lines {
        // Check if we have a command
        if line.starts_with("$") {
            let cap = CMD_PATTERN.captures(line).unwrap();

            // Check if we have a cd or an ls
            if let Some(dir_name) = cap.name("cd") {
                // cd 
                let dir_name = dir_name.as_str();

                // Move into the next directory and run next command
                current = match current.clone().borrow().get(dir_name).unwrap() {
                    Node::Dir(next_dir) => next_dir.clone(),
                    _ => panic!()
                };
            }
        } else {
            let cap = NODE_PATTERN.captures(line).unwrap();
            
            if let Some(dir_name) = cap.name("dir") {
                // Build directory
                let dir_name = dir_name.as_str().to_string();
                let new_node = new_dir(current.clone());
                current.borrow_mut().insert(dir_name, new_node);
            } else {
                // Build file
                let file_name = cap.name("file").unwrap().as_str().to_string();
                let file_size = cap.name("size").unwrap().as_str().parse().unwrap();
                let new_node = new_file(file_size);
                current.borrow_mut().insert(file_name, new_node);
            }
        }
    }

    return Node::Dir(root_dir);
}

fn list_directories(tree: &Node) -> Vec<Node> {
    let mut acc = vec![];
    list_directories2(&mut acc, tree);
    return acc;
}

fn list_directories2(acc: &mut Vec<Node>, tree: &Node) {
    if let Node::Dir(dir) = tree {
        acc.push(Node::Dir(dir.clone()));

        for dir_name in dir.borrow().keys() {
            if dir_name != ".." {
                list_directories2(acc, dir.borrow().get(dir_name).unwrap())
            }
        }
    }
}

fn dir_size(tree: &Node) -> u32 {
    match tree {
        Node::File(size) => *size,
        Node::Dir(dir) => {
            let mut total = 0;
            for dir_name in dir.borrow().keys() {
                if dir_name != ".." {
                    total += dir_size(dir.borrow().get(dir_name).unwrap());
                }
            }
            total
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let tree = parse_dir_tree(&lines[1..]);

    let dirs = list_directories(&tree);
    println!("num of directories = {}", dirs.len());

    let sizes: Vec<u32> = dirs.iter().map(|n| dir_size(n)).collect();
    println!("sizes = {:?}", sizes);
    
    // Calculate answer
    let mut answer = 0;
    for s in sizes {
        if s <= 100000 {
            answer += s;
        } 
    }

    println!("answer = {}", answer);
}
