use std::fs;
use day_03::calculate_score_first;
use day_03::calculate_score_second;

fn main() {
    let file1_contents = fs::read_to_string("inputPart1.txt")
        .expect("Error reading file");
    let score1 = calculate_score_first(&file1_contents);
    let file2_contents = fs::read_to_string("inputPart2.txt")
    .expect("Error reading file");
    let score2 = calculate_score_second(&file2_contents);
    println!("{}", score1);
    println!("{}", score2);
}