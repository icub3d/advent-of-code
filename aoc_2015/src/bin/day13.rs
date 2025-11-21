use std::time::Instant;

use itertools::Itertools;
use rustc_hash::FxHashMap;

const INPUT: &str = include_str!("inputs/day13.txt");

fn parse(input: &str) -> (FxHashMap<(&str, &str), isize>, Vec<&str>) {
    let happiness_index = input
        .lines()
        .map(|l| {
            let parts = l
                .trim_end_matches('.')
                .split_ascii_whitespace()
                .collect::<Vec<&str>>();
            let negate = match parts[2] {
                "gain" => 1,
                _ => -1,
            };
            (
                (parts[0], parts[10]),
                parts[3].parse::<isize>().unwrap() * negate,
            )
        })
        .collect::<FxHashMap<(&str, &str), isize>>();

    let people = happiness_index.keys().map(|(l, _)| *l).unique().collect();
    (happiness_index, people)
}

fn calculate_happiness(
    happiness_index: &FxHashMap<(&str, &str), isize>,
    arrangement: &[&&str],
) -> isize {
    arrangement
        .iter()
        .circular_tuple_windows::<(_, _)>()
        .map(|(&left, &right)| {
            happiness_index.get(&(left, right)).unwrap_or(&0)
                + happiness_index.get(&(right, left)).unwrap_or(&0)
        })
        .sum()
}

fn max_happiness(happiness_index: &FxHashMap<(&str, &str), isize>, people: &[&str]) -> isize {
    people
        .iter()
        .permutations(people.len())
        .map(|p| calculate_happiness(happiness_index, &p))
        .max()
        .unwrap()
}

fn p1(input: &str) -> isize {
    let (happiness_index, people) = parse(input);
    max_happiness(&happiness_index, &people)
}

fn p2(input: &str) -> isize {
    let (happiness_index, mut people) = parse(input);
    people.push("Me");

    max_happiness(&happiness_index, &people)
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
