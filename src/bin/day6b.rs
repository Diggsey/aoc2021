const INPUT: &str = include_str!("../../inputs/day6.txt");

fn calc_num_adults(cache: &mut Vec<u64>, t: isize) -> u64 {
    if t < 0 {
        0
    } else if (t as usize) < cache.len() {
        cache[t as usize]
    } else {
        let res = calc_num_adults(cache, t - 9) + calc_num_adults(cache, t - 7);
        calc_num_adults(cache, t - 1);
        cache.push(res);
        res
    }
}

fn calc_total_fish(cache: &mut Vec<u64>, t: isize) -> u64 {
    (0..9).map(|i| calc_num_adults(cache, t - i)).sum()
}

fn main() {
    let mut cache = vec![1];
    let inputs: Vec<_> = INPUT
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect();

    let t = 256;
    let mut total = 0;
    for &input in &inputs {
        total += calc_total_fish(&mut cache, t + (6 - input));
    }
    println!("{}", total);
}
