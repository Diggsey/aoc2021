use std::collections::HashMap;

use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day5.txt");

fn main() {
    let mut counts = HashMap::<_, i32>::new();
    for line in INPUT.lines() {
        let (x0, y0, x1, y1) = scan_fmt!(line, "{},{} -> {},{}", i32, i32, i32, i32).unwrap();
        let dx = x1 - x0;
        let dy = y1 - y0;
        let d = dx.abs().max(dy.abs());
        let ix = dx / d;
        let iy = dy / d;
        for i in 0..=d {
            let x = x0 + ix * i;
            let y = y0 + iy * i;
            *counts.entry((x, y)).or_default() += 1;
        }
    }
    println!("{}", counts.values().filter(|&&v| v > 1).count());
}
