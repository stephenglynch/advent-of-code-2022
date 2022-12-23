use std::fs;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32
}
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct RopeState {
    head: Pos,
    tail: Pos
}

fn parse_line(line: &str) -> (char, u32) {
    let mut parts = line.split_whitespace();
    let direction = parts.next().unwrap().chars().next().unwrap();
    let steps = parts.next().unwrap().parse().unwrap();
    return (direction, steps)
}

fn execute_instruction(mut state: RopeState, unique_pos: &mut HashSet<Pos>, line: &str) -> RopeState {
    let (direction, steps) = parse_line(line);

    for _ in 0..steps {
        state = update_head_position(state, direction);
        state = update_tail_position(state);
        unique_pos.insert(state.tail);
    }

    return state;
}

fn update_head_position(state: RopeState, dir: char) -> RopeState {
    let mut head = state.head;
    match dir {
        'U' => head.y += 1,
        'D' => head.y -= 1,
        'L' => head.x -= 1,
        'R' => head.x += 1,
        _ => panic!()
    };

    return RopeState { head: head, tail: state.tail };
}

fn update_tail_position(state: RopeState) -> RopeState {
    if is_touching(state) {
        return state;
    }

    let head = state.head;
    let tail = state.tail;

    let x_diff = tail.x - head.x;
    let y_diff = tail.y - head.y;

    let mut x_new = 0;
    let mut y_new = 0;
    if x_diff == -2 {
        x_new = tail.x + 1;
        y_new = head.y;
    } else if x_diff == 2 {
        x_new = tail.x - 1;
        y_new = head.y;
    } else if y_diff == -2 {
        y_new = tail.y + 1;
        x_new = head.x;
    } else if y_diff == 2 {
        y_new = tail.y - 1;
        x_new = head.x;
    }

    return RopeState { head: head, tail: Pos { x: x_new, y: y_new} };
}

fn is_touching(state: RopeState) -> bool {
    let head = state.head;
    let tail = state.tail;

    let x_diff = tail.x - head.x;
    let y_diff = tail.y - head.y;

    return !(x_diff.abs() >= 2 || y_diff.abs() >= 2)
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();

    let mut state = RopeState { head: Pos {x: 0, y: 0}, tail: Pos {x: 0, y: 0}};
    let mut unique_positions = HashSet::new();

    unique_positions.insert(Pos {x: 0, y: 0});
    for line in contents.lines() {
        state = execute_instruction(state, &mut unique_positions, line);
    }

    println!("answer = {}", unique_positions.len());
}
