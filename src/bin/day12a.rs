use std::collections::{BTreeSet, HashMap};

const INPUT: &str = include_str!("../../inputs/day12.txt");

struct Graph {
    edges: HashMap<&'static str, Vec<&'static str>>,
}

impl Graph {
    fn visit(&self, from: &'static str, mut visited: BTreeSet<&'static str>) -> u64 {
        if from == "end" {
            1
        } else {
            let mut res = 0;
            if from.chars().next().unwrap().is_ascii_lowercase() {
                visited.insert(from);
            }
            for to in self.edges[from].iter().copied() {
                if !visited.contains(&to) {
                    res += self.visit(to, visited.clone());
                }
            }
            res
        }
    }
}

fn main() {
    let mut edges = HashMap::<_, Vec<_>>::new();
    for line in INPUT.lines() {
        let (a, b) = line.split_once('-').unwrap();
        edges.entry(a).or_default().push(b);
        edges.entry(b).or_default().push(a);
    }
    let total = Graph { edges }.visit("start", BTreeSet::new());

    println!("{}", total);
}
