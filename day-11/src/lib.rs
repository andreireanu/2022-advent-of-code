#![feature(iter_next_chunk)]

extern crate nom;
use nom::{IResult, bytes::complete::{tag, take_until}};


fn nom_monkey(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("Monkey ")(input)?;
    let (dummy, id) = take_until(":")(input)?; 
    let id_usize = id.parse::<usize>().unwrap();
    Ok((dummy, id_usize))
  }

  fn nom_items(input: &str) -> IResult<&str, Vec<u16>> {
    let (input, err) = tag("  Starting items: ")(input)?;
    let splt = input.split(", ").collect::<Vec<&str>>();
    let parsed = splt
        .into_iter()
        .map( |element| element.parse::<u16>().unwrap())
        .collect::<Vec<u16>>();
    Ok((err, parsed))
  }
  
  fn nom_operation(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, err) = tag("  Operation: new = old ")(input)?;
    let splt = input.split(" ").collect::<Vec<&str>>();
    Ok((err, splt))
    }

fn nom_divisor(input: &str) -> IResult<&str, u16>  {
    let (divisor, err) = tag("  Test: divisible by ")(input)?;
    let out = divisor.parse::<u16>().unwrap();
    Ok((err, out))
    }

fn nom_true(input: &str) -> IResult<&str, usize>  {
    let (true_val, err) = tag("    If true: throw to monkey ")(input)?;
    let out = true_val.parse::<usize>().unwrap();
    Ok((err, out))
    }

fn nom_false(input: &str) -> IResult<&str, usize>  {
    let (false_val, err) = tag("    If false: throw to monkey ")(input)?;
    let out = false_val.parse::<usize>().unwrap();
    Ok((err, out))
    }

pub fn part_one(s: &str) -> isize {
    let monkeys: &mut Vec<Vec<u16>> = &mut vec![vec![];7];
    dbg!(&monkeys);
    let lines = s.lines().collect::<Vec<&str>>();
    let chunks = lines.chunks(7);
    for chunk in chunks.into_iter() {
        dbg!(&chunk);
        let (_,monkey) = nom_monkey(chunk[0]).unwrap();
        dbg!(&monkey);
        let (_, items) = nom_items(chunk[1]).unwrap();
        dbg!(&items);
        let (_, operation) = nom_operation(chunk[2]).unwrap();
        dbg!(&operation);
        let (_, divisor) = nom_divisor(chunk[3]).unwrap();
        dbg!(&divisor);
        let (_, true_val) = nom_true(chunk[4]).unwrap();
        dbg!(&true_val);
        let (_, false_val) = nom_false(chunk[5]).unwrap();
        dbg!(&false_val);
        for item in items.into_iter() {
            monkeys[monkey].push(item);
        };

        for monkey in monkeys.into_iter() {
            for item in monkey.into_iter() {
                dbg!(&item);
                let value = match operation[1].parse::<u16>() {
                    Ok(result) => { result },
                    Err(_) => { *item},
                };
                dbg!(&value);
                let result = match operation[0] {
                    "+" => {
                        if ((*item + value) / 3) % divisor == 0 {
                            true
                        } else {
                            false
                        }
                    },
                    "*" => {
                        if  ((*item * value) / 3) % divisor == 0 {
                            true
                        } else {
                            false
                        }
                    },
                    _ => { false }
                };
                dbg!(&result);
                match result {
                    true => { 
                        monkeys[true_val].push(*item);
                    },
                    false => { 
                        monkeys[false_val].push(*item);
                    },
                }
            }
        }
    }
    dbg!(&monkeys);
    0
}

pub fn part_two(s: &str) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0";

// Monkey 2:
//   Starting items: 79, 60, 97
//   Operation: new = old * old
//   Test: divisible by 13
//     If true: throw to monkey 1
//     If false: throw to monkey 3

// Monkey 3:
//   Starting items: 74
//   Operation: new = old + 3
//   Test: divisible by 17
//     If true: throw to monkey 0
//     If false: throw to monkey 1";
        let result = part_one(&input);
        assert_eq!(result, 13140);
    }

    #[test]
    fn test_part_two() {
        let input = "0";
        let result = part_two(&input);
        assert_eq!(result, 2);
    }
}
