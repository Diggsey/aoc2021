use std::collections::BTreeMap;

use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day21.txt");
const TRINOMIAL: [(u64, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct PlayerState {
    score: u64,
    pos: u64,
}

impl PlayerState {
    fn new(pos: u64) -> Self {
        Self {
            pos: pos - 1,
            score: 0,
        }
    }
    fn advance(&mut self, steps: u64) -> bool {
        self.pos = (self.pos + steps) % 10;
        self.score += self.pos + 1;
        self.score >= 21
    }
}

fn pop_first<K: Ord + Clone, V>(queue: &mut BTreeMap<K, V>) -> Option<(K, V)> {
    let k = queue.keys().next()?.clone();
    queue.remove_entry(&k)
}

fn main() {
    let mut lines = INPUT.lines();
    let player1 = PlayerState::new(
        scan_fmt!(lines.next().unwrap(), "Player 1 starting position: {}", u64).unwrap(),
    );
    let player2 = PlayerState::new(
        scan_fmt!(lines.next().unwrap(), "Player 2 starting position: {}", u64).unwrap(),
    );
    let mut queue = BTreeMap::new();
    queue.insert((player1, player2), 1);
    let mut player1_wins = 0;
    let mut player2_wins = 0;
    while let Some(((player1, player2), count)) = pop_first(&mut queue) {
        for (steps, freq1) in TRINOMIAL {
            let mut player1 = player1.clone();
            if player1.advance(steps) {
                player1_wins += count * freq1;
            } else {
                for (steps, freq2) in TRINOMIAL {
                    let mut player2 = player2.clone();
                    if player2.advance(steps) {
                        player2_wins += count * freq1 * freq2;
                    } else {
                        *queue.entry((player1.clone(), player2)).or_default() +=
                            count * freq1 * freq2;
                    }
                }
            }
        }
    }

    println!("{}", player1_wins.max(player2_wins));
}
