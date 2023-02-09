pub fn part_one(s: &str) -> isize {
    let mut cycle: isize = 0;
    let mut v: isize = 1;
    let cycles: Vec<isize> = vec![20, 60, 100, 140, 180, 220];
    let mut cycles_index = 0;
    s.lines()
        .map(|line| {
            let command: Vec<&str> = line.split(' ').collect();
            let strength = match command[0] {
                "noop" => {
                    let range = cycle..cycle + 1;
                    let mut return_value: isize = 0;
                    if range.contains(&cycles[cycles_index]) {
                        return_value = v * cycles[cycles_index];
                        if let Some(_) = cycles.get(cycles_index + 1) {
                            cycles_index += 1;
                        } else {
                            cycles_index = 0;
                        }
                    };
                    cycle += 1;
                    return_value
                }
                "addx" => {
                    let range = cycle..=cycle + 2;
                    let mut return_value: isize = 0;
                    if range.contains(&cycles[cycles_index]) {
                        return_value = v * cycles[cycles_index];
                        if let Some(_) = cycles.get(cycles_index + 1) {
                            cycles_index += 1;
                        } else {
                            cycles_index = 0;
                        }
                    };
                    cycle += 2;
                    v += command[1].parse::<isize>().unwrap();
                    return_value
                }
                _ => 0,
            };
            strength
        })
        .sum()
}

pub fn part_two(s: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let result = part_one(&input);
        assert_eq!(result, 13140);
    }

    #[test]
    fn test_part_two() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let result = part_two(&input);
        assert_eq!(result, 36);
    }
}
