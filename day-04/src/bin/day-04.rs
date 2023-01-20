use std::fs;
use day_04::calculate_score_first;
use day_04::calculate_score_second;

fn main() {
    let file_contents = fs::read_to_string("input.txt")
        .expect("Error reading file");
    let score1 = calculate_score_first(&file_contents);
    let score2 = calculate_score_second(&file_contents);
    println!("{}", score1);
    println!("{}", score2);
}