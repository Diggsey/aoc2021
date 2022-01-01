use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day2.txt");

fn main() {
    let (x, y, _) = INPUT.lines().fold((0, 0, 0), |(mut x, mut y, mut aim), l| {
        let (command, dist) = scan_fmt!(l, "{} {}", String, i32).unwrap();
        match command.as_str() {
            "forward" => {
                x += dist;
                y += aim * dist;
            }
            "down" => aim += dist,
            "up" => aim -= dist,
            _ => panic!("Unknown command"),
        }
        (x, y, aim)
    });
    println!("{}", x * y);
}
