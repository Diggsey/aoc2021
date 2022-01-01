const INPUT: &str = include_str!("../../inputs/day10.txt");

fn score(s: &str) -> u64 {
    match s {
        ")" => 1,
        "]" => 2,
        "}" => 3,
        ">" => 4,
        _ => panic!("Invalid character '{}'", s),
    }
}

fn parse(input: &mut &str) -> Option<u64> {
    while !input.is_empty() {
        let (head, tail) = input.split_at(1);
        let closing = match head {
            "[" => "]",
            "(" => ")",
            "{" => "}",
            "<" => ">",
            _ => break,
        };
        *input = tail;
        let prev_score = parse(input)?;
        if input.is_empty() {
            return Some(prev_score * 5 + score(closing));
        }
        let (head, tail) = input.split_at(1);
        *input = tail;
        if head != closing {
            return None;
        }
    }
    Some(0)
}

fn parse_line(mut input: &str) -> Option<u64> {
    parse(&mut input)
}

fn main() {
    let mut scores: Vec<_> = INPUT.lines().filter_map(|line| parse_line(line)).collect();
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}
