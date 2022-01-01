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

fn read_packet(bits: &mut VecDeque<bool>) -> u64 {
    let mut version = read_bits(bits, 3);
    let type_ = read_bits(bits, 3);
    match type_ {
        4 => {
            while read_bits(bits, 1) != 0 {
                read_bits(bits, 4);
            }
            read_bits(bits, 4);
        }
        _ => {
            if read_bits(bits, 1) == 0 {
                let total_len = read_bits(bits, 15);
                let finish_len = bits.len() - (total_len as usize);
                while bits.len() != finish_len {
                    version += read_packet(bits);
                }
            } else {
                let num_packets = read_bits(bits, 11);
                for _ in 0..num_packets {
                    version += read_packet(bits);
                }
            }
        }
    }
    version
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
