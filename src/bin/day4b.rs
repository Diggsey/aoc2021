use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day4.txt");
const SIZE: usize = 5;

#[derive(Default)]
struct Board {
    numbers: HashMap<usize, (usize, usize)>,
    cols: [usize; SIZE],
    rows: [usize; SIZE],
    won: bool,
}

impl Board {
    fn base_score(&self) -> usize {
        self.numbers.keys().copied().sum()
    }
    fn mark(&mut self, number: usize) -> Option<usize> {
        if !self.won {
            if let Some((x, y)) = self.numbers.remove(&number) {
                self.cols[x] += 1;
                self.rows[y] += 1;
                if self.cols[x] >= SIZE || self.rows[y] >= SIZE {
                    self.won = true;
                    return Some(self.base_score() * number);
                }
            }
        }
        None
    }
}

fn main() {
    let mut lines = INPUT.lines();
    let numbers: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut boards = Vec::new();
    lines.next().unwrap();
    loop {
        let mut board = Board::default();
        for y in 0..SIZE {
            let line = lines.next().unwrap();
            for (x, number) in line
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .enumerate()
            {
                board.numbers.insert(number, (x, y));
            }
        }
        boards.push(board);
        if lines.next().is_none() {
            break;
        }
    }

    for number in numbers {
        for board in boards.iter_mut() {
            if let Some(score) = board.mark(number) {
                println!("{}", score);
            }
        }
    }
}
