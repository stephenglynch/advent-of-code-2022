use std::fs;

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();

    let total_cals_list: Vec<i32> = contents.split("\n\n").map(|elf_food| {
        elf_food.split('\n').map(|s| s.parse::<i32>().unwrap()).sum()
    }).collect();

    let (i, max_cals) = total_cals_list
        .iter()
        .enumerate()
        .max_by(|(_,x), (_,y)| x.cmp(y))
        .unwrap();

    println!("Most calories carried by elf is {}", max_cals);
}
