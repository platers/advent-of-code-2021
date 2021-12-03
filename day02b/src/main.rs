fn main() {
    let input = include_str!("../input.txt").lines();
    let mut depth = 0;
    let mut x = 0;
    let mut aim = 0;

    for line in input {
        let tokens:Vec<_> = line.split_whitespace().collect();
        let s = tokens[0];
        let n:u32 = tokens[1].parse().unwrap();
        
        match s {
            "forward" => {x += n; depth += aim * n},
            "down" => {aim += n},
            "up" => {aim -= n},
            _ => {panic!("invalid instruction")}
        }
    }

    println!("Depth {}, X {}", depth, x);
    println!("Ans {}", depth * x);
}
