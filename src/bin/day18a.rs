use nom::{
    *,
    IResult,
    character::complete::*,
    sequence::*,
};

enum Node {
    Val(u32),
    Pair(Box<Node>, Box<Node>),
}

impl Node {
    fn to_string(&self) -> String {
        match self {
            Node::Val(v) => v.to_string(),
            Node::Pair(l, r) => format!("[{},{}]", l.to_string(), r.to_string()),
        }
    }

    fn from_string(s: &str) -> Node {
        match pair(s) {
            Ok((input, pair)) => pair,
            Err(e) => panic!(),
        }
    }

    fn explode(&mut self) -> bool {
        
    }

    fn find_explode_idx(&self, idx:&mut u32, depth: u32) -> Option<u32> {
        match self {
            Node::Val(_) => None,
            Node::Pair(l, r) => {
                if depth == 4 {
                    Some(idx)
                }

                if let mut l_idx = l.find_explode_idx(&mut idx, depth + 1) {
                    return Some(l_idx);
                }

                idx += 1;

                if let mut r_idx = r.find_explode_idx(&mut idx, depth + 1) {
                    return Some(r_idx);
                }

            }
        }
    }
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

fn reduce(root: &mut Node) {
    loop {
        if explode(root) {
            continue;
        }
        if split(root) {
            continue;
        }
        break;
    }
}

fn pair(input: &str) -> IResult<&str, Node> {
    if input.chars().next().unwrap() == '[' {
        let (input, _) = char('[')(input)?;
        let (input, children) = separated_pair(pair, char(','), pair)(input)?;
        let (input, _) = char(']')(input)?;
        Ok((input, Node::Pair(Box::new(children.0), Box::new(children.1))))
    } else {
        let (input, val) = digit1(input)?;
        Ok((input, Node::Val(val.parse::<u32>().unwrap())))
    }
}
