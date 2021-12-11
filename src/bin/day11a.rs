fn main() {
    let input = include_str!("../../input.txt");
    // parse grid
    let mut grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        }).collect();
    let n = grid.len();
    let m = grid[0].len();

    // surround grid with min i32
    grid.insert(0, vec![std::i32::MIN; m]);
    grid.push(vec![std::i32::MIN; m]);
    for row in grid.iter_mut() {
        row.insert(0, std::i32::MIN);
        row.push(std::i32::MIN);
    }

    let mut flashes = 0;

    for step in 1..=100 {
        for x in 1..n+1 {
            for y in 1..m+1 {
                grid[x][y] += 1;
            }
        }
        for row in 1..n+1 {
            for col in 1..m+1 {
                flashes += flash(&mut grid, row, col);
            }
        }
        for x in 1..n+1 {
            for y in 1..m+1 {
                grid[x][y] = std::cmp::max(grid[x][y], 0);
            }
        }
    }

    println!("{}", flashes);
}

fn flash(grid: &mut Vec<Vec<i32>>, x: usize, y: usize) -> u32 {
    if grid[x][y] <= 9 {
        return 0;
    }

    grid[x][y] = i32::MIN;
    // add one to adjacent cells
    let mut flashes = 1;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            grid[nx as usize][ny as usize] += 1;
            flashes += flash(grid, nx as usize, ny as usize);
        }
    }

    flashes
}