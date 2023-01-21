use std::fs;
use day_05::part_one;
use day_05::part_two;

fn main() {
    let file_contents = fs::read_to_string("input.txt")
        .expect("Error reading file");
    let score1 = part_one(&file_contents);
    let score2 = part_two(&file_contents);
    println!("{}", score1);
    println!("{}", score2);
}