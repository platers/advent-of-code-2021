use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let count = input.lines().map(|l| l.parse::<i32>().unwrap()).collect_vec()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count();
    println!("{}", count);
}
