use std::collections::HashMap;

#[derive(Debug)]
pub enum CommandsAndOutput {
    // posible line outcome, either a command or an output
    CdHome,         // cd home
    CdInto(String), // cd into folder
    CdUp,           // cd up
    Size(usize),    // file size print
    None,           // command or output not needed for determining size
}

pub fn parse_line(s: &str) -> CommandsAndOutput {
    let out: Vec<&str> = s.split(' ').collect();
    match out[0].parse::<usize>() {
        Err(_) => match (out[0], out[1]) {
            ("$", "ls") => CommandsAndOutput::None,
            _ => match out.get(2) {
                None => CommandsAndOutput::None,
                Some(dir) => match *dir {
                    "/" => CommandsAndOutput::CdHome,
                    ".." => CommandsAndOutput::CdUp,
                    _ => CommandsAndOutput::CdInto(dir.to_string()),
                },
            },
        },
        Ok(value) => CommandsAndOutput::Size(value),
    }
}

pub fn part_one(s: &str) -> usize {
    let mut dir_sizes = HashMap::new();
    let mut wd = vec!["/".to_string()];
    dir_sizes.entry("/".to_string()).or_insert(0);
    s.lines().for_each(|line| {
        let message = parse_line(line);
        match message {
            CommandsAndOutput::CdHome => {
                wd.clear();
                wd.push("/".to_string());
            }
            CommandsAndOutput::CdInto(dir) => {
                let temp = wd[&wd.len() - 1].to_owned() + &dir + "/";
                wd.push(temp);
            }
            CommandsAndOutput::CdUp => {
                wd.pop();
            }
            CommandsAndOutput::Size(size) => {
                wd.iter().for_each(|dir| {
                    dir_sizes
                        .entry(dir.to_string())
                        .and_modify(|dir| *dir += size)
                        .or_insert(size);
                });
            }
            CommandsAndOutput::None => {}
        };
    });
    dir_sizes
        .values()
        .filter(|&val| *val < 100000)
        .sum::<usize>()
}

pub fn part_two(s: &str) -> usize {
    let mut dir_sizes = HashMap::new();
    let mut wd = vec!["/".to_string()];
    dir_sizes.entry("/".to_string()).or_insert(0);
    s.lines().for_each(|line| {
        let message = parse_line(line);
        match message {
            CommandsAndOutput::CdHome => {
                wd.clear();
                wd.push("/".to_string());
            }
            CommandsAndOutput::CdInto(dir) => {
                let temp = wd[&wd.len() - 1].to_owned() + &dir + "/";
                wd.push(temp);
            }
            CommandsAndOutput::CdUp => {
                wd.pop();
            }
            CommandsAndOutput::Size(size) => {
                wd.iter().for_each(|dir| {
                    dir_sizes
                        .entry(dir.to_string())
                        .and_modify(|dir| *dir += size)
                        .or_insert(size);
                });
            }
            CommandsAndOutput::None => {}
        };
    });
    let required = dir_sizes["/"] - 40000000;
    let values_iter = dir_sizes.into_values();
    let mut filtered = values_iter
        .filter(|&value| value >= required)
        .collect::<Vec<usize>>();
    filtered.sort();
    filtered[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_part_one() {
        let input = "$ cd /
$ ls
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let result = part_one(&input);
        assert_eq!(result, 95437);
    }

    #[test]
    fn test_part_two() {
        let input = "$ cd /
$ ls
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let result = part_two(&input);
        assert_eq!(result, 24933642);
    }
}
