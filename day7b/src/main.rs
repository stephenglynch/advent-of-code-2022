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

            // Check if we have a cd (do nothing on ls)
            if let Some(dir_name) = cap.name("cd") {
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
    let mut nodes = vec![];
    if let Node::Dir(dir) = tree {
        nodes.push(Node::Dir(dir.clone()));

        for dir_name in dir.borrow().keys() {
            if dir_name != ".." {
                nodes.append(&mut list_directories(dir.borrow().get(dir_name).unwrap()));
            }
        }
    }
    return nodes;
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
    let root = parse_dir_tree(&lines[1..]);

    let dirs = list_directories(&root);
    let mut sizes: Vec<u32> = dirs.iter().map(|n| dir_size(n)).collect();
    sizes.sort();
    println!("sizes = {:?}", sizes);

    const CAPACITY: u32 = 70000000;
    const UPDATE_REQUIRES: u32 = 30000000;
    let space_left = CAPACITY - dir_size(&root);
    println!("space left = {}", space_left);

    let space_to_free = UPDATE_REQUIRES - space_left;
    println!("space to free = {}", space_to_free);

    let answer = sizes.into_iter().filter(|s| *s >= space_to_free).next().unwrap();
    println!("answer = {:?}", answer);
}
