use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

const INPUT: &str = include_str!("../../inputs/day15.txt");
const SIZE: usize = 500;

fn main() {
    let grid: Vec<Vec<_>> = (0..5)
        .into_iter()
        .flat_map(|y| {
            INPUT.lines().map(move |line| {
                (0..5)
                    .into_iter()
                    .flat_map(|x| {
                        line.chars().map(move |c| {
                            let mut risk = c.to_digit(10).unwrap() + x + y;
                            while risk > 9 {
                                risk -= 9;
                            }
                            risk
                        })
                    })
                    .collect()
            })
        })
        .collect();

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    queue.push((Reverse(0), (0, 0)));

    while let Some((Reverse(risk), (x, y))) = queue.pop() {
        if visited.insert((x, y)) {
            if x == SIZE - 1 && y == SIZE - 1 {
                println!("{}", risk);
                break;
            }
            if x > 0 {
                queue.push((Reverse(risk + grid[y][x - 1]), (x - 1, y)));
            }
            if y > 0 {
                queue.push((Reverse(risk + grid[y - 1][x]), (x, y - 1)));
            }
            if x < SIZE - 1 {
                queue.push((Reverse(risk + grid[y][x + 1]), (x + 1, y)));
            }
            if y < SIZE - 1 {
                queue.push((Reverse(risk + grid[y + 1][x]), (x, y + 1)));
            }
        }
    }
}
