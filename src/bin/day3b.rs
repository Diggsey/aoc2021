const INPUT: &str = include_str!("../../inputs/day3.txt");

fn find(input: &[u32], index: usize, mode: bool) -> u32 {
    if input.len() == 1 {
        return input[0];
    }
    let count = input.iter().filter(|&v| (v & (1 << index)) != 0).count();
    let which = (count > (input.len() - 1) / 2) == mode;
    let new_input: Vec<_> = input
        .iter()
        .copied()
        .filter(|v| ((v & (1 << index)) != 0) == which)
        .collect();
    find(&new_input, index.wrapping_sub(1), mode)
}

fn main() {
    let numbers: Vec<_> = INPUT
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();
    let ogr = find(&numbers, 11, true);
    let csr = find(&numbers, 11, false);
    println!("{}", ogr * csr);
}
