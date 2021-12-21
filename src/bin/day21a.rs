#[derive(Debug, Copy, Clone)]
struct Player {
    score: u32,
    pos: u32,
}
impl Player {
    fn turn(&mut self, roll: u32) -> u32 {
        self.pos += roll;
        self.pos %= 10;
        self.score += self.pos + 1;
        self.score
    }
}

struct Dice {
    value: u32,
    sides: u32,
    cnt: u32,
}
impl Dice {
    fn new(sides: u32) -> Dice {
        Dice {
            value: 0,
            sides: sides,
            cnt: 0,
        }
    }

    fn roll(&mut self, times: u32) -> u32 {
        let mut v = 0;
        for _ in 0..times {
            v += self.value + 1;
            self.value += 1;
            self.value %= self.sides;
            self.cnt += 1;
        }
        v
    }
}

fn main() {
    let mut players = vec![Player { score: 0, pos: 4 }, Player { score: 0, pos: 8 }];
    let mut dice = Dice::new(100);

    loop {
        for player in &mut players {
            player.turn(dice.roll(3));
            if player.score >= 1000 {
                let other = players.iter().find(|p| p.score < 1000).unwrap();
                println!("{:?}", other.score * dice.cnt);
                return;
            }
        }
    }
}
