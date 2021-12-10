const BRACKETS: [(char, char, u32); 4] = [('(', ')', 3), ('[', ']', 57), ('{', '}', 1197), ('<', '>', 25137)];

fn main() {
    let input = include_str!("../../input.txt");
    let ans:u32 = input.lines().map(|line| {
        let mut stack = Vec::new();
        for c in line.chars() {
            let b = BRACKETS.iter().find(|b| b.0 == c || b.1 == c).unwrap();
            if c == b.0 {
                stack.push(c);
            } else if c == b.1 {
                if stack.pop().unwrap() != b.0 {
                    return b.2;
                }
            }
        }
        return 0;
    }).sum();

    println!("{}", ans);
}