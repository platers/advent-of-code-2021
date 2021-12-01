use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let lines = input.lines().map(|l| l.parse::<i32>().unwrap()).collect_vec();
    let mut count = 0;
    for i in 1..lines.len()-2 {
        let sumA = lines[i-1] + lines[i] + lines[i+1];
        let sumB = lines[i+2] + lines[i+1] + lines[i];
        if sumA < sumB {
            count += 1;
        }
    }
    println!("{}", count);
}
