use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day21.txt");

struct PlayerState {
    pos: u64,
    score: u64,
}

impl PlayerState {
    fn new(pos: u64) -> Self {
        Self {
            pos: pos - 1,
            score: 0,
        }
    }
    fn advance(&mut self, die: &mut impl Iterator<Item = u64>) -> bool {
        let steps: u64 = die.take(3).map(|v| (v % 1000) + 1).sum();
        self.pos = (self.pos + steps) % 10;
        self.score += self.pos + 1;
        self.score >= 1000
    }
}

fn main() {
    let mut lines = INPUT.lines();
    let mut player1 = PlayerState::new(
        scan_fmt!(lines.next().unwrap(), "Player 1 starting position: {}", u64).unwrap(),
    );
    let mut player2 = PlayerState::new(
        scan_fmt!(lines.next().unwrap(), "Player 2 starting position: {}", u64).unwrap(),
    );
    let mut die = 0..;
    let losing_score = loop {
        if player1.advance(&mut die) {
            break player2.score;
        }
        if player2.advance(&mut die) {
            break player1.score;
        }
    };

    println!("{}", losing_score * die.next().unwrap());
}
