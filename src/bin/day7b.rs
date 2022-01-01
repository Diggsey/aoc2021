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
    let fuel = (inputs[0]..=inputs[inputs.len() - 1])
        .map(|pos| {
            inputs
                .iter()
                .map(|&v| {
                    let dist = (v - pos).abs();
                    (dist * (dist + 1)) / 2
                })
                .sum::<i32>()
        })
        .min()
        .unwrap();
    println!("{}", fuel);
}
