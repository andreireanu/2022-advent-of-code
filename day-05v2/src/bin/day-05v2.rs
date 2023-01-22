use day_05v2::part_one;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Error reading file");
    let score1 = part_one(&file_contents);
}
