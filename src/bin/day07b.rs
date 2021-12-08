fn main() {
    let input = include_str!("../../input.txt");
    let mut a = input.split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    a.sort();
    let ans = (0..1000)
        .map(|m| {
            a.iter().map(|x| {
                let d = (x - m).abs();
                d * (d + 1) / 2
            }).sum::<i64>()
        }).min().unwrap();
    println!("{}", ans);
}
