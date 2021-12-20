use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    let (map, image_str) = input.split_once("\n\n").unwrap();
    let map = map.chars().collect::<Vec<_>>();
    let mut image = HashSet::new();
    for (y, line) in image_str.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                image.insert((y as isize, x as isize));
            }
        }
    }

    let n = image_str.lines().count();
    let m = image_str.lines().next().unwrap().len();

    for iter in 0..2 {
        let mut new_image = HashSet::new();
        let xs = image.iter().map(|&(y, x)| x).collect::<Vec<_>>();
        let ys = image.iter().map(|&(y, x)| y).collect::<Vec<_>>();
        let min_x = xs.iter().min().unwrap() - 20;
        let max_x = xs.iter().max().unwrap() + 20;
        let min_y = ys.iter().min().unwrap() - 20;
        let max_y = ys.iter().max().unwrap() + 20;
        println!("{} {} {} {}", min_x, max_x, min_y, max_y);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let mut bin = 0usize;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let (nx, ny) = (x + dx, y + dy);
                        bin <<= 1;
                        if image.contains(&(ny, nx)) {
                            bin |= 1;
                        }
                    }
                }
                if map[bin] == '#' {
                    new_image.insert((y, x));
                }
            }
        }

        image = new_image;
    }

    let mut count = 0;
    for (y, x) in image.iter() {
        if *x >= -5 && *x < 5 + m as isize && *y >= -5 && *y < 5 + n as isize {
            count += 1;
        }
    }

    println!("{}", count);
}
