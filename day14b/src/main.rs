use std::fs;
use std::cmp;

struct Cave {
    grid: Vec<char>,
    x_min: usize,
    x_max: usize,
    y_max: usize
}

impl Cave {
    fn get(&self, x: usize, y: usize) -> char {
        let x_max = self.x_max;
        return self.grid[x + y * (x_max+1)];
    }

    fn set(&mut self, x: usize, y: usize, val: char) {
        let x_max = self.x_max;
        self.grid[x + y * (x_max+1)] = val;
    }
}

fn build_cave(walls: &Vec<Vec<(usize, usize)>>) -> Cave {
    // Get cave limits
    let (x, y): (Vec<usize>, Vec<usize>) = walls.iter().flatten().cloned().unzip();
    let x_min = *x.iter().min().unwrap();
    let x_max = *x.iter().max().unwrap();
    let y_max = *y.iter().max().unwrap();

    // Adjust y_max to accommodate floor
    let y_max = y_max + 2;

    // Adjust x_max to accommodate maximum possible width
    let x_max = cmp::max(x_max, y_max + 500);

    let grid = vec![' '; (x_max+1) * (y_max+1)];
    let mut cave = Cave {grid: grid, x_min: x_min, x_max: x_max, y_max: y_max};

    // Build cave floor
    for x in 0..=x_max {
        cave.set(x, y_max, '#');
    }

    // Build other walls
    for wall in walls.iter() {
        for coords in wall.windows(2) {
            let (start_x, start_y) = coords[0];
            let (end_x, end_y) = coords[1];
            let is_vertical = start_x == end_x;

            if is_vertical {
                let from_y = cmp::min(start_y, end_y);
                let to_y = cmp::max(start_y, end_y);
                for y in from_y..=to_y {
                    cave.set(start_x, y, '#');
                }
            } else {
                let from_x = cmp::min(start_x, end_x);
                let to_x = cmp::max(start_x, end_x);
                for x in from_x..=to_x {
                    cave.set(x, start_y, '#');
                }
            }
        }
    }

    return cave;
}

fn print_cave(cave: &Cave) {
    let x_max = cave.x_max;
    let y_max = cave.y_max;
    let x_min = cmp::min(cave.x_min, 500 - cave.y_max);
    
    for y in 0..=y_max {
        print!("{:3.} ", y);
        for x in x_min..=x_max {
            print!("{}", cave.get(x, y));
        }
        println!();
    }
}

fn parse(text: &str) -> Vec<Vec<(usize, usize)>> {
    let mut walls = vec![];
    for line in text.lines() {
        let mut v = vec![];
        for coord_txt in line.split(" -> ") {
            let (x, y) = coord_txt.split_once(',').unwrap();
            v.push((x.parse().unwrap(), y.parse().unwrap()));
        }
        walls.push(v);
    }
    return walls;
}

fn drop_sand_grain(cave: &mut Cave) -> bool {
    let mut sand_x = 500;
    let mut sand_y = 0;

    if cave.get(sand_x, sand_y) != ' ' {
        return true;
    }

    loop {
        // Propagate fall
        for y in sand_y.. {
            // Check if sand has met wall
            let square = cave.get(sand_x, y + 1);
            if square == '#' || square == '*' {
                sand_y = y;
                break;
            }
        }

        // Roll off left
        if cave.get(sand_x - 1, sand_y + 1) == ' ' {
            sand_x = sand_x - 1;
            sand_y = sand_y + 1;
            continue;
        }

        // Roll off right
        if cave.get(sand_x + 1, sand_y + 1) == ' ' {
            sand_x = sand_x + 1;
            sand_y = sand_y + 1;
            continue;
        }

        // Sand is at rest
        cave.set(sand_x, sand_y, '*');
        return false;
    }
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let walls = parse(&contents);
    let mut cave = build_cave(&walls);
    let mut sand_grains = 0;
    for i in 0.. {
        // print!("\n----Cave----\n");
        // print_cave(&cave);
        if drop_sand_grain(&mut cave) {
            sand_grains = i;
            break;
        }
    }
    println!("\nanswer = {}", sand_grains);
}
