use std::ops::Range;

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

pub fn part_two(s: &str) -> Vec<Vec<char>> {
    let mut cycle: usize = 0;
    let mut v: isize = 1;
    let mut crt: Vec<Vec<char>> = vec![vec!['*'; 40]; 6];
    let cycles: Vec<usize> = vec![40, 80, 120, 160, 200, 240];
    let mut cycles_index = 0;
    s.lines().for_each(|line| {
        let command: Vec<&str> = line.split(' ').collect();
        match command[0] {
            "noop" => {
                let range: Range<usize> = cycle..cycle + 1;
                for idx in range {
                    if cycles.contains(&idx) {
                        cycles_index += 1;
                    }
                    let sprite_center = v;
                    let idx_compact = (idx % 40) as isize;
                    if sprite_center == idx_compact
                        || sprite_center - 1 == idx_compact
                        || sprite_center + 1 == idx_compact
                    {
                        crt[cycles_index][idx % 40] = '#';
                    } else {
                        crt[cycles_index][idx % 40] = '.';
                    }
                }
                cycle += 1;
            }
            "addx" => {
                let range = cycle..cycle + 2;
                for idx in range {
                    if cycles.contains(&idx) {
                        cycles_index += 1;
                    }
                    let sprite_center = v;
                    let idx_compact = (idx % 40) as isize;
                    if sprite_center == idx_compact
                        || sprite_center - 1 == idx_compact
                        || sprite_center + 1 == idx_compact
                    {
                        crt[cycles_index][idx % 40] = '#';
                    } else {
                        crt[cycles_index][idx % 40] = '.';
                    }
                }
                cycle += 2;
                v += command[1].parse::<isize>().unwrap();
            }
            _ => {}
        };
    });
    crt
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
        let result = part_two(&input);
        assert_eq!(result, [['#', '#', '.', '.', '#', '#', '.', '.', '#', '#', '.', '.', '#', '#', '.', '.', '#', '#', '.', '.', '#', '#', '.', '.', '#', '#', '.', '.', '#', '#', '.', '.', '#', '#', '.', '.', '#', '#', '.', '.'], ['#', '#', '#', '.', '.', '.', '#', '#', '#', '.', '.', '.', '#', '#', '#', '.', '.', '.', '#', '#', '#', '.', '.', '.', '#', '#', '#', '.', '.', '.', '#', '#', '#', '.', '.', '.', '#', '#', '#', '.'], ['#', '#', '#', '#', '.', '.', '.', '.', '#', '#', '#', '#', '.', '.', '.', '.', '#', '#', '#', '#', '.', '.', '.', '.', '#', '#', '#', '#', '.', '.', '.', '.', '#', '#', '#', '#', '.', '.', '.', '.'], ['#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.'], ['#', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '.', '#', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '.', '#', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '.', '#', '#', '#', '#'], ['#', '#', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '.', '.', '#', '#', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.', '.', '.', '#', '#', '#', '#', '#', '#', '#', '.', '.', '.', '.', '.']]);
    }
}
