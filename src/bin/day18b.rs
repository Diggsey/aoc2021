use std::{fmt::Display, num::ParseIntError, ops::Add, str::FromStr};

const INPUT: &str = include_str!("../../inputs/day18.txt");

#[derive(Debug, Clone)]
enum SFNum {
    Regular(u32),
    Pair(Box<[SFNum; 2]>),
}

impl SFNum {
    fn add_to_side(&mut self, side: bool, value: u32) {
        match self {
            SFNum::Regular(v) => *v += value,
            SFNum::Pair(pair) => {
                if side {
                    pair[1].add_to_side(side, value);
                } else {
                    pair[0].add_to_side(side, value);
                }
            }
        }
    }
    fn try_explode(&mut self, depth: usize) -> Option<[Option<u32>; 2]> {
        match self {
            SFNum::Regular(_) => None,
            SFNum::Pair(pair) => {
                if depth >= 4 {
                    if let [SFNum::Regular(a), SFNum::Regular(b)] = **pair {
                        *self = SFNum::Regular(0);
                        Some([Some(a), Some(b)])
                    } else {
                        panic!("Invalid SFNum")
                    }
                } else if let Some(mut explosion) = pair[0].try_explode(depth + 1) {
                    if let Some(rhs) = explosion[1].take() {
                        pair[1].add_to_side(false, rhs);
                    }
                    Some(explosion)
                } else if let Some(mut explosion) = pair[1].try_explode(depth + 1) {
                    if let Some(lhs) = explosion[0].take() {
                        pair[0].add_to_side(true, lhs);
                    }
                    Some(explosion)
                } else {
                    None
                }
            }
        }
    }
    fn try_split(&mut self) -> bool {
        match self {
            &mut SFNum::Regular(v) => {
                if v >= 10 {
                    *self = SFNum::Pair(Box::new([
                        SFNum::Regular(v / 2),
                        SFNum::Regular((v + 1) / 2),
                    ]));
                    true
                } else {
                    false
                }
            }
            SFNum::Pair(pair) => pair[0].try_split() || pair[1].try_split(),
        }
    }
    fn reduce_once(&mut self) -> bool {
        self.try_explode(0).is_some() || self.try_split()
    }
    fn reduce(&mut self) {
        while self.reduce_once() {}
    }
    fn magnitude(&self) -> u32 {
        match self {
            &SFNum::Regular(v) => v,
            SFNum::Pair(pair) => pair[0].magnitude() * 3 + pair[1].magnitude() * 2,
        }
    }
}

impl Add for SFNum {
    type Output = SFNum;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = SFNum::Pair(Box::new([self, rhs]));
        res.reduce();
        res
    }
}

impl FromStr for SFNum {
    type Err = String;

    fn from_str(mut s: &str) -> Result<Self, String> {
        fn skip_ws(s: &mut &str) {
            *s = s.trim_start();
        }
        fn expect(s: &mut &str, v: &str) -> Result<(), String> {
            skip_ws(s);
            if let Some(suffix) = s.strip_prefix(v) {
                *s = suffix;
                Ok(())
            } else {
                Err(format!("Expected: {:?}, found: {:?}", v, s))
            }
        }
        fn parse_inner(s: &mut &str) -> Result<SFNum, String> {
            skip_ws(s);
            Ok(if let Some(suffix) = s.strip_prefix("[") {
                *s = suffix;
                let lhs = parse_inner(s)?;
                expect(s, ",")?;
                let rhs = parse_inner(s)?;
                expect(s, "]")?;
                SFNum::Pair(Box::new([lhs, rhs]))
            } else {
                let offset = s
                    .char_indices()
                    .find(|&(_, c)| (c < '0' || c > '9'))
                    .ok_or_else(|| format!("Expected number, found: {:?}", s))?
                    .0;
                let v = s[0..offset]
                    .parse()
                    .map_err(|e: ParseIntError| e.to_string())?;
                *s = &s[offset..];
                SFNum::Regular(v)
            })
        }
        let res = parse_inner(&mut s)?;
        if s.trim().is_empty() {
            Ok(res)
        } else {
            Err(format!("Unexpected: {:?}", s))
        }
    }
}

impl Display for SFNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SFNum::Regular(v) => v.fmt(f),
            SFNum::Pair(pair) => {
                f.write_str("[")?;
                pair[0].fmt(f)?;
                f.write_str(",")?;
                pair[1].fmt(f)?;
                f.write_str("]")
            }
        }
    }
}

fn main() {
    let numbers: Vec<_> = INPUT
        .lines()
        .map(|line| line.parse::<SFNum>().unwrap())
        .collect();

    let res = numbers
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            numbers
                .iter()
                .enumerate()
                .filter(move |(j, _)| i != *j)
                .map(move |(_, b)| (a.clone() + b.clone()).magnitude())
        })
        .max()
        .unwrap();

    println!("{}", res);
}
