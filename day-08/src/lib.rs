pub struct Direction {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

pub fn evaluate_tree(matrix: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
    let current = matrix[i][j];
    let mut direction_eval = Direction {
        up: true,
        down: true,
        left: true,
        right: true,
    };

    // Eval left
    for x in (0..j).rev() {
        if matrix[i][x] >= current {
            direction_eval.left = false;
            break;
        }
    }
    if direction_eval.left == true {
        return true;
    }

    // Eval right
    if j + 1 < matrix[0].len() {
        for x in j + 1..matrix[0].len() {
            if matrix[i][x] >= current {
                direction_eval.right = false;
                break;
            }
        }
    }

    if direction_eval.right == true {
        return true;
    }

    // Eval up
    for x in (0..i).rev() {
        if matrix[x][j] >= current {
            direction_eval.up = false;
            break;
        }
    }
    if direction_eval.up == true {
        return true;
    }

    // Eval down
    if i + 1 < matrix[0].len() {
        for x in i + 1..matrix.len() {
            if matrix[x][j] >= current {
                direction_eval.down = false;
                break;
            }
        }
    }
    if direction_eval.down == true {
        return true;
    }

    false
}

pub fn part_one(s: &str) -> usize {
    let matrix: Vec<Vec<u8>> = s
        .lines()
        .map(|line| {
            let mut line_vec: Vec<u8> = vec![];
            for ch in line.chars() {
                line_vec.push(ch.to_digit(10).unwrap() as u8);
            }
            line_vec
        })
        .collect();
    let mut sum = 0;
    for i in 0..matrix[0].len() {
        for j in 0..matrix.len() {
            if evaluate_tree(&matrix, i, j) {
                sum += 1;
            }
        }
    }
    sum
}

pub fn calculate_scenic_score(matrix: &Vec<Vec<u8>>, i: usize, j: usize) -> u32 {
    let current = matrix[i][j];
    let (mut left, mut right, mut up, mut down) = (0, 0, 0, 0);
    // Eval left
    for x in (0..j).rev() {
        left += 1;
        if matrix[i][x] >= current {
            break;
        }
    }

    // Eval right
    if j + 1 < matrix[0].len() {
        for x in j + 1..matrix[0].len() {
            right += 1;
            if matrix[i][x] >= current {
                break;
            }
        }
    }

    // Eval up
    for x in (0..i).rev() {
        up += 1;
        if matrix[x][j] >= current {
            break;
        }
    }

    // Eval down
    if i + 1 < matrix[0].len() {
        for x in i + 1..matrix.len() {
            down += 1;
            if matrix[x][j] >= current {
                break;
            }
        }
    }

    left * right * up * down
}

pub fn part_two(s: &str) -> u32 {
    let matrix: Vec<Vec<u8>> = s
        .lines()
        .map(|line| {
            let mut line_vec: Vec<u8> = vec![];
            for ch in line.chars() {
                line_vec.push(ch.to_digit(10).unwrap() as u8);
            }
            line_vec
        })
        .collect();
    let mut max = 0;
    for i in 0..matrix[0].len() {
        for j in 0..matrix.len() {
            let score = calculate_scenic_score(&matrix, i, j);
            if score > max {
                max = score;
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "30373
25512
65332
33549
35390";
        let result = part_one(&input);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_two() {
        let input = "30373
25512
65332
33549
35390";
        let result = part_two(&input);
        assert_eq!(result, 8);
    }
}
