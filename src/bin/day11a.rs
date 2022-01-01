const INPUT: &str = include_str!("../../inputs/day11.txt");

fn inc(grid: &mut Vec<Vec<u32>>, x: i32, y: i32) {
    if x < 0 || x >= 10 || y < 0 || y >= 10 {
        return;
    }
    let cell = &mut grid[y as usize][x as usize];
    if *cell < 10 {
        *cell += 1;
        if *cell == 10 {
            inc(grid, x - 1, y - 1);
            inc(grid, x, y - 1);
            inc(grid, x + 1, y - 1);
            inc(grid, x - 1, y);
            inc(grid, x + 1, y);
            inc(grid, x - 1, y + 1);
            inc(grid, x, y + 1);
            inc(grid, x + 1, y + 1);
        }
    }
}

fn flash(grid: &mut Vec<Vec<u32>>) -> u64 {
    let mut total = 0;
    for y in 0..10 {
        for x in 0..10 {
            if grid[y][x] >= 10 {
                grid[y][x] = 0;
                total += 1;
            }
        }
    }
    total
}

fn main() {
    let mut grid: Vec<_> = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut total = 0;
    for _ in 0..100 {
        for y in 0..10 {
            for x in 0..10 {
                inc(&mut grid, x, y);
            }
        }
        total += flash(&mut grid);
    }

    println!("{}", total);
}
