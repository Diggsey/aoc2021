const INPUT: &str = include_str!("../../inputs/day7.txt");

fn main() {
    let mut inputs: Vec<_> = INPUT
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    inputs.sort();
    let median = inputs[inputs.len() / 2];
    let fuel: i32 = inputs.into_iter().map(|v| (v - median).abs()).sum();
    println!("{}", fuel);
}
