const INPUT: &str = include_str!("../../inputs/day9.txt");

fn fill(grid: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u64 {
    if y >= grid.len() || x >= grid[y].len() || grid[y][x] == 9 {
        0
    } else {
        grid[y][x] = 9;
        1 + fill(grid, x.wrapping_sub(1), y)
            + fill(grid, x, y.wrapping_sub(1))
            + fill(grid, x + 1, y)
            + fill(grid, x, y + 1)
    }
}

fn main() {
    let mut grid = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sizes = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            sizes.push(fill(&mut grid, x, y));
        }
    }
    sizes.sort();
    let total: u64 = sizes.iter().rev().take(3).product();
    println!("{}", total);
}
