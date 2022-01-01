const INPUT: &str = include_str!("../../inputs/day8.txt");

fn main() {
    let mut total = 0;
    for input in INPUT.lines() {
        let output = input.splitn(2, " | ").nth(1).unwrap();
        total += output
            .split(' ')
            .filter(|s| match s.len() {
                2 | 3 | 4 | 7 => true,
                _ => false,
            })
            .count();
    }
    println!("{}", total);
}
