fn main() {
    let input = include_str!("../../input.txt");
    let mut grid = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let n = grid.len();
    let m = grid[0].len();
    let mut iter = 0;
    loop {
        let mut new_grid = grid.clone();
        let mut changed = false;
        for y in 0..n {
            for x in 0..m {
                let next_x = (x + 1) % m;
                if grid[y][x] == '>' && grid[y][next_x] == '.' {
                    new_grid[y][x] = '.';
                    new_grid[y][next_x] = '>';
                    changed = true;
                }
            }
        }
        grid = new_grid;
        new_grid = grid.clone();
        for y in 0..n {
            for x in 0..m {
                let next_y = (y + 1) % n;
                if grid[y][x] == 'v' && grid[next_y][x] == '.' {
                    new_grid[y][x] = '.';
                    new_grid[next_y][x] = 'v';
                    changed = true;
                }
            }
        }

        iter += 1;
        if !changed {
            println!("{}", iter);
            break;
        }

        grid = new_grid;
    }
}
