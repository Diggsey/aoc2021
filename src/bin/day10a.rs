const INPUT: &str = include_str!("../../inputs/day10.txt");

fn score(s: &str) -> u64 {
    match s {
        ")" => 3,
        "]" => 57,
        "}" => 1197,
        ">" => 25137,
        _ => panic!("Invalid character '{}'", s),
    }
}

fn parse(input: &mut &str) -> Result<(), u64> {
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
        parse(input)?;
        if input.is_empty() {
            break;
        }
        let (head, tail) = input.split_at(1);
        *input = tail;
        if head != closing {
            return Err(score(head));
        }
    }
    Ok(())
}

fn parse_line(mut input: &str) -> Result<(), u64> {
    parse(&mut input)?;
    if input.is_empty() {
        Ok(())
    } else {
        Err(score(&input[0..1]))
    }
}

fn main() {
    let total: u64 = INPUT
        .lines()
        .filter_map(|line| parse_line(line).err())
        .sum();

    println!("{}", total);
}
