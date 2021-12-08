fn main() {
    let input = include_str!("../../input.txt");
    let mut a = input.split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    a.sort();
    let median = a[a.len() / 2];
    let sum_dist_from_median = a.iter().map(|x| (x - median).abs()).sum::<i64>();
    println!("{}", sum_dist_from_median);
}
