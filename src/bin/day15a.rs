use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let input = include_str!("../../input.txt");
    let mut risk = HashMap::new();
    let n = input.lines().count() as i32;
    let m = input.lines().next().unwrap().len() as i32;

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            risk.entry((i as i32, j as i32)).or_insert(c.to_digit(10).unwrap() as i32);
        }
    }

    let mut dist = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(((0, 0), 0)));
    dist.insert((0i32, 0i32), 0);

    while let Some(Reverse(((x, y), d))) = queue.pop() {
        println!("{:?}", (x, y));
        for (dx, dy) in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || ny < 0 || nx >= n * 5 || ny >= m * 5 {
                continue;
            }

            let nd = d + risk.get(&(nx, ny)).unwrap();
            if !dist.contains_key(&(nx, ny)) || dist.get(&(nx, ny)).unwrap() > &nd {
                dist.insert((nx, ny), nd);
                queue.push(Reverse(((nx, ny), nd)));
            }
        }
    }

    println!("{}", dist.get(&(n - 1, m - 1)).unwrap());

}