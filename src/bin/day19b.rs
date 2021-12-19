use std::collections::HashSet;
use itertools::Itertools;


#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}


impl Point {
    fn translate(&self, v: &Point) -> Point {
        Point {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    fn permute_axis(&self, permutation: &[usize]) -> Point {
        let p = vec![self.x, self.y, self.z];
        Point {
            x: p[permutation[0]],
            y: p[permutation[1]],
            z: p[permutation[2]],
        }
    }

    fn flip(&self, flip: &[i32]) -> Point {
        Point {
            x: self.x * flip[0],
            y: self.y * flip[1],
            z: self.z * flip[2],
        }
    }

    fn dist(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Scanner {
    beacons: Vec<Point>,
    pos: Point,
    set: bool,
    new_set: bool,
}

impl Scanner {
    fn match_fixed_beacons(&self, beacons: &Vec<Point>) -> Option<Point> {
        let absolutes = self.beacons.iter()
            .map(|b| b.translate(&self.pos))
            .collect::<Vec<Point>>();
        for &b in beacons[..beacons.len() - 11].iter() {
            for a in &absolutes {
                let translate_vec = Point {
                    x: a.x - b.x,
                    y: a.y - b.y,
                    z: a.z - b.z,
                };

                let new_absolutes = beacons.iter()
                    .map(|b| b.translate(&translate_vec))
                    .collect::<Vec<Point>>();
                
                let mut intersection_size = 0;
                for &a in &new_absolutes {
                    if absolutes.contains(&a) {
                        intersection_size += 1;
                    }
                }

                // println!("Intersection size: {}", intersection_size);
                if intersection_size >= 12 {
                    return Some(translate_vec);
                }
            }
        }
        None
    }

    fn matches(&mut self, other: &Scanner) -> bool {
        for permutation in (0..3).permutations(3) {
            let beacons = self.beacons.iter()
                .map(|p| p.permute_axis(&permutation))
                .collect::<Vec<Point>>();
            let flips = [[1, 1, 1], [1, 1, -1], [1, -1, 1], [1, -1, -1], [-1, 1, 1], [-1, 1, -1], [-1, -1, 1], [-1, -1, -1]];
            for flip in &flips {
                let beacons = beacons.iter().map(|p| p.flip(flip)).collect::<Vec<Point>>();
                match other.match_fixed_beacons(&beacons) {
                    Some(p) => {
                        self.pos = p;
                        self.set = true;
                        self.beacons = beacons;
                        return true;
                    },
                    None => (),
                }
            }
        }
        false
    }

    fn try_match_to_set(&mut self, set_scanners: &HashSet<&Scanner>) -> bool {
        for set in set_scanners {
            if self.matches(set) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let mut scanners = input
        .split("\n\n")
        .map(|lines| {
            Scanner {
                beacons: lines
                    .lines()
                    .skip(1)
                    .map(|line| {
                        let c = line.split(',')
                            .map(|s| s.trim().parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();
                        Point {
                            x: c[0],
                            y: c[1],
                            z: c[2],
                        }
                    })
                    .collect(),
                pos: Point { x: 0, y: 0, z: 0 },
                set: false,
                new_set: false,
            }
        })
        .collect::<Vec<Scanner>>();
    println!("{:?}", scanners.len());
    scanners[0].set = true;
    scanners[0].new_set = true;
   
    while scanners.iter().filter(|s| !s.set).count() > 0 {
        println!("{:?}", scanners.iter().filter(|s| !s.set).count());
        let c = scanners.clone();
        let set_scanners = c.iter()
            .filter(|s| s.new_set)
            .collect::<HashSet<&Scanner>>();
        for s in &mut scanners {
            s.new_set = false;
        }
        for s in &mut scanners {
            if !s.set {
                if s.try_match_to_set(&set_scanners) {
                    s.new_set = true;
                }
            }
        }

    }

    let mut largest_dist = 0;
    for perm in scanners.iter().permutations(2) {
        let dist = perm[0].pos.dist(&perm[1].pos);
        if dist > largest_dist {
            largest_dist = dist;
        }
    }

    println!("{}", largest_dist);
}
