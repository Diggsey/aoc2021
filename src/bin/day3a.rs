const INPUT: &str = include_str!("../../inputs/day3.txt");

fn main() {
    let mut counts = vec![0; 12];
    let mut n = 0;
    for line in INPUT.lines() {
        n += 1;
        for (c, count) in line.chars().zip(counts.iter_mut()) {
            if c == '1' {
                *count += 1;
            }
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for (i, &count) in counts.iter().rev().enumerate() {
        if count > n / 2 {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }
    println!("{}", gamma * epsilon);
}
