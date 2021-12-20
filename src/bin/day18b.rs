use itertools::Itertools;
type Num = Vec<(u32, u32)>;

fn main() {
    let input = include_str!("../../input.txt");
    let nums = input.lines()
        .map(|line| {
            line.chars().fold((Num::new(), 0), |(mut nums, mut d), c| {
                match c {
                    '[' => d += 1,
                    ']' => d -= 1,
                    ',' => {},
                    _ => nums.push((d as u32, c.to_digit(10).unwrap() as u32)),
                }
                (nums, d)
            }).0
        })
        .collect::<Vec<Num>>();
    
    let result = nums.iter().permutations(2)
        .map(|p| {
            magnitude(&add(&p[0], &p[1]))
        })
        .max().unwrap();
    println!("{}", result);
}

fn add(a: &Num, b: &Num) -> Num {
    let mut num = a.clone();
    num.extend(b);
    num = num.iter().map(|(d, n)| (d + 1, *n)).collect();
    reduce(num)
}

fn reduce(mut num: Num) -> Num {
    loop {
        match explode(&num) {
            Some(n) => {
                num = n;
                continue;
            },
            None => {},
        }

        match split(&num) {
            Some(n) => {
                num = n;
                continue;
            },
            None => {},
        }
        return num;
    }
}

fn explode(num: &Num) -> Option<Num> {
    match num.iter().enumerate().find(|(_, (d, _))| *d == 5) {
        Some((i, (_, n))) => {
            let mut num = num.clone();
            if i > 0 {
                num[i - 1].1 += n;
            }
            num.remove(i);
            let (d, n) = num[i];
            assert_eq!(d, 5);
            if i + 1 < num.len() {
                num[i + 1].1 += n;
            }
            num[i] = (d - 1, 0);
            Some(num)
        },
        _ => None,
    }
}

fn split(num: &Num) -> Option<Num> {
    match num.iter().enumerate().find(|(_, (_, n))| *n >= 10) {
        Some((i, (d, n))) => {
            let mut num = num.clone();
            num.splice(i..i + 1, vec![(d + 1, n / 2), (d + 1, (n + 1) / 2)]);
            Some(num)
        },
        _ => None,
    }
}

fn magnitude(num: &Num) -> u32 {
    if num.len() == 1 {
        return num[0].1;
    }
    
    for i in 0..num.len() - 1 {
        if num[i].0 == num[i + 1].0 {
            let mut num = num.clone();
            num[i].1 = 3 * num[i].1 + 2 * num[i + 1].1;
            num[i].0 -= 1;
            num.remove(i + 1);

            return magnitude(&num);
        }
    }

    panic!("No magnitude found");
}