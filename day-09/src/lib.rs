use std::vec;

pub struct Position {
    x: usize,
    y: usize,
}

pub trait Adjacent {
    fn is_adjacent(&self, other: &Self) -> bool;
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Adjacent for Position {
    fn is_adjacent(&self, other: &Self) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }
}

pub fn print_scene(h: &Position, t: &Position, tailed: &Vec<Vec<char>>) {
    println!("Status: ");
    for i in 0..tailed.len() {
        for j in 0..tailed[0].len() {
            let current_pos = Position { x: i, y: j };
            if current_pos == *h && current_pos == *t {
                print!("H");
            } else if current_pos == *h {
                print!("H");
            } else if current_pos == *t {
                print!("T");
            } else {
                print!(".");
            };
        }
        println!();
    }
    println!("Visited: ");
    for i in 0..tailed.len() {
        for j in 0..tailed[0].len() {
            print!("{}", tailed[i][j]);
        }
        println!();
    }
    println!();
}

pub fn update_status(h: &mut Position, t: &mut Position, tailed: &mut Vec<Vec<char>>) {
    if !&h.is_adjacent(&t) {
        if h.x < t.x && h.y > t.y {
            t.x -= 1;
            t.y += 1;
        } else if h.x < t.x && h.y == t.y {
            t.x -= 1;
        } else if h.x < t.x && h.y < t.y {
            t.x -= 1;
            t.y -= 1;
        } else if h.x > t.x && h.y > t.y {
            t.x += 1;
            t.y += 1;
        } else if h.x > t.x && h.y == t.y {
            t.x += 1;
        } else if h.x > t.x && h.y < t.y {
            t.x += 1;
            t.y -= 1;
        } else if h.x == t.x && h.y < t.y {
            t.y -= 1;
        } else if h.x == t.x && h.y > t.y {
            t.y += 1;
        };
        tailed[t.x][t.y] = '#';
    };
}

pub fn part_one(s: &str) -> usize {
    let start_offset = 375;
    let mut tailed = vec![vec!['.'; start_offset * 2 + 1]; start_offset * 2 + 1];
    tailed[start_offset][start_offset] = '#';
    let mut h = Position {
        x: start_offset,
        y: start_offset,
    };
    let mut t = Position {
        x: start_offset,
        y: start_offset,
    };
    // println!("BEGINNIG: ");
    // print_scene(&h, &t, &tailed);
    s.lines().for_each(|line| {
        let splt = line.split(" ");
        let read_line: Vec<&str> = splt.map(|el| el).collect();
        let command = read_line[0];
        let positions = read_line[1].parse::<usize>().unwrap();
        match command {
            "R" => {
                for _ in 0..positions {
                    h.y += 1;
                    update_status(&mut h, &mut t, &mut tailed);
                    // println!("RIGHT --> ");
                    // print_scene(&h, &t, &tailed);
                }
            }
            "L" => {
                for _ in 0..positions {
                    h.y -= 1;
                    update_status(&mut h, &mut t, &mut tailed);
                    // println!("LEFT --> ");
                    // print_scene(&h, &t, &tailed);
                }
            }
            "U" => {
                for _ in 0..positions {
                    h.x -= 1;
                    update_status(&mut h, &mut t, &mut tailed);
                    // println!("UP --> ");
                    // print_scene(&h, &t, &tailed);
                }
            }
            "D" => {
                for _ in 0..positions {
                    h.x += 1;
                    update_status(&mut h, &mut t, &mut tailed);
                    // println!("DOWN --> ");
                    // print_scene(&h, &t, &tailed);
                }
            }
            _ => {}
        }
    });
    tailed
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|element| if element == '#' { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

pub fn part_two(s: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let result = part_one(&input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_two() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let result = part_two(&input);
        assert_eq!(result, 8);
    }
}
