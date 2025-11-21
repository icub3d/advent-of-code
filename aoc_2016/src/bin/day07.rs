use std::time::Instant;

use rustc_hash::FxHashSet;

const INPUT: &str = include_str!("inputs/day07.txt");

type Input<'a> = Vec<(Vec<&'a str>, Vec<&'a str>)>;

fn split_brackets(mut input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut out = Vec::new();
    let mut inside = Vec::new();

    while let Some((left, right)) = input.split_once('[') {
        out.push(left);
        let (left, right) = right.split_once(']').unwrap();
        inside.push(left);
        input = right;
    }

    if !input.is_empty() {
        out.push(input)
    }

    (out, inside)
}

fn parse_input(input: &'_ str) -> Input<'_> {
    input.lines().map(split_brackets).collect()
}

fn contains_abba(chunk: &&str) -> bool {
    chunk
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
}

fn p1(input: &Input) -> usize {
    input
        .iter()
        .filter(|(outside, inside)| {
            outside.iter().any(contains_abba) && !inside.iter().any(contains_abba)
        })
        .count()
}

// Find and the ABA groupings and return their equivalent BABs.
fn find_abas(chunk: &&str) -> FxHashSet<String> {
    chunk
        .chars()
        .collect::<Vec<_>>()
        .windows(3)
        .filter(|w| w[0] == w[2] && w[0] != w[1])
        .map(|w| format!("{}{}{}", w[1], w[0], w[1]))
        .collect::<FxHashSet<_>>()
}

fn p2(input: &Input) -> usize {
    input
        .iter()
        .filter(|(outside, inside)| {
            let abas = outside.iter().flat_map(find_abas).collect::<FxHashSet<_>>();
            inside
                .iter()
                .any(|chunk| abas.iter().any(|a| chunk.contains(a)))
        })
        .count()
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
