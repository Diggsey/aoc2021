use std::collections::HashMap;

use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day5.txt");

fn main() {
    let mut counts = HashMap::<_, i32>::new();
    for line in INPUT.lines() {
        let (x0, y0, x1, y1) = scan_fmt!(line, "{},{} -> {},{}", i32, i32, i32, i32).unwrap();
        if x0 == x1 || y0 == y1 {
            for y in y0.min(y1)..=y0.max(y1) {
                for x in x0.min(x1)..=x0.max(x1) {
                    *counts.entry((x, y)).or_default() += 1;
                }
            }
        }
    }
    println!("{}", counts.values().filter(|&&v| v > 1).count());
}
