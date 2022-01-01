use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day17.txt");

fn main() {
    let (_, _, y0, _) =
        scan_fmt!(INPUT, "target area: x={}..{}, y={}..{}", i32, i32, i32, i32).unwrap();

    println!("{}", (y0 * y0 + y0) / 2);
}
