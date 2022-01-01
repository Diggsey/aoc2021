const INPUT: &str = include_str!("../../inputs/day1.txt");

fn main() {
    let values: Vec<i32> = INPUT.lines().map(|l| l.parse().unwrap()).collect();
    let count = values.windows(4).filter(|w| w[3] > w[0]).count();
    println!("{}", count);
}
