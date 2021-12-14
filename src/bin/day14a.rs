use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let (polymer, rest) = input.split_once("\n\n").unwrap();
    let mut polymer = polymer.to_string();
    let mut rules = HashMap::new();
    for line in rest.lines() {
        let (key, val) = line.split_once(" -> ").unwrap();
        rules.insert(key, val.chars().next().unwrap()); 
    }

    for iter in 0..10 {
        // println!("Iteration {}: {}", iter, polymer);
        let mut chars = polymer.chars().collect::<Vec<_>>();
        let insertions = polymer.chars().collect::<Vec<_>>().windows(2).enumerate().filter_map(|w| {
            let (i, w) = w;
            let key = format!("{}{}", w[0], w[1]);
            if rules.contains_key(key.as_str()) {
                Some((i, rules[key.as_str()]))
            } else {
                None
            }
        }).collect::<Vec<_>>();

        for (i, c) in insertions.iter().rev() {
            chars.insert(*i + 1, *c);
        }
        
        polymer = chars.iter().collect::<String>();
    }

    let char_counts = polymer.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    let most_common = char_counts.iter().max_by_key(|(_, v)| *v).unwrap();
    let least_common = char_counts.iter().min_by_key(|(_, v)| *v).unwrap();
    println!("{}", most_common.1 - least_common.1); 


}