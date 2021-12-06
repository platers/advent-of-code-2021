fn main() {
    let input = include_str!("../../input.txt");
    let bins = input.lines().map(|l| l).collect::<Vec<_>>();

    let oxy = solve(true, &bins);
    let co2 = solve(false, &bins);
    println!("{} {}", oxy, co2);
    println!("{}", oxy * co2);
}

fn solve(oxy: bool, bins: &Vec<&str>) -> usize {
    let m = bins[0].len();
    let mut bins = bins.clone();

    for i in 0..m {
        let n = bins.len();
        let cnt = bins
            .iter()
            .map(|l| l.chars().nth(i).unwrap())
            .filter(|c| c == &'1')
            .count();
        
        let mut keep = 0;
        if oxy {
            if cnt >= n - cnt { keep = 1; }
        } else {
            if cnt == n || cnt > 0 && cnt < n - cnt { keep = 1; }
        }

        bins.retain(|l| l.chars().nth(i).unwrap() == char::from_digit(keep, 2).unwrap());
        // println!("{:?} {}", bins, keep);
    }

    usize::from_str_radix(bins[0], 2).unwrap()
}

    