fn main() {
    let input = include_str!("../../input.txt");
    let arr = input.split(',').map(|x| x.parse::<u128>().unwrap()).collect::<Vec<u128>>();
    let mut state = (0..9).map(|i| arr.iter()
        .filter(|&&x| x == i).count() as u128).collect::<Vec<u128>>();
    
    for _ in 0..256 {
        state.rotate_left(1);
        state[6] += state[8];
    }

    println!("{}", state.iter().sum::<u128>());
}
