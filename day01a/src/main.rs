use itertools::Itertools;

fn main() {
    println!("Hello, world!");
    let input = include_str!("../input.txt");
    let lines = input.lines().map(|l| l.parse::<i32>().unwrap()).collect_vec();
    let mut count = 0;
    for i in 1..lines.len() {
        if lines[i] > lines[i - 1] {
            count += 1;
        }
    }
    println!("{}", count);
}
