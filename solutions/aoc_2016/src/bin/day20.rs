use std::time::Instant;

use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day20.txt");

type Input<'a> = Vec<(u32, u32)>;

fn parse_input(input: &'_ str) -> Input<'_> {
    let mut input = input
        .trim()
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .collect::<Input>();

    input.sort_by_key(|t| t.0);

    let mut merged = Vec::new();
    let mut prev = input[0];
    for &cur in input.iter().skip(1) {
        // Do we need to extend?
        if cur.0 <= prev.1 {
            prev.1 = cur.1.max(prev.1);
        } else {
            merged.push(prev);
            prev = cur;
        }
    }
    merged.push(prev);

    merged
}

fn p1(input: &Input) -> u32 {
    input
        .iter()
        .tuple_windows()
        .find(|(left, right)| right.0 - 1 > left.1)
        .map(|(left, _)| left.1 + 1)
        .unwrap()
}

fn p2(input: &Input) -> u32 {
    input
        .iter()
        .tuple_windows()
        .map(|(left, right)| right.0 - left.1 - 1)
        .sum::<u32>()
        + (u32::MAX - input[input.len() - 1].1)
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

