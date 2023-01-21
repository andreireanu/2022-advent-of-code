pub fn part_one(s: &str) -> String {
    let input = s
        .split("\n\n")
        .collect::<Vec<&str>>();
    let mut setup = input[0]
        .lines()
        .rev()
        .collect::<Vec<&str>>()
        .into_iter();
    let indexes = setup.next().unwrap();
    let idx_vec = indexes
                    .split(" ")
                    .collect::<Vec<&str>>();
    let idx = idx_vec[idx_vec.len()-2].parse::<usize>().unwrap();
    let mut crates: Vec<Vec<char>> = vec![vec![]; idx];
    while let Some(line) = setup.next() {
        for id in 0..idx {
            let current_char = line.chars().nth(id*4+1).unwrap();
            if current_char != ' ' {
                crates[id].push(current_char);
            }
        }
    }
    let moves = input[1].lines()
        .map(|line| {
            line.split(" ")
            .filter(|element| {
               if let Ok(_) = element.parse::<usize>() {
                    true
               } else {
                    false
               }
            })
            .collect::<Vec<&str>>()
            .iter()
            .map(|str_nr| str_nr.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()            
        })
        .collect::<Vec<Vec<usize>>>() ;
    for single_move in moves.iter() {
        for _ in 0..single_move[0] {
            let to_move = crates[single_move[1]-1].pop().unwrap();
            crates[single_move[2]-1].push(to_move);
        }
    }
    let mut out_string: String = "".to_owned();
    for id in 0..idx {
        let top_crate = crates[id].pop().unwrap();
        out_string += &top_crate.to_string();
    }
    out_string
    }

pub fn part_two(s: &str) -> String {
    let input = s
        .split("\n\n")
        .collect::<Vec<&str>>();
    let mut setup = input[0]
        .lines()
        .rev()
        .collect::<Vec<&str>>()
        .into_iter();
    let indexes = setup.next().unwrap();
    let idx_vec = indexes
                    .split(" ")
                    .collect::<Vec<&str>>();
    let idx = idx_vec[idx_vec.len()-2].parse::<usize>().unwrap();
    let mut crates: Vec<Vec<char>> = vec![vec![]; idx];
    while let Some(line) = setup.next() {
        for id in 0..idx {
            let current_char = line.chars().nth(id*4+1).unwrap();
            if current_char != ' ' {
                crates[id].push(current_char);
            }
        }
    }
    let moves = input[1].lines()
        .map(|line| {
            line.split(" ")
            .filter(|element| {
                if let Ok(_) = element.parse::<usize>() {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<&str>>()
            .iter()
            .map(|str_nr| str_nr.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()            
        })
        .collect::<Vec<Vec<usize>>>() ;
    for single_move in moves.iter() {
        let mut temp_vec: Vec<char> = Vec::new();
        for _ in 0..single_move[0] {
            let to_move = crates[single_move[1]-1].pop().unwrap();
            temp_vec.push(to_move);
        }
        while let Some(poped) = temp_vec.pop() {
            crates[single_move[2]-1].push(poped);
        }

    }
    let mut out_string: String = "".to_owned();
    for id in 0..idx {
        let top_crate = crates[id].pop().unwrap();
        out_string += &top_crate.to_string();
    }
    out_string
    }
    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = part_one(&input);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part_two() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = part_two(&input);
        assert_eq!(result, "MCD");
    } 
}
