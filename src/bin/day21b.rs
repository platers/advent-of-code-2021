use std::cmp::min;
use itertools::iproduct;
// state = (p1 score, p2 score, p1 pos, p2 pos)
// state sizez = (21, 21, 10, 10)
fn main() {
    let mut dp = vec![vec![vec![vec![(0u128, 0u128); 10]; 10]; 25]; 25];
    for s in (0..21*2).rev() {
        for s2 in (0..=min(21, s)).rev() {
            let s1 = s - s2;
            if s1 >= 21 {
                continue;
            }

            for p1 in 0..10 {
                for p2 in 0..10 {

                    for (r1, r2, r3) in iproduct!(1..=3, 1..=3, 1..=3) {
                        let r = r1 + r2 + r3;
                        let n_p1 = (p1 + r) % 10;
                        let n_s1 = s1 + n_p1 + 1;
                        if n_s1 >= 21 {
                            dp[s1][s2][p1][p2].0 += 1;
                        } else {
                            let (a, b) = dp[s2][n_s1][p2][n_p1];
                            dp[s1][s2][p1][p2].0 += b;
                            dp[s1][s2][p1][p2].1 += a;
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", dp[0][0][4][8]);
} 
