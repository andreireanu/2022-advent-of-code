#![feature(iter_next_chunk)]
use std::collections::{HashMap, hash_map::Entry};

pub fn char_to_val(s: char) -> u16 {
    if s.is_uppercase() {
        (s as u16) - 38
    } else {
        (s as u16) - 96 
    }
    }

pub fn calculate_score_first(s: &String) -> u16 {
    s
    .lines()
    .map(|line| {
        let splt = line
            .split_at(line.len()/2);
        let mut char_counts: HashMap<char,i32> = HashMap::new();
        for c in splt.0.chars() {
            let _ = *char_counts.entry(c).or_insert(0);
        }
        for c in splt.1.chars() {
            if let Entry::Occupied(_v) = char_counts.entry(c) {
                *char_counts.entry(c).or_default() += 1;
            }
        }
        char_counts
            .into_iter()
            .filter(|&(_k, v)| v >= 1  )
            .map(|(k, _v)| char_to_val(k))
            .sum::<u16>()
        
    })
    .sum::<u16>()
}

pub fn calculate_score_second(s: &String) -> u16 {
    let mut lines = s.lines();
    let mut sum: u16 = 0;
    while let Ok(arr) =  lines.next_chunk::<3>() {
        let mut char_counts: HashMap<char,i32> = HashMap::new();
        for c in arr[0].chars() {
            let _ = *char_counts.entry(c).or_insert(1);
        }
        for c in arr[1].chars() {
            char_counts.entry(c).and_modify(|counter| {if counter == &1 {*counter += 1}});
        }
        for c in arr[2].chars() {
            char_counts.entry(c).and_modify(|counter| {if counter == &2 {*counter += 3}});
        }
        let temp = char_counts
            .into_iter()
            .filter(|&(_k, v)| v >= 3  )
            .map(|(k, _v)| char_to_val(k))
            .sum::<u16>();
        sum += temp;
    }  
    sum
    }
 