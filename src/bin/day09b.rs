fn main() {
    let input = include_str!("../../input.txt");
    let mut grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();
    
    let mut sizes = (0..grid.len()).flat_map(|i| (0..grid[0].len()).map(|j| 
        dfs(&mut grid, i, j)).collect::<Vec<_>>()).collect::<Vec<_>>();
    sizes.sort();
    let ans:u32 = sizes.iter().rev().take(3).product();

    println!("{}", ans);
}

fn dfs(grid: &mut Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    if grid[i][j] == 9 { return 0; }
    grid[i][j] = 9;

    let mut adj = vec![];
    if i > 0 { adj.push((i - 1, j)); }
    if j > 0 { adj.push((i, j - 1)); }
    if i < grid.len() - 1 { adj.push((i + 1, j)); }
    if j < grid[0].len() - 1 { adj.push((i, j + 1)); }

    adj.iter().map(|(i, j)| dfs(grid, *i, *j)).sum::<u32>() + 1
}