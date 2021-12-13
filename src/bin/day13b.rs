fn main() {
    let input = include_str!("../../input.txt");
    let (dots, folds) = input.split_once("\n\n").unwrap();
    let mut dots = dots.lines().map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
        .collect::<Vec<_>>();
    let folds = folds.lines().map(|line| line.split(' ').nth(2).unwrap()
        .split_once('=').unwrap())
        .map(|(x, y)| (x, y.parse::<i32>().unwrap())).collect::<Vec<_>>();

    for fold in folds {
        let (axis, pos) = fold;
        dots = dots.iter().map(|(x, y)| {
            if axis == "x" {
                let d = *x - pos;
                if d > 0 {
                    (pos - d, *y)
                } else {
                    (*x, *y)
                }
            } else {
                let d = *y - pos;
                if d > 0 {
                    (*x, pos - d)
                } else {
                    (*x, *y)
                }
            }
        }).collect::<Vec<_>>();
        dots.sort();
        dots.dedup();
    }

    let mut grid = vec![vec!['.'; 40]; 6];
    for (x, y) in dots {
        grid[y as usize][x as usize] = '#';
    }
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}