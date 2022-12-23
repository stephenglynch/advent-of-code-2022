use std::fs;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32
}
#[derive(Hash, Eq, PartialEq, Clone)]
struct RopeState {
    segments: Vec<Pos>
}

fn new_rope(length: usize) -> RopeState {
    let mut v = vec![];
    for _ in 0..length {
        v.push(Pos {x: 0, y: 0});
    }
    return RopeState{ segments: v };
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
        // First update head position
        state = update_head_position(state, direction);

        // Then update remaining segments
        for i in 0..state.segments.len()-1 {
            let leading = state.segments[i];
            let trailing = state.segments[i+1];
            let new_trailing = update_segment_pair(leading, trailing);
            state.segments[i+1] = new_trailing;

            let tail = state.segments.last().unwrap();
            unique_pos.insert(*tail);
        }
    }

    return state;
}

fn update_head_position(mut state: RopeState, dir: char) -> RopeState {
    let mut head = &mut state.segments[0];
    match dir {
        'U' => head.y += 1,
        'D' => head.y -= 1,
        'L' => head.x -= 1,
        'R' => head.x += 1,
        _ => panic!()
    };

    return state;
}

fn update_segment_pair(leading: Pos, trailing: Pos) -> Pos {
    if is_touching(leading, trailing) {
        return trailing;
    }

    let x_diff = trailing.x - leading.x;
    let y_diff = trailing.y - leading.y;

    let mut x_new = 0;
    let mut y_new = 0;
    if x_diff.abs() == 2 && y_diff.abs() == 2 {
        x_new = trailing.x - x_diff / 2;
        y_new = trailing.y - y_diff / 2;
    } else if x_diff == -2 {
        x_new = trailing.x + 1;
        y_new = leading.y;
    } else if x_diff == 2 {
        x_new = trailing.x - 1;
        y_new = leading.y;
    } else if y_diff == -2 {
        y_new = trailing.y + 1;
        x_new = leading.x;
    } else if y_diff == 2 {
        y_new = trailing.y - 1;
        x_new = leading.x;
    }

    return Pos {x: x_new, y: y_new};
}

fn is_touching(leading: Pos, trailing: Pos) -> bool {
    let x_diff = trailing.x - leading.x;
    let y_diff = trailing.y - leading.y;

    return !(x_diff.abs() >= 2 || y_diff.abs() >= 2)
}

fn print_rope(rope: &RopeState) {

    const X_MAX: usize = 100;
    const Y_MAX: usize = 100
    ; 

    let mut table = vec![];
    for _ in 0..Y_MAX {
        let mut v = vec![];
        for _ in 0..X_MAX {
            v.push(".".to_string());
        }
        table.push(v);
    }

    for (i, seg) in rope.segments.iter().enumerate() {
        let x: usize = ((X_MAX/2) as i32 - seg.x) as usize;
        let y: usize = ((Y_MAX/2) as i32 - seg.y) as usize;
        if i == 0 {
            table[y][x] = "H".to_string();
        } else {
            table[y][x] = i.to_string();
        }
    }

    for i in (0..Y_MAX) {
        for j in (0..X_MAX).rev() {
            print!("{}", table[i][j]);
        }
        println!();
    }
    println!();
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();

    let mut state = new_rope(10);
    let mut unique_positions = HashSet::new();
    unique_positions.insert(Pos {x: 0, y: 0});

    for line in contents.lines() {
        state = execute_instruction(state, &mut unique_positions, line);
    }

    println!("answer = {}", unique_positions.len());
}
