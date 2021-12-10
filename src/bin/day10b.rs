const BRACKETS: [(char, char, u32); 4] = [('(', ')', 3), ('[', ']', 57), ('{', '}', 1197), ('<', '>', 25137)];

fn main() {
    let input = include_str!("../../input.txt");
    let mut scores = input.lines().map(|line| {
        let mut stack = Vec::new();
        for c in line.chars() {
            let b = BRACKETS.iter().find(|b| b.0 == c || b.1 == c).unwrap();
            if c == b.0 {
                stack.push(c);
            } else if c == b.1 {
                if stack.pop().unwrap() != b.0 {
                    return 0;
                }
            }
        }
        
        stack.iter().rev().fold(0, |acc, c|
            acc * 5 + BRACKETS.iter().position(|b| b.0 == *c).unwrap() as u128 + 1
)
    }).filter(|&x| x != 0).collect::<Vec<u128>>();
    scores.sort();
    println!("{}", scores[(scores.len() - 1) / 2]);
}