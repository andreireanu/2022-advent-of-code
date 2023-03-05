#![feature(iter_next_chunk)]

extern crate nom;
use nom::{IResult, bytes::complete::{tag, take_until}};


fn nom_monkey(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("Monkey ")(input)?;
    let (dummy, id) = take_until(":")(input)?; 
    let id_usize = id.parse::<usize>().unwrap();
    Ok((dummy, id_usize))
  }

  fn nom_items(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, err) = tag("  Starting items: ")(input)?;
    let splt = input.split(", ").collect::<Vec<&str>>();
    let parsed = splt
        .into_iter()
        .map( |element| element.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    Ok((err, parsed))
  }
  
  fn nom_operation(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, err) = tag("  Operation: new = old ")(input)?;
    let splt = input.split(" ").collect::<Vec<&str>>();
    Ok((err, splt))
    }

fn nom_divisor(input: &str) -> IResult<&str, u32>  {
    let (divisor, err) = tag("  Test: divisible by ")(input)?;
    let out = divisor.parse::<u32>().unwrap();
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

pub fn part_one(s: &str) -> usize {
    let monkeys: &mut Vec<Vec<u32>> = &mut vec![vec![];9];
    let count: &mut Vec<usize> = &mut vec![0;9];
    for round in 0..20 {
        let lines = s.lines().collect::<Vec<&str>>();
        let chunks = lines.chunks(7);
        for chunk in chunks.into_iter() {
            let (_,monkey) = nom_monkey(chunk[0]).unwrap();
            let (_, items) = nom_items(chunk[1]).unwrap();
            let (_, operation) = nom_operation(chunk[2]).unwrap();
            let (_, divisor) = nom_divisor(chunk[3]).unwrap();
            let (_, true_val) = nom_true(chunk[4]).unwrap();
            let (_, false_val) = nom_false(chunk[5]).unwrap();
            if round == 0 {
                for item in items.into_iter() {
                    monkeys[monkey].push(item);
                };
            };
            count[monkey] += monkeys[monkey].len();
            for i in 0..monkeys[monkey].len()  {
                let item = monkeys[monkey][i];
                let value = match operation[1].parse::<u32>() {
                    Ok(result) => { result },
                    Err(_) => { item},
                };
                let result = match operation[0] {
                    "+" => {
                        (item + value) / 3  
                        },
                    "*" => {
                        (item * value) / 3
                    },
                    _ => { 0 }
                };
                match result % divisor == 0 {
                    true => { 
                        monkeys[true_val].push(result);
                    },
                    false => { 
                        monkeys[false_val].push(result);
                    },
                }
            }
            monkeys[monkey].clear();
        };
    };
    count.sort_by(|a, b| b.partial_cmp(a).unwrap());
    dbg!(&count);
    count[0] * count[1]
}

pub fn part_two(s: &str) -> usize {
    let monkeys: &mut Vec<Vec<u32>> = &mut vec![vec![];9];
    let count: &mut Vec<usize> = &mut vec![0;9];
    for round in 0..20 {
        let lines = s.lines().collect::<Vec<&str>>();
        let chunks = lines.chunks(7);
        for chunk in chunks.into_iter() {
            let (_,monkey) = nom_monkey(chunk[0]).unwrap();
            let (_, items) = nom_items(chunk[1]).unwrap();
            let (_, operation) = nom_operation(chunk[2]).unwrap();
            let (_, divisor) = nom_divisor(chunk[3]).unwrap();
            let (_, true_val) = nom_true(chunk[4]).unwrap();
            let (_, false_val) = nom_false(chunk[5]).unwrap();
            if round == 0 {
                for item in items.into_iter() {
                    monkeys[monkey].push(item);
                };
            };
            count[monkey] += monkeys[monkey].len();
            for i in 0..monkeys[monkey].len()  {
                let item = monkeys[monkey][i];
                let value = match operation[1].parse::<u32>() {
                    Ok(result) => { result },
                    Err(_) => { item},
                };
                let result = match operation[0] {
                    "+" => {
                        (item + value) / 3  
                        },
                    "*" => {
                        (item * value) / 3
                    },
                    _ => { 0 }
                };
                match result % divisor == 0 {
                    true => { 
                        monkeys[true_val].push(result);
                    },
                    false => { 
                        monkeys[false_val].push(result);
                    },
                }
            }
            monkeys[monkey].clear();
        };
    };
    count.sort_by(|a, b| b.partial_cmp(a).unwrap());
    dbg!(&count);
    count[0] * count[1]
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
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        let result = part_one(&input);
        assert_eq!(result, 10605);
    }

    #[test]
    fn test_part_two() {
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
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        let result = part_two(&input);
        assert_eq!(result, 2713310158);
    }
}
