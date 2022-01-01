use std::collections::HashSet;

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
        break;
    }

    let unique_dots: HashSet<_> = dots.into_iter().collect();

    println!("{}", unique_dots.len());
}
