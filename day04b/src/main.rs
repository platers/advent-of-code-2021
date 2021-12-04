struct Board {
    arr: Vec<Vec<i32>>, // -1 is picked
}

impl Board {
    fn bingo(&self) -> bool {
        (0..5).any(
            |i| (0..5).all(|j| self.arr[i][j] == -1)
        ) ||
        (0..5).any(
            |i| (0..5).all(|j| self.arr[j][i] == -1)
        )
    }

    fn pick(&mut self, n: &i32) {
        self.arr.iter_mut().flatten().for_each(|x| {
            if x == n {
                *x = -1;
            }
        });
    }

    fn score(&self, last: &i32) -> i32 {
        self.arr.iter().flatten().filter(|x| **x != -1).sum::<i32>() * last
    }

    fn sim(&mut self, picks: &Vec<i32>) -> (usize, i32) { // (moves, score)
        for (i, pick) in picks.iter().enumerate() {
            self.pick(pick);
            if self.bingo() { return (i, self.score(pick)) }
        }
        panic!();
    }
}

fn build_board(s:&str) -> Board {
    Board {
        arr: s.split('\n').map(
            |l| l.split_whitespace().map(|t| t.parse().unwrap()).collect()
        ).collect()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let (first, rest) = input.split_at(input.find('\n').unwrap());

    let picks:Vec<i32> = first.split(',').map(|c| c.parse().unwrap()).collect();
    
    let res = rest
        .trim()
        .split("\n\n")
        .map(build_board) // boards
        .map(|mut board| board.sim(&picks))
        .max_by(|x, y| (x.0).cmp(&y.0))
        .unwrap();

    println!("Ans {}", res.1);
}

// tests
#[test]
fn test_bingo_cols() {
    let mut b = Board {
        arr: vec![vec![1,2,3,4,5], vec![6,7,8,9,10], vec![11,12,13,14,15], vec![16,17,18,19,20], vec![21,22,23,24,25]]
    };
    assert_eq!(b.bingo(), false);
    b.pick(&1);
    assert_eq!(b.bingo(), false);
    b.pick(&6);
    b.pick(&11);
    b.pick(&16);
    b.pick(&21);
    assert_eq!(b.bingo(), true);
}
#[test]
fn test_bingo_rows() {
    let mut b = Board {
        arr: vec![vec![1,2,3,4,5], vec![6,7,8,9,10], vec![11,12,13,14,15], vec![16,17,18,19,20], vec![21,22,23,24,25]]
    };
    assert_eq!(b.bingo(), false);
    b.pick(&1);
    b.pick(&2);
    b.pick(&3);
    b.pick(&4);
    b.pick(&5);
    assert_eq!(b.bingo(), true);
}
