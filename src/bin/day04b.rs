fn main() {
    let input = include_str!("../../input.txt");
    let (first, rest) = input.split_once("\n\n").unwrap();
    let picks = first.split(',').map(|s| s.parse::<isize>().unwrap()).collect::<Vec<_>>();
    let mut boards = rest.split("\n\n")
        .map(|s| s.split_whitespace()
            .map(|s| s.parse::<isize>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    let mut last_score = 0;
    for pick in picks {
        for board in boards.iter_mut() {
            for i in 0..board.len() {
                if board[i] == pick {
                    board[i] = -1;
                    if bingo(board) {
                        last_score = pick * board.iter().filter(|&&x| x != -1).sum::<isize>();
                    }
                }
            }
        }
        
        boards.retain(|board| !bingo(board));
    }

    println!("{}", last_score);
}

fn bingo(board: &Vec<isize>) -> bool {
    (0..5).any(|i| {
        (0..5).all(|j| board[i*5+j] == -1)
    }) || (0..5).any(|j| {
        (0..5).all(|i| board[i*5+j] == -1)
    })
}