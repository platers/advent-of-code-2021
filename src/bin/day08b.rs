fn main() {
    let input = include_str!("../../input.txt");
    let ans = input
        .lines()
        .map(|line| {
            let (inputs, outputs) = line.split_once(" | ").unwrap();
            let inputs = inputs.split_whitespace().collect::<Vec<_>>();
            let outputs = outputs.split_whitespace().collect::<Vec<_>>();
            let one = inputs.iter().find(|w| w.len() == 2).unwrap();
            let four = inputs.iter().find(|w| w.len() == 4).unwrap();
            
            outputs.iter().map(|word| {
                match word.len() {
                    2 => 1,
                    4 => 4,
                    3 => 7,
                    7 => 8,
                    _ => {
                        let one = intersection_len(one, &word);
                        let four = intersection_len(four, &word);
                        match (word.len(), one, four) {
                            (5, 1, 2) => 2,
                            (5, 1, 3) => 5,
                            (6, 1, 3) => 6,
                            (6, 2, 4) => 9,
                            (6, 2, 3) => 0,
                            (5, 2, 3) => 3,
                            _ => panic!("{} {} {}", word.len(), one, four)
                        }
                    }
                }
            })
            .fold(0, |acc, d| acc * 10 + d)
        }).sum::<u32>();
    println!("{}", ans);
}

fn intersection_len(a: &str, b: &str) -> usize {
    a.chars().map(|c| b.contains(c)).filter(|x| *x).count()
}
