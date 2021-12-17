use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let edges = input.lines().map(|line| {
        let mut parts = line.split("-");
        (parts.next().unwrap(), parts.next().unwrap())
    }).collect::<Vec<_>>();

    let mut g = HashMap::new();
    for (a, b) in edges {
        g.entry(a).or_insert(vec![]).push(b);
        g.entry(b).or_insert(vec![]).push(a);
    }

    let mut small_caves = vec![];
    let num_paths = dfs(&g, &String::from("start"), &mut small_caves);
    println!("{}", num_paths);
}

fn dfs<'a>(g: &HashMap<&'a str, Vec<&'a str>>, cur: &'a str, small_caves: &mut Vec<&'a str>) -> usize {
    if cur == "end" {
        return 1;
    }

    if small_caves.contains(&cur) {
        return 0;
    }

    let mut num_paths = 0;
    if cur.to_lowercase() == *cur {
        small_caves.push(&cur);
    }

    for next in g.get(cur).unwrap() {
        num_paths += dfs(g, &next, small_caves);
    }

    if cur.to_lowercase() == *cur {
        small_caves.pop();
    }

    num_paths
}