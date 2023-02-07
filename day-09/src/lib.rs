use std::{vec, cell::Cell, char::from_digit};

#[derive(Copy, Clone, Debug)]
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


// Part 1
pub fn print_scene(h: &Position, t: &Position, size: usize) {
    println!("Status: ");
    for i in 0..size * 2 + 1 {
        for j in 0..size*2+1 {
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
    println!();
}

pub fn print_visited(tailed: &Vec<Vec<char>>) {
    println!("Visited: ");
    for i in 0..tailed.len() {
        for j in 0..tailed[0].len() {
            print!("{}", tailed[i][j]);
        }
        println!();
    }
    println!();
}

pub fn update_status(h: &Position, t: &mut Position) {
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

    };
}

pub fn update_status_tail(tailed: &mut Vec<Vec<char>>, t: &Position) {
    tailed[t.x][t.y] = '#';
}

pub fn update_scene_part1(
    h: &Position,
    t: &mut Position,
    tailed: &mut Vec<Vec<char>>,
    offset: usize,
) {
    update_status(&h, t);
    update_status_tail(tailed, &t);
    // print_scene(&h, &t, offset);
    // print_visited(&tailed);
}

// Part 2

pub fn update_status_complex_tail(h: &Position, tail: &mut Vec<Cell<Position>>) {
    update_status(h, tail[0].get_mut());
    for i in 0..tail.len() - 1 {
        update_status(&tail[i].get(), tail[i + 1].get_mut());
    }
}

pub fn print_visited_complex_tail(h: &Position, tails: &Vec<Cell<Position>>, offset: usize) {
    let mut temp = vec![vec!['.'; offset * 2 + 1]; offset * 2 + 1];
    for (i, tail) in tails.iter().enumerate().rev() {
        let t = tail.get();
        temp[t.x][t.y] = from_digit((i + 1) as u32, 10).unwrap();
    }
    temp[h.x][h.y] = 'H';
    println!("Visited: ");
    for i in 0..temp.len() {
        for j in 0..temp[0].len() {
            print!("{}", temp[i][j]);
        }
        println!();
    }
    println!();
}

pub fn update_scene_part2(
    h: &Position,
    tails: &mut Vec<Cell<Position>>,
    tailed: &mut Vec<Vec<char>>,
    _offset: usize,
) {
    update_status_complex_tail(&h, tails);
    // print_visited_complex_tail(&h, &tails, offset);
    let tail9 = tails[8].get();
    tailed[tail9.x][tail9.y] = '#';
    // print_visited(&tailed);
}

// Main

pub fn part_one(s: &str) -> usize {
    let offset = 375;
    let mut tailed = vec![vec!['.'; offset * 2 + 1]; offset * 2 + 1];
    tailed[offset][offset] = '#';
    let mut h = Position {
        x: offset,
        y: offset,
    };
    let mut t = Position {
        x: offset,
        y: offset,
    };
    // println!("BEGINNING: ");
    // print_scene(&h, &t, offset);
    // print_visited(&tailed);
    s.lines().for_each(|line| {
        let splt = line.split(" ");
        let read_line: Vec<&str> = splt.map(|el| el).collect();
        let command = read_line[0];
        let positions = read_line[1].parse::<usize>().unwrap();
        match command {
            "R" => {
                for _ in 0..positions {
                    h.y += 1;
                    // println!("RIGHT --> ");
                    update_scene_part1(& h, &mut t,&mut tailed, offset);
                }
            }
            "L" => {
                for _ in 0..positions {
                    h.y -= 1;
                    // println!("LEFT --> ");
                    update_scene_part1(& h, &mut t,&mut tailed, offset);
                }
            }
            "U" => {
                for _ in 0..positions {
                    h.x -= 1;
                    // println!("UP --> ");
                    update_scene_part1(& h, &mut t,&mut tailed, offset);
                }
            }
            "D" => {
                for _ in 0..positions {
                    h.x += 1;
                    // println!("DOWN --> ");
                    update_scene_part1(& h, &mut t,&mut tailed, offset);
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

pub fn part_two(s: &str) -> usize {
    let offset = 300;
    let mut h = Position {
        x: offset,
        y: offset,
    };
    let t = Position {
        x: offset,
        y: offset,
    };
    let mut tails: Vec<Cell<Position>> = vec![Cell::new(t); 9];
    let mut tailed = vec![vec!['.'; offset * 2 + 1]; offset * 2 + 1];
    // println!("BEGINNING: ");
    // print_scene(&h, &t, offset);
    s.lines().for_each(|line| {
        let splt = line.split(" ");
        let read_line: Vec<&str> = splt.map(|el| el).collect();
        let command = read_line[0];
        let positions = read_line[1].parse::<usize>().unwrap();
        match command {
            "R" => {
                for _ in 0..positions {
                    h.y += 1;
                    // println!("RIGHT --> ");
                    update_scene_part2(&h, &mut tails, &mut tailed, offset);
                }
            }
            "L" => {
                for _ in 0..positions {
                    h.y -= 1;
                    // println!("LEFT --> ");
                    update_scene_part2(&h, &mut tails, &mut tailed, offset);
                }
            }
            "U" => {
                for _ in 0..positions {
                    h.x -= 1;
                    // println!("UP --> ");
                    update_scene_part2(&h, &mut tails, &mut tailed, offset);
                }
            }
            "D" => {
                for _ in 0..positions {
                    h.x += 1;
                    // println!("DOWN --> ");
                    update_scene_part2(&h, &mut tails, &mut tailed, offset);
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
