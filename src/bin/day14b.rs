use std::{collections::HashMap, convert::TryInto, ops::AddAssign};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day14.txt");

#[derive(Clone)]
struct Counter {
    counts: HashMap<u8, u64>,
}

impl Counter {
    fn new(bytes: &[u8]) -> Self {
        let mut counts = HashMap::new();
        for &b in bytes {
            *counts.entry(b).or_default() += 1;
        }
        Self { counts }
    }
    fn score(&self) -> u64 {
        let (min, max) = self.counts.values().minmax().into_option().unwrap();
        max - min
    }
}

impl AddAssign<&Self> for Counter {
    fn add_assign(&mut self, rhs: &Self) {
        for (&k, &v) in &rhs.counts {
            *self.counts.entry(k).or_default() += v;
        }
    }
}

struct State {
    mapping: HashMap<[u8; 2], u8>,
    solutions: HashMap<([u8; 2], usize), Counter>,
}

impl State {
    fn solve_pair(&mut self, a: u8, b: u8, levels: usize) -> Counter {
        if let Some(solution) = self.solutions.get(&([a, b], levels)) {
            solution.clone()
        } else if levels == 0 {
            Counter::new(&[])
        } else {
            if let Some(&c) = self.mapping.get(&[a, b]) {
                let mut counter = Counter::new(&[c]);
                counter += &self.solve_pair(a, c, levels - 1);
                counter += &self.solve_pair(c, b, levels - 1);
                self.solutions.insert(([a, b], levels), counter.clone());
                counter
            } else {
                Counter::new(&[])
            }
        }
    }
    fn solve(input: &'static [u8], mapping: HashMap<[u8; 2], u8>, levels: usize) -> u64 {
        let mut state = State {
            mapping,
            solutions: HashMap::new(),
        };
        let mut counter = Counter::new(input);
        for pair in input.windows(2) {
            counter += &state.solve_pair(pair[0], pair[1], levels);
        }
        counter.score()
    }
}

fn main() {
    let mut lines = INPUT.lines();
    let input = lines.next().unwrap().as_bytes();
    lines.next();

    let mut mapping = HashMap::new();
    for line in lines {
        let (ab, c) = line.split_once(" -> ").unwrap();
        let ab = ab.as_bytes();
        let c = c.as_bytes();
        mapping.insert(ab.try_into().unwrap(), c[0]);
    }

    println!("{}", State::solve(input, mapping, 40));
}
