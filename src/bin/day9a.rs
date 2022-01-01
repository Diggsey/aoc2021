const INPUT: &str = include_str!("../../inputs/day9.txt");

fn main() {
    let grid = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut total = 0;
    for y in 0..grid.len() {
        let ya = y.wrapping_sub(1);
        let yb = y + 1;
        let row = &grid[y];
        for x in 0..row.len() {
            let level = row[x];
            let xa = x.wrapping_sub(1);
            let xb = x + 1;

            if ya < grid.len() && grid[ya][x] <= level {
                continue;
            }
            if yb < grid.len() && grid[yb][x] <= level {
                continue;
            }
            if xa < row.len() && row[xa] <= level {
                continue;
            }
            if xb < row.len() && row[xb] <= level {
                continue;
            }

            total += level + 1;
        }
    }
    println!("{}", total);
}
