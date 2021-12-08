use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt");
    let lines = input.lines().map(|l| {
        let (a, b) = l.split_once(" | ").unwrap();
        let mut a = a.split_whitespace().map(|s| {
            let mut v = s.chars().collect::<Vec<_>>();
            v.sort(); 
            v
        }).collect::<Vec<_>>();
        let b = b.split_whitespace().map(|s| {
            let mut v = s.chars().collect::<Vec<_>>();
            v.sort(); 
            v
        }).collect::<Vec<_>>();
        a.sort();
        (a, b)
    }).collect::<Vec<_>>();
    let alph = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let base = vec![
        "abcefg",
        "cf",
        "acdeg",
        "acdfg",
        "bcdf",
        "abdfg",
        "abdefg",
        "acf",
        "abcdefg",
        "abcdfg"
    ].iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut base_sorted = base.clone();
    base_sorted.sort();


    let ans = lines.iter().map(|(a, b)| {
        let mut perm = vec![];
        for p in alph.iter().permutations(alph.len()) {
            // println!("{:?}", apply_permutation(&p, a, &alph));
            if apply_permutation(&p, a, &alph, true) == base_sorted {
                println!("{:?}", p);
                perm = p; 
                break;
            }
        };

        let n = apply_permutation(&perm, b, &alph, false).iter()
            .map(|w| {
                base.iter().position(|b| b == w).unwrap()
            })
            .map(|i| char::from_digit(i as u32, 10).unwrap())
            .collect::<String>()
            .parse::<usize>().unwrap();
        println!("{}", n);
        n
    }).sum::<usize>();

    println!("{}", ans);
}

fn apply_permutation(perm: &Vec<&char>, base: &Vec<Vec<char>>, alph: &Vec<char>, sort: bool) -> Vec<Vec<char>> {
    // println!("{:?}", perm);
    let mut q = base.iter().map(|b| {
        let mut v = b.iter().map(|c| {
            *perm[alph.iter().position(|a| a == c).unwrap()]
        }).collect::<Vec<_>>();
        v.sort();
        v
    }).collect::<Vec<_>>();
    if sort {
        q.sort();
    }
    q
}

// tests
#[test]
fn test_apply_permutation() {
    let alph = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let perm = alph
        .iter()
        .permutations(7)
        .nth(0) 
        .unwrap();
    println!("{:?}", perm);
    let base = vec![
        "abcef",
        "cf",
        "acdeg",
        "acdfg",
        "bcdf",
        "abdfg",
        "abdefg",
        "abcdefg",
        "abcdfg"
    ].iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut base_sorted = base.clone();
    base_sorted.sort();

    assert_eq!(apply_permutation(&perm, &base_sorted, &alph, true), base_sorted);
}
