use nom::{
    IResult,
    bits::*,
    bits::complete::*,
};
use hex;
struct Packet {
    version: u8,
    type_id: u8,

    val: u32,
    sub_packets: Vec<Packet>,
}

fn main() {
    let input = include_str!("../../input.txt");
    let root = packet(&hex::decode(input).unwrap()).unwrap().1;

    let version_sum = dfs(&root);
    println!("{}", version_sum);
}

fn dfs(packet: &Packet) -> u32 {
    let mut sum = 0u32;
    sum += packet.version as u32;
    for sub_packet in &packet.sub_packets {
        sum += dfs(sub_packet);
    }
    sum
}

fn literal_value(mut input: (&[u8], usize)) -> IResult<(&[u8], usize), u32> {
    let mut val = 0u32;
    loop {
        let (rest, chunk): ((&[u8], usize), u8) = take(5usize)(input)?;
        input = rest;
        val = val << 4 | (chunk & ((1 << 4) - 1)) as u32;
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
    let (input, version): ((&[u8], usize), u8) = take(3usize)(input)?;
    let (input, type_id): ((&[u8], usize), u8) = take(3usize)(input)?;
    match type_id {
        4 => {
            let (input, val) = literal_value(input)?;
            Ok((input, Packet {
                version,
                type_id,
                val,
                sub_packets: vec![],
            }))
        },
        _ => {
            let (input, sub_packets) = operator(input)?;
            Ok((input, Packet {
                version,
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

#[test]
fn test_literal() {
    let hex_array = hex::decode("D2FE28").unwrap();
    assert_eq!(literal_value((&hex_array, 6)).unwrap().1, 2021);
}

#[test]
fn test_operator_len() {
    let hex_array = hex::decode("38006F45291200").unwrap();
    let (rest, packets) = operator((&hex_array, 6)).unwrap();
    assert_eq!(packets.len(), 2);
}

#[test]
fn test_operator_num() {
    let hex_array = hex::decode("EE00D40C823060").unwrap();
    let (rest, packets) = operator((&hex_array, 6)).unwrap();
    assert_eq!(packets.len(), 3);
    assert_eq!(packets[0].val, 1);
    assert_eq!(packets[1].val, 2);
    assert_eq!(packets[2].val, 3);
}

#[test]
fn test_packet() {
    let hex_array = hex::decode("D2FE28").unwrap();
    let (rest, packet) = packet(&hex_array).unwrap();
    assert_eq!(packet.val, 2021);
}