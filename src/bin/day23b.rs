use std::collections::BinaryHeap;
use std::collections::HashSet;

const N:usize = 4;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    hallway: Vec<i32>,
    rooms: Vec<Vec<i32>>,
    dist: u32,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl State {
    fn hallway_dist(&self, a: usize, b: usize) -> u32 {
        ((a as i32) - (b as i32)).abs() as u32
    }
    fn check_hallway_clear(&self, a: usize, b: usize) -> bool {
        // [a, b)
        if a < b {
            for i in a..b {
                if self.hallway[i] != -1 {
                    return false;
                }
            }
        } else {
            for i in b + 1..=a {
                if self.hallway[i] != -1 {
                    return false;
                }
            }
        }
        true
    }

    fn path_to_room(&mut self, hallway_pos: usize) {
        let letter = self.hallway[hallway_pos] as usize;
        let room = self.rooms[letter].clone();

        let room_x = letter * 2 + 2;
        // make sure hallway is clear from room_x to hallway_pos
        if !self.check_hallway_clear(room_x, hallway_pos) {
            return;
        }

        for i in (0..N).rev() {
            match room[i] {
                -1 => {
                    self.hallway[hallway_pos] = -1;
                    self.rooms[letter][i] = letter as i32;
                    self.dist += MOVE_COSTS[letter] * (self.hallway_dist(hallway_pos, room_x) + i as u32 + 1);
                    return;
                },
                x if x == letter as i32 => {},
                _ => {
                    return;
                }
            }
        }
    }

    fn move_hallway_guys(&mut self) {
        for i in 0..self.hallway.len() {
            if self.hallway[i] != -1 {
                self.path_to_room(i);
                if self.hallway[i] == -1 {
                    self.move_hallway_guys();
                    return;
                }
            }
        }
    }

    fn move_to_hallway(&self, room_id: usize, hallway_pos: usize) -> Option<State> {
        let room = self.rooms[room_id].clone();
        let room_x = room_id * 2 + 2;
        if !self.check_hallway_clear(hallway_pos, room_x) {
            return None;
        }
        if room.iter().all(|&x| x == -1 || x == room_id as i32) {
            return None;
        }

        for i in 0..N {
            if room[i] != -1 {
                let mut new_state = self.clone();
                new_state.hallway[hallway_pos] = room[i];
                new_state.rooms[room_id][i] = -1;
                let dist = self.hallway_dist(hallway_pos, room_x) + i as u32 + 1;
                new_state.dist += MOVE_COSTS[room[i] as usize] * dist;
                return Some(new_state)
            }
        }
        panic!("no room to move to");
    }
}

const MOVE_COSTS: [u32; 4] = [1, 10, 100, 1000];

fn main() {
    let input = include_str!("../../input.txt");
    let mut initial_rooms = vec![vec![]; 4];
    for l in 0..N {
        let line = input.lines().nth(l + 2).unwrap();
        for i in 0..4 {
            initial_rooms[i].push(line
                .chars()
                .nth(i * 2 + 3)
                .unwrap()
                .to_digit(16)
                .unwrap() as i32
                - 10);
        }
    }
    let initial_state = State {
        hallway: vec![-1; input.lines().next().unwrap().len() - 2],
        rooms: initial_rooms,
        dist: 0,
    };

    println!("{:?}", initial_state);

    let goal_rooms = vec![vec![0; N], vec![1; N], vec![2; N], vec![3; N]];


    let mut open_set = BinaryHeap::new();
    let mut seen = HashSet::new();
    open_set.push(initial_state);
    while let Some(state) = open_set.pop() {
        if seen.contains(&state) {
            continue;
        }
        seen.insert(state.clone());
        if state.rooms == goal_rooms {
            println!("{:?}", state);
            return;
        }

        for i in 0..4 {
            // move guy in room to hallway
            for h in 0..state.hallway.len() {
                if h == 2 || h == 4 || h == 6 || h == 8 {
                    continue;
                }
                match state.move_to_hallway(i, h) {
                    Some(mut new_state) => {
                        new_state.move_hallway_guys();
                        open_set.push(new_state);
                    },
                    None => (),
                }
            }
        }
    }
}
