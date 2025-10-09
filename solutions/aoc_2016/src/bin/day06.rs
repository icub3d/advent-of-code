use std::time::Instant;

use rustc_hash::FxHashMap;

const INPUT: &str = include_str!("inputs/day06.txt");

type Input<'a> = Vec<&'a str>;

fn parse_input(input: &'_ str) -> Input<'_> {
    input.lines().collect()
}

fn p1(input: &Input) -> String {
    let mut frequencies: Vec<FxHashMap<char, usize>> = vec![FxHashMap::default(); input[0].len()];

    input.iter().for_each(|l| {
        l.chars().enumerate().for_each(|(i, c)| {
            *frequencies[i].entry(c).or_insert(0) += 1;
        });
    });

    frequencies
        .iter()
        .map(|m| m.iter().max_by_key(|&(_, c)| c).map(|(c, _)| c).unwrap())
        .collect()
}

fn p2(input: &Input) -> String {
    let mut frequencies: Vec<FxHashMap<char, usize>> = vec![FxHashMap::default(); input[0].len()];

    input.iter().for_each(|l| {
        l.chars().enumerate().for_each(|(i, c)| {
            *frequencies[i].entry(c).or_insert(0) += 1;
        });
    });

    frequencies
        .iter()
        .map(|m| m.iter().min_by_key(|&(_, c)| c).map(|(c, _)| c).unwrap())
        .collect()
}

fn main() {
    let now = Instant::now();
    let input = parse_input(INPUT);
    let solution = p1(&input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(&input);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
