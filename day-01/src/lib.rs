 

pub fn process_part1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|el|  { 
                el
                    .split("\n")
                    .map(|elf| 
                            {
                            let parse_result = elf.parse::<i32>();
                            match parse_result {
                                Err(_) => 0,
                                Ok(val) => {
                                    // println!("{:?}", val);
                                    val
                                }
                            }
                            })
                    .sum::<i32>()
                })
        .max()
        .unwrap()
}

pub fn process_part2(input: &str) -> i32 {
    let mut out = input
        .split("\n\n")
        .map(|el|  { 
                el
                    .split("\n")
                    .map(|elf| 
                            {
                            let parse_result = elf.parse::<i32>();
                            match parse_result {
                                Err(_) => 0,
                                Ok(val) => {
                                    // println!("{:?}", val);
                                    val
                                }
                            }
                            })
                    .sum::<i32>()
                })
        .collect::<Vec<i32>>();
    out
        .sort_by(|a, b| b.cmp(a));
    out
        .truncate(3);
    out
        .into_iter()
        .sum::<i32>() 
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn it_works() {
        let input = "1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000";
        let result = process_part1(&input);
        assert_eq!(result, 24000);
    }
}
