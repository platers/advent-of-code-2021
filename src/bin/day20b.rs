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

    let iters = 50isize;
    for iter in 0..iters {
        println!("{}", iter);
        let mut new_image = HashSet::new();
        let min_x = -iters * 3;
        let max_x = iters * 3 + m as isize;
        let min_y = -iters * 3;
        let max_y = iters * 3 + n as isize;

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
        if *x >= -(iters as isize)
            && *x < iters as isize + m as isize
            && *y >= -(iters as isize)
            && *y < iters as isize + n as isize
        {
            count += 1;
        }
    }

    println!("{}", count);
}
