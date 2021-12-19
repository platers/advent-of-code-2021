use nom::{
    *,
    IResult,
    character::complete::*,
    sequence::*,
};

struct Node {
    depth: u32,
    l: Option<u32>,
    r: Option<u32>,
}

fn parse_nodes(s: &str, depth: u32) -> IResult<&str, Vec<Node>> {
    let mut nodes = vec![];
    let mut p = Node {
        depth,
        l: None,
        r: None,
    };
    let mut s = s[1..];
    match s.chars().nth(1).unwrap() {
        '[' => {
            nodes.extend(parse_nodes(s, depth + 1).unwrap().1);
        },
        _ => {
            p.l = Some(s.parse::<u32>().unwrap());
        }
    }
    s = s[s.find(',').unwrap() + 1..];
    nodes.push(p);
    match s.chars().nth(1).unwrap() {
        '[' => {
            nodes.extend(parse_nodes(s, depth + 1).unwrap().1);
        },
        _ => {
            p.r = Some(s.parse::<u32>().unwrap());
        }
    }
    s = s[s.find(']').unwrap() + 1..];
    Ok((s, nodes))
}

fn main() {
    let input = include_str!("../../input.txt");
    let mut root = Node::from_string(input.lines().next().unwrap());

    for line in input.lines().skip(1) {
        let pair = parse_pair(line);
        root = add_pairs(root, pair);
        reduce(&mut root);
    }

}

