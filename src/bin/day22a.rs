use std::collections::HashMap;

use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day22.txt");

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct AABB {
    a: [i32; 3],
    b: [i32; 3],
}

impl AABB {
    fn is_empty(&self) -> bool {
        self.b[0] <= self.a[0] || self.b[1] <= self.a[1] || self.b[2] <= self.a[2]
    }
    fn intersection(self, other: AABB) -> Self {
        Self {
            a: [
                self.a[0].max(other.a[0]),
                self.a[1].max(other.a[1]),
                self.a[2].max(other.a[2]),
            ],
            b: [
                self.b[0].min(other.b[0]),
                self.b[1].min(other.b[1]),
                self.b[2].min(other.b[2]),
            ],
        }
    }
    fn volume(&self) -> i64 {
        if self.is_empty() {
            0
        } else {
            let w = (self.b[0] - self.a[0]) as i64;
            let h = (self.b[1] - self.a[1]) as i64;
            let d = (self.b[2] - self.a[2]) as i64;
            w * h * d
        }
    }
}

fn main() {
    let mut volumes = HashMap::<AABB, i32>::new();
    for line in INPUT.lines() {
        let (state, xa, xb, ya, yb, za, zb) = scan_fmt!(
            line,
            "{} x={}..{},y={}..{},z={}..{}",
            String,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32
        )
        .unwrap();
        let state = match state.as_str() {
            "on" => true,
            "off" => false,
            _ => panic!("Unknown state: {}", state),
        };

        let new_volume = AABB {
            a: [xa, ya, za],
            b: [xb + 1, yb + 1, zb + 1],
        };
        let mut new_volumes = HashMap::<AABB, i32>::new();
        for (&k, &v) in volumes.iter() {
            if v != 0 {
                let intersection = k.intersection(new_volume);
                if !intersection.is_empty() {
                    *new_volumes.entry(intersection).or_default() -= v;
                }
            }
        }
        for (k, v) in new_volumes {
            *volumes.entry(k).or_default() += v;
        }
        if state {
            *volumes.entry(new_volume).or_default() += 1;
        }
    }

    let region = AABB {
        a: [-50, -50, -50],
        b: [51, 51, 51],
    };
    let count: i64 = volumes
        .into_iter()
        .map(|(k, v)| k.intersection(region).volume() * (v as i64))
        .sum();
    println!("{}", count);
}
