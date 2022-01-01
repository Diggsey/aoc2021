const INPUT: &str = include_str!("../../inputs/day25.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    East,
    South,
}

fn step(grid: &mut Vec<Vec<Cell>>, scratch: &mut Vec<Vec<Cell>>) -> usize {
    let (sy, sx) = (grid.len(), grid[0].len());
    let mut count = 0;
    for y in 0..sy {
        for x in 0..sx {
            let src = grid[y][x];
            scratch[y][x] = match src {
                Cell::Empty if grid[y][(x + sx - 1) % sx] == Cell::East => {
                    count += 1;
                    Cell::East
                }
                Cell::East if grid[y][(x + 1) % sx] == Cell::Empty => Cell::Empty,
                _ => src,
            }
        }
    }
    for y in 0..sy {
        for x in 0..sx {
            let src = scratch[y][x];
            grid[y][x] = match src {
                Cell::Empty if scratch[(y + sy - 1) % sy][x] == Cell::South => {
                    count += 1;
                    Cell::South
                }
                Cell::South if scratch[(y + 1) % sy][x] == Cell::Empty => Cell::Empty,
                _ => src,
            }
        }
    }
    count
}

fn display(grid: &Vec<Vec<Cell>>) {
    for row in grid {
        for cell in row {
            print!(
                "{}",
                match cell {
                    Cell::Empty => ".",
                    Cell::East => ">",
                    Cell::South => "v",
                }
            );
        }
        println!();
    }
    println!();
}

fn main() {
    let mut grid: Vec<Vec<Cell>> = INPUT
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '>' => Cell::East,
                    'v' => Cell::South,
                    _ => unimplemented!(),
                })
                .collect()
        })
        .collect();
    let mut scratch = grid.clone();

    for i in 1.. {
        display(&grid);
        if step(&mut grid, &mut scratch) == 0 {
            println!("{}", i);
            break;
        }
    }
}
