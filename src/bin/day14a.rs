use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day14.txt");

fn expand(input: &[u8], mapping: &HashMap<&[u8], u8>) -> Vec<u8> {
    let mut res = Vec::new();
    for pair in input.windows(2) {
        res.push(pair[0]);
        if let Some(&v) = mapping.get(&pair) {
            res.push(v);
        }
    }
    res.push(input[input.len() - 1]);
    res
}

fn main() {
    let mut lines = INPUT.lines();
    let mut input = lines.next().unwrap().as_bytes().to_owned();
    lines.next();

    let mut mapping = HashMap::new();
    for line in lines {
        let (ab, c) = line.split_once(" -> ").unwrap();
        let ab = ab.as_bytes();
        let c = c.as_bytes();
        mapping.insert(ab, c[0]);
    }

    for _ in 0..10 {
        input = expand(&input, &mapping)
    }

    let mut counts = HashMap::<_, usize>::new();
    for b in input {
        *counts.entry(b).or_default() += 1;
    }

    let (min, max) = counts.values().minmax().into_option().unwrap();

    println!("{}", max - min)
}
