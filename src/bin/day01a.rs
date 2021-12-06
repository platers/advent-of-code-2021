use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt");
    let times_increased = input.lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect_vec()
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count();
    println!("{}", times_increased);
}
