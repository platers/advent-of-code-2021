use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let edges = input.lines().map(|line| {
        let mut parts = line.split("-");
        (parts.next().unwrap(), parts.next().unwrap())
    }).collect::<Vec<_>>();

    let mut g = HashMap::new();
    for (a, b) in edges {
        g.entry(String::from(a)).or_insert(vec![]).push(b);
        g.entry(String::from(b)).or_insert(vec![]).push(a);
    }

    let mut small_caves = vec![];
    let num_paths = dfs(&g, &String::from("start"), &mut small_caves, false);
    println!("{}", num_paths);
}

fn dfs(g: &HashMap<String, Vec<&str>>, cur: &String, small_caves: &mut Vec<String>, twice: bool) -> usize {
    if cur == "end" {
        return 1;
    }

    // println!("{} {:?} {}", cur, small_caves, twice);
    let repeat = small_caves.contains(cur);
    if repeat && (twice || cur == "start") {
        return 0;
    }
    if (cur.to_lowercase() == *cur) && !repeat {
        small_caves.push(String::from(cur));
    }

    let mut num_paths = 0;

    for next in g.get(cur).unwrap() {
        num_paths += dfs(g, &String::from(*next), small_caves, repeat || twice);
    }

    if cur.to_lowercase() == *cur && !repeat {
        small_caves.pop();
    }

    num_paths
}