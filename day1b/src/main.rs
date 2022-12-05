use std::fs;

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();

    let mut total_cals_list: Vec<i32> = contents.split("\n\n").map(|elf_food| {
        elf_food.split('\n').map(|s| s.parse::<i32>().unwrap()).sum()
    }).collect();

    total_cals_list.sort();

    let top_3_total: i32 = total_cals_list.iter().rev().take(3).sum();

    println!("Top three elves total calories = {}", top_3_total);
}
