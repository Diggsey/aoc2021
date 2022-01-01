use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day8.txt");

fn str_to_indices(s: &str) -> Vec<u8> {
    s.bytes().map(|c| (c - b'a') as u8).collect()
}

fn decode(input: &[u8], perm: &[u8]) -> Option<u8> {
    let mut mapped_input: Vec<_> = input.iter().copied().map(|b| perm[b as usize]).collect();
    mapped_input.sort();
    Some(match mapped_input.as_slice() {
        &[0, 1, 2, 4, 5, 6] => 0,
        &[2, 5] => 1,
        &[0, 2, 3, 4, 6] => 2,
        &[0, 2, 3, 5, 6] => 3,
        &[1, 2, 3, 5] => 4,
        &[0, 1, 3, 5, 6] => 5,
        &[0, 1, 3, 4, 5, 6] => 6,
        &[0, 2, 5] => 7,
        &[0, 1, 2, 3, 4, 5, 6] => 8,
        &[0, 1, 2, 3, 5, 6] => 9,
        _ => return None,
    })
}

fn main() {
    let mut total = 0;
    for input in INPUT.lines() {
        let mut section_iter = input.splitn(2, " | ");
        let inputs: Vec<_> = section_iter
            .next()
            .unwrap()
            .split(' ')
            .map(str_to_indices)
            .collect();
        let outputs: Vec<_> = section_iter
            .next()
            .unwrap()
            .split(' ')
            .map(str_to_indices)
            .collect();

        'next_perm: for perm in (0..7).permutations(7) {
            for input in &inputs {
                if decode(input, &perm).is_none() {
                    continue 'next_perm;
                }
            }
            let mut output_value = 0;
            for output in &outputs {
                if let Some(res) = decode(output, &perm) {
                    output_value = output_value * 10 + (res as u64);
                } else {
                    continue 'next_perm;
                }
            }
            total += output_value;
            break;
        }
    }
    println!("{}", total);
}
