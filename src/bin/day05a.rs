use regex::Regex;
use std::cmp::*;

struct Pt {
    x: i32,
    y: i32,
}

struct Line {
    a: Pt,
    b: Pt,
}

fn main() {
    let input = include_str!("../../input.txt");
    // parse lines like "0,9 -> 5,9"
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    let lines = input.lines().map(|line| {
        let caps = re.captures(line).unwrap();
        Line {
            a: Pt {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
            },
            b: Pt {
                x: caps[3].parse().unwrap(),
                y: caps[4].parse().unwrap(),
            },
        }
    });

    let mut grid = vec![vec![0; 1000]; 1000];

    for line in lines {
        let a = line.a;
        let b = line.b;

        if a.x == b.x {
            // vertical line
            for y in min(a.y, b.y)..=max(a.y, b.y) {
                grid[a.x as usize][y as usize] += 1;
            }
        } else if a.y == b.y {
            // horizontal line
            for x in min(a.x, b.x)..=max(a.x, b.x) {
                grid[x as usize][a.y as usize] += 1;
            }
        }
    }

    let num_points_at_least_2 = grid
        .iter()
        .map(|row| row.iter().filter(|&&x| x >= 2).count())
        .sum::<usize>();

    println!("{}", num_points_at_least_2);
}
