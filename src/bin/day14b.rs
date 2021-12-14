use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt");
    let (polymer, rest) = input.split_once("\n\n").unwrap();
    let mut rules = HashMap::new();
    for line in rest.lines() {
        let (key, val) = line.split_once(" -> ").unwrap();
        let ab = vec![key.chars().nth(0).unwrap(), val.chars().next().unwrap()];
        let ba = vec![val.chars().next().unwrap(), key.chars().nth(1).unwrap()];
        let key = vec![key.chars().nth(0).unwrap(), key.chars().nth(1).unwrap()];
        rules.insert(key, vec![ab, ba]); 
    }
    let first = polymer.chars().nth(0).unwrap();
    let last = polymer.chars().last().unwrap();
    let mut polymer = polymer.chars().collect::<Vec<char>>().windows(2).fold(HashMap::new(), |mut acc, pair| {
        *acc.entry(pair.to_vec()).or_insert(0u64) += 1;
        acc
    });

    for iter in 0..40 {
        let mut new_polymer = HashMap::new();
        for (key, cnt) in polymer {
            match rules.get(&key) {
                Some(val) => {
                    for next in val.clone() {
                        *new_polymer.entry(next).or_insert(0) += cnt;
                    }
                },
                None => {}
            }
        }
        polymer = new_polymer;
    }

    let mut char_counts = polymer.iter().fold(HashMap::new(), |mut acc, c| {
        let (key, &val) = c;
        for c in key {
            *acc.entry(*c).or_insert(0) += val;
        }
        acc
    });
    *char_counts.entry(first).or_insert(0) += 1;
    *char_counts.entry(last).or_insert(0) += 1;
    let most_common = char_counts.iter().max_by_key(|(_, v)| *v).unwrap();
    let least_common = char_counts.iter().min_by_key(|(_, v)| *v).unwrap();
    println!("{}", (most_common.1 - least_common.1) / 2); 


}