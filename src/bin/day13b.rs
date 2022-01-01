use std::collections::HashSet;

use itertools::Itertools;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day13.txt");

fn main() {
    let mut lines = INPUT.lines();
    let mut dots = Vec::new();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        dots.push(scan_fmt!(line, "{},{}", i32, i32).unwrap())
    }

    for line in lines {
        match line.split_once('=').unwrap() {
            ("fold along x", coord) => {
                let coord: i32 = coord.parse().unwrap();
                for dot in &mut dots {
                    if dot.0 > coord {
                        dot.0 = coord * 2 - dot.0;
                    }
                }
            }
            ("fold along y", coord) => {
                let coord: i32 = coord.parse().unwrap();
                for dot in &mut dots {
                    if dot.1 > coord {
                        dot.1 = coord * 2 - dot.1;
                    }
                }
            }
            _ => panic!("Unknown input"),
        }
    }

    let unique_dots: HashSet<_> = dots.into_iter().collect();
    let (min_y, max_y) = unique_dots
        .iter()
        .map(|dot| dot.1)
        .minmax()
        .into_option()
        .unwrap();
    let (min_x, max_x) = unique_dots
        .iter()
        .map(|dot| dot.0)
        .minmax()
        .into_option()
        .unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!(
                "{}",
                if unique_dots.contains(&(x, y)) {
                    "#"
                } else {
                    " "
                }
            );
        }
        println!();
    }
}
