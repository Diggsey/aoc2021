use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day2.txt");

fn main() {
    let (x, y) = INPUT
        .lines()
        .map(|l| {
            let (command, dist) = scan_fmt!(l, "{} {}", String, i32).unwrap();
            match command.as_str() {
                "forward" => (dist, 0),
                "down" => (0, dist),
                "up" => (0, -dist),
                _ => panic!("Unknown command"),
            }
        })
        .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
    println!("{}", x * y);
}
