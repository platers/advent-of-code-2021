use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    hallway: Vec<i32>,
    rooms: Vec<(i32, i32)>, // (top, bottom)
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
        let room = self.rooms[letter];
        if (room.0 >= 0 && room.0 != letter as i32) || (room.1 >= 0 && room.1 != letter as i32) {
            return;
        }

        let room_x = letter * 2 + 2;
        // make sure hallway is clear from room_x to hallway_pos
        if !self.check_hallway_clear(room_x, hallway_pos) {
            return;
        }

        match (room.0, room.1) {
            (-1, -1) => {
                self.hallway[hallway_pos] = -1;
                self.rooms[letter].1 = letter as i32;
                self.dist += MOVE_COSTS[letter] * (self.hallway_dist(hallway_pos, room_x) + 2);
            },
            (-1, b) if b == letter as i32 => {
                self.hallway[hallway_pos] = -1;
                self.rooms[letter].0 = letter as i32;
                self.dist += MOVE_COSTS[letter] * (self.hallway_dist(hallway_pos, room_x) + 1);
            },
            _ => {}
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
        let room = self.rooms[room_id];
        let room_x = room_id * 2 + 2;
        if !self.check_hallway_clear(hallway_pos, room_x) {
            return None;
        }
        if (room.0 == -1 || room.0 == room_id as i32) && (room.1 == -1 || room.1 == room_id as i32) {
            return None;
        }
        match (room.0, room.1) {
            (-1, a) if a != room_id as i32 => {
                let mut new_state = self.clone();
                new_state.hallway[hallway_pos] = a;
                new_state.rooms[room_id].1 = -1;
                let dist = self.hallway_dist(hallway_pos, room_x) + 2;
                new_state.dist += MOVE_COSTS[a as usize] * dist;
                Some(new_state)
            },
            (a, b) if a != room_id as i32 || b != room_id as i32 => {
                let mut new_state = self.clone();
                new_state.hallway[hallway_pos] = a;
                new_state.rooms[room_id].0 = -1;
                let dist = self.hallway_dist(hallway_pos, room_x) + 1;
                new_state.dist += MOVE_COSTS[a as usize] * dist;
                Some(new_state)
            },
            _ => None,
        }
    }
}

const MOVE_COSTS: [u32; 4] = [1, 10, 100, 1000];

fn main() {
    let input = include_str!("../../input.txt");
    let mut initial_rooms = vec![(0, 0); 4];
    let top_line = input.lines().nth(2).unwrap();
    let bottom_line = input.lines().nth(3).unwrap();
    for i in 0..4 {
        initial_rooms[i].0 = top_line
            .chars()
            .nth(i * 2 + 3)
            .unwrap()
            .to_digit(16)
            .unwrap() as i32
            - 10;
        initial_rooms[i].1 = bottom_line
            .chars()
            .nth(i * 2 + 3)
            .unwrap()
            .to_digit(16)
            .unwrap() as i32
            - 10;
    }
    let initial_state = State {
        hallway: vec![-1; input.lines().next().unwrap().len() - 2],
        rooms: initial_rooms,
        dist: 0,
    };

    println!("{:?}", initial_state);

    let goal_rooms = vec![(0, 0), (1, 1), (2, 2), (3, 3)];


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
