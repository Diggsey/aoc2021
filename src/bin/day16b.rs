use std::collections::VecDeque;

const INPUT: &str = include_str!("../../inputs/day16.txt");

fn read_bits(bits: &mut VecDeque<bool>, count: usize) -> u64 {
    let mut res = 0;
    for _ in 0..count {
        res <<= 1;
        if bits.pop_front().unwrap() {
            res |= 1;
        }
    }
    res
}

fn read_subpackets(bits: &mut VecDeque<bool>) -> impl Iterator<Item = u64> {
    let mut res = Vec::new();
    if read_bits(bits, 1) == 0 {
        let total_len = read_bits(bits, 15);
        let finish_len = bits.len() - (total_len as usize);
        while bits.len() != finish_len {
            res.push(read_packet(bits));
        }
    } else {
        let num_packets = read_bits(bits, 11);
        for _ in 0..num_packets {
            res.push(read_packet(bits));
        }
    }
    res.into_iter()
}

fn binary_op(mut it: impl Iterator<Item = u64>) -> (u64, u64) {
    (it.next().unwrap(), it.next().unwrap())
}

fn read_packet(bits: &mut VecDeque<bool>) -> u64 {
    let _version = read_bits(bits, 3);
    let type_ = read_bits(bits, 3);
    match type_ {
        0 => read_subpackets(bits).sum(),
        1 => read_subpackets(bits).product(),
        2 => read_subpackets(bits).min().unwrap(),
        3 => read_subpackets(bits).max().unwrap(),
        4 => {
            let mut value = 0;
            while read_bits(bits, 1) != 0 {
                value <<= 4;
                value |= read_bits(bits, 4);
            }
            value <<= 4;
            value |= read_bits(bits, 4);
            value
        }
        5 => {
            let (a, b) = binary_op(read_subpackets(bits));
            if a > b {
                1
            } else {
                0
            }
        }
        6 => {
            let (a, b) = binary_op(read_subpackets(bits));
            if a < b {
                1
            } else {
                0
            }
        }
        7 => {
            let (a, b) = binary_op(read_subpackets(bits));
            if a == b {
                1
            } else {
                0
            }
        }
        _ => panic!("Unknown type: {}", type_),
    }
}

fn main() {
    let mut bits = VecDeque::new();
    for c in INPUT.chars() {
        if let Some(digit) = c.to_digit(16) {
            for i in (0..4).rev() {
                bits.push_back(((digit >> i) & 1) != 0);
            }
        }
    }
    println!("{}", read_packet(&mut bits));
}
