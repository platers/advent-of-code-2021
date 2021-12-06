fn main() {
    let input = include_str!("../../input.txt");
    let (depth, x) = input.lines()
        .map(|line| {
            let mut parts = line.split(" ");
            let opcode = parts.next().unwrap();
            let depth = parts.next().unwrap().parse::<usize>().unwrap();
            (opcode, depth)
        })
        .fold((0, 0), |(depth, x), (opcode, val)| {
            match opcode {
                "forward" => (depth, x + val),
                "down" => (depth + val, x),
                "up" => (depth - val, x),
                _ => panic!("Unknown opcode: {}", opcode),
            }
        });
    println!("{}", depth * x);
}
