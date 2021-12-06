fn main() {
    let input = include_str!("../../input.txt");
    let (depth, x, _) = input.lines()
        .map(|line| {
            let mut parts = line.split(" ");
            let opcode = parts.next().unwrap();
            let depth = parts.next().unwrap().parse::<usize>().unwrap();
            (opcode, depth)
        })
        .fold((0, 0, 0), |(depth, x, aim), (opcode, val)| {
            match opcode {
                "forward" => (depth + val * aim, x + val, aim),
                "down" => (depth, x, aim + val),
                "up" => (depth, x, aim - val),
                _ => panic!("Unknown opcode: {}", opcode),
            }
        });
    println!("{}", depth * x);
}
