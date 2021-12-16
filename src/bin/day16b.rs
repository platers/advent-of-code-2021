use nom::{
    IResult,
    bits::*,
    bits::complete::*,
};
use hex;
struct Packet {
    type_id: u8,

    val: u128,
    sub_packets: Vec<Packet>,
}

fn main() {
    let input = include_str!("../../input.txt");
    let root = packet(&hex::decode(input).unwrap()).unwrap().1;

    let ans = dfs(&root);
    println!("{}", ans);
}

fn dfs(p: &Packet) -> i128 {
    let children = p.sub_packets.iter().map(|p| dfs(p)).collect::<Vec<_>>();
    match p.type_id {
        0 => { children.iter().sum() },
        1 => { children.iter().product() },
        2 => { *children.iter().min().unwrap() },
        3 => { *children.iter().max().unwrap() },
        4 => { p.val as i128 },
        5 => { (children[0] > children[1]) as i128 },
        6 => { (children[0] < children[1]) as i128 },
        7 => { (children[0] == children[1]) as i128 },
        _ => { panic!("unexpected type_id") }
    }
}

fn literal_value(mut input: (&[u8], usize)) -> IResult<(&[u8], usize), u128> {
    let mut val = 0u128;
    loop {
        let (rest, chunk): ((&[u8], usize), u8) = take(5usize)(input)?;
        input = rest;
        val = val << 4 | (chunk & ((1 << 4) - 1)) as u128;
        if chunk >> 4 == 0 {
            return Ok((rest, val))
        }
    }
}

fn bit_length(bits: (&[u8], usize)) -> usize {
    bits.0.len() * 8 - bits.1
}

fn operator(input: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<Packet>> {
    let (rest, length_type): ((&[u8], usize), u8) = take(1usize)(input)?;
    let mut sub_packets = Vec::new();
    if length_type == 0 {
        let (mut rest, length): ((&[u8], usize), usize) = take(15usize)(rest)?;
        // take packets until we consume the length
        let start_len = bit_length(rest);
        while bit_length(rest) > start_len - length {
            let (t, packet): ((&[u8], usize), Packet) = packet_bits(rest)?;
            rest = t;
            sub_packets.push(packet);
        }
        Ok((rest, sub_packets))
    } else {
        let (mut rest, num_packets): ((&[u8], usize), usize) = take(11usize)(rest)?;
        for _ in 0..num_packets {
            let (t, packet): ((&[u8], usize), Packet) = packet_bits(rest)?;
            rest = t;
            sub_packets.push(packet);
        }
        Ok((rest, sub_packets))
    }
}

fn packet_bits(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let (input, _): ((&[u8], usize), u8) = take(3usize)(input)?;
    let (input, type_id): ((&[u8], usize), u8) = take(3usize)(input)?;
    match type_id {
        4 => {
            let (input, val) = literal_value(input)?;
            Ok((input, Packet {
                type_id,
                val: val as u128,
                sub_packets: vec![],
            }))
        },
        _ => {
            let (input, sub_packets) = operator(input)?;
            Ok((input, Packet {
                type_id,
                val: 0,
                sub_packets,
            }))
        }
    }
}

fn packet(input: &[u8]) -> IResult<&[u8], Packet> {
    bits(packet_bits)(input)
}