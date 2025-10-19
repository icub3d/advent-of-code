use std::time::Instant;

use rustc_hash::FxHashMap;

const INPUT: &str = include_str!("inputs/day06.txt");

type Input<'a> = Vec<FxHashMap<char, usize>>;

fn parse_input(input: &'_ str) -> Input<'_> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut frequencies: Vec<FxHashMap<char, usize>> = vec![FxHashMap::default(); lines[0].len()];
    lines.iter().for_each(|l| {
        for (idx, ch) in l.chars().enumerate() {
            *frequencies[idx].entry(ch).or_insert(0) += 1;
        }
    });
    frequencies
}

fn p1(input: &Input) -> String {
    solve_by_cmp(input, usize::cmp)
}

fn p2(input: &Input) -> String {
    solve_by_cmp(input, |a, b| b.cmp(a))
}

fn solve_by_cmp<F>(input: &Input, cmp: F) -> String
where
    F: Fn(&usize, &usize) -> std::cmp::Ordering,
{
    input
        .iter()
        .map(|column| {
            column
                .iter()
                .max_by(|a, b| cmp(a.1, b.1))
                .map(|(ch, _)| *ch)
                .unwrap()
        })
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
