use itertools::Itertools;
use num::integer::gcd;

type Point = (isize, isize);
type Line = (Point, Point);

fn main() {
    let input = include_str!("../input.txt");
    let lines:Vec<Line> = input.lines().map(|l| {
        l.split_whitespace()
            .filter(|t| t.chars().nth(0).unwrap().is_digit(10))
            .map(|t| {
               t.split(',')
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap()
            }).collect_tuple().unwrap()
    }).collect();

    println!("{:?}", lines);

    let mut grid = [[0; 1000]; 1000];

    for line in lines {
        let a = line.0;
        let b = line.1;
        let dx = b.0 - a.0;
        let dy = b.1 - a.1;
        let g = gcd(dx, dy);

        // adjust for each part
        if !(dx == 0 || dy == 0 || dx.abs() == dy.abs()) {
            continue;
        }

        let mut x = a.0;
        let mut y = a.1;
        loop {
            grid[x as usize][y as usize] += 1;
            if x == b.0 && y == b.1 {break}
            x += dx / g;
            y += dy / g;
        }
    }

    let ans = grid.iter().flatten()
        .filter(|x| **x >= 2)
        .count();
    println!("{}", ans);
    
}
