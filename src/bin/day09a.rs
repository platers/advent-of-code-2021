fn main() {
    let input = include_str!("../../input.txt");
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    let cnt:u32 = (0..grid.len()).map(|i| {(0..grid[i].len()).map(|j| {
                    // check cell is less than all neighbours
                    let mut adj = vec![];
                    if i > 0 { adj.push((i - 1, j)); }
                    if j > 0 { adj.push((i, j - 1)); }
                    if i < grid.len() - 1 { adj.push((i + 1, j)); }
                    if j < grid[0].len() - 1 { adj.push((i, j + 1)); }
                    let lowest = adj.iter()
                        .map(|(i, j)| grid[*i][*j])
                        .all(|n| n > grid[i][j]);
                    if lowest {
                        return grid[i][j] + 1;
                    }
                    0
                }).sum::<u32>()
        }).sum();
    
    println!("{}", cnt);
}