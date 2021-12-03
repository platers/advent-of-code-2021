fn main() {
    let input = include_str!("../input.txt")
        .lines().collect::<Vec<_>>();
    let n = input.len();
    let m = input[0].len();
    let cnts:Vec<usize> = input.iter().fold(vec![0; m], |mut acc, l| {
        for (i, c) in l.chars().enumerate() {
            if c == '1' {
                acc[i] += 1;
            }
        }
        acc
    });

    let gamma:String = cnts.iter().map(|c| {
        if c > &(&n / 2) {
            '1'
        } else {
            '0'
        }
    }).collect();
    println!("Gamma {}", gamma);
    let eps:String = cnts.iter().map(|c| {
        if c <= &(&n / 2) {
            '1'
        } else {
            '0'
        }
    }).collect();
    println!("Eps {}", eps);

    let gamma = isize::from_str_radix(&gamma, 2).unwrap();
    let eps = isize::from_str_radix(&eps, 2).unwrap();
    println!("Ans {}", gamma * eps);
}
