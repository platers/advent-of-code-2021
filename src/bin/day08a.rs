fn main() {
    let input = include_str!("../../input.txt");
    let lines = input.lines().map(|l| 
        l.split_once(" | ").unwrap()).collect::<Vec<_>>();
    let num_unique:usize = lines.iter().map(|(_, r)| 
        r.split_whitespace()
        .map(|s| s.len())
        .filter(|&l| [2, 3, 4, 7].contains(&l))
        .count()).sum();

    println!("{}", num_unique);
}
