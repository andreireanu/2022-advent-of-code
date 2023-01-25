use std::collections::HashSet;

pub fn evaluate_slice(s: &str, len: usize) -> bool {
    let mut current: HashSet<char> = HashSet::new();
    for c in s.chars() {
        current.insert(c);
    }
    if current.len() < len {
        false
    } else {
        true
    }
}

pub fn evaluate_line(s: &str, len: usize) -> usize {
    let mut counter = 0;
    for i in 0..s.len() - len {
        if evaluate_slice(&s[i..i + len], len) {
            break;
        }
        counter = i + len + 1;
    }
    counter
}

pub fn part_one(s: &str) -> usize {
    evaluate_line(s, 4)
}

pub fn part_two(s: &str) -> usize {
    evaluate_line(s, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part_one() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = part_one(&input);
        assert_eq!(result, 5);
    }

    #[test]
    fn test2_part_one() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let result = part_one(&input);
        assert_eq!(result, 6);
    }

    #[test]
    fn test4_part_one() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let result = part_one(&input);
        assert_eq!(result, 10);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = part_two(&input);
        assert_eq!(result, 2);
    }
}
