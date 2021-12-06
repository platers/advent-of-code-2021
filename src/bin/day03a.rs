fn main() {
    let input = include_str!("../../input.txt");
    let bins = input.lines().map(|l| l).collect::<Vec<_>>();
    let m = bins[0].len();
    let n = bins.len();

    let cnts = (0..m).map(|i| {
        bins
            .iter()
            .map(|b| b.chars().nth(i).unwrap())
            .filter(|&c| c == '1')
            .count()
    }).collect::<Vec<_>>();

    let mut eps = 0;
    let mut gamma = 0;
    for i in 0..m {
        if cnts[i] > n - cnts[i] {
            gamma += 1 << (m - i - 1);
        } else {
            eps += 1 << (m - i - 1);
        }
    }

    println!("{}", gamma * eps);
}
