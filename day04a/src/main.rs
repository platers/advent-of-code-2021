fn main() {
    let input = include_str!("../input.txt");
    let (first, rest) = input.split_once("\n\n").unwrap();
    let picks = first.split(',').map(|s| s.parse::<i32>().unwrap());
    let mut boards:Vec<Vec<i32>> = rest.split("\n\n")
        .map(|s| s.split_whitespace().map(|x| x.parse().unwrap()).collect())
        .collect();
    'out: for pick in picks {
        boards.iter_mut().for_each(|b| b.iter_mut().for_each(|x| {
            if x == &pick {*x = -1};
        }));
        
        match boards.iter().find(
            |b| (0..5).any(|i| (0..5).all(|j| b[i*5 + j] == -1)) ||
                (0..5).any(|i| (0..5).all(|j| b[i + j*5] == -1))) {
            Some(b) => {
                let score = b.iter().filter(|x| **x != -1).sum::<i32>() * pick;
                println!("{}", score);
                break 'out;
            }
            None => {}
        }
    }
}