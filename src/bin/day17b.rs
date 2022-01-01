use std::ops::RangeInclusive;

use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day17.txt");

fn test(mut vel: (i32, i32), xr: RangeInclusive<i32>, yr: RangeInclusive<i32>) -> bool {
    let mut pos = (0, 0);
    while vel.1 >= 0 || pos.1 >= *yr.start() {
        if xr.contains(&pos.0) && yr.contains(&pos.1) {
            return true;
        }

        pos.0 += vel.0;
        pos.1 += vel.1;
        vel.0 -= vel.0.signum();
        vel.1 -= 1;
    }
    false
}

fn main() {
    let (x0, x1, y0, y1) =
        scan_fmt!(INPUT, "target area: x={}..{}, y={}..{}", i32, i32, i32, i32).unwrap();
    let xr = x0..=x1;
    let yr = y0..=y1;

    let mut count = 0;
    for vy in y0..=106 {
        for vx in 1..=x1 {
            if test((vx, vy), xr.clone(), yr.clone()) {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
