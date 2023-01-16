pub fn normal_game_score((h1, h2): (&str, &str)) -> u32 {
    match (h1, h2) {
        ("A", "X") => {4},
        ("A", "Y") => {8},
        ("A", "Z") => {3},
        ("B", "X") => {1},
        ("B", "Y") => {5},
        ("B", "Z") => {9}, 
        ("C", "X") => {7},
        ("C", "Y") => {2},
        ("C", "Z") => {6}, 
        (_, _) => {0},
    }
}

pub fn cheat_game_score((h1, h2): (&str, &str)) -> u32 {
    match (h1, h2) {
        ("A", "X") => {3},
        ("A", "Y") => {4},
        ("A", "Z") => {8},
        ("B", "X") => {1},
        ("B", "Y") => {5},
        ("B", "Z") => {9}, 
        ("C", "X") => {2},
        ("C", "Y") => {6},
        ("C", "Z") => {7}, 
        (_, _) => {0},
    }
}
 

pub fn calculate_score_first(s: &String) -> u32 {
    s 
        .lines()
        .map(|line| {
            let splt = line.split(" ");
            let read_line: Vec<&str> = splt 
                .map(|el| el)
                .collect();
            let result = normal_game_score((read_line[0], read_line[1]));
            result
        })
        .sum() 
}

pub fn calculate_score_second(s: &String) -> u32 {
    s 
        .lines()
        .map(|line| {
            let splt = line.split(" ");
            let read_line: Vec<&str> = splt 
                .map(|el| el)
                .collect();
            let result = cheat_game_score((read_line[0], read_line[1]));
            result
        })
        .sum() 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_line_test() {
        let input = ("C", "X");
        let result = normal_game_score(input);
        assert_eq!(result, 7);
    }
}
