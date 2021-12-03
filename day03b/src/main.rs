fn main() {
    let input = include_str!("../input.txt")
        .lines().collect::<Vec<_>>();
    
    let oxy = solve(true, &input);
    let co2 = solve(false, &input);
    println!("{} {} {}", oxy, co2, oxy * co2);
}

fn solve(most_common: bool, lines: &Vec<&str>) -> isize {
    let m = lines[0].len();
    let mut lines = lines.clone();
    for b in 0..m {
        let ones = lines.iter().
            fold(0, |mut acc, l| if l.as_bytes()[b] == b'1' {acc += 1; acc} else {acc});
        let n = lines.len();
        let want_ones = if most_common {
            ones >= n - ones
        } else {
            n - ones == 0 || (ones > 0 &&
                ones < n - ones)
        };

        // println!("ones {} want {}", ones, want_ones);
        lines = lines.into_iter().filter(|l| {
            want_ones == (l.as_bytes()[b] == b'1')
        }).collect();
        // println!("{:?}", lines);
    }

    assert_eq!(lines.len(), 1);

    isize::from_str_radix(lines[0], 2).unwrap()
}
