use std::{sync::OnceLock, time::Instant};

use rustc_hash::FxHashMap;

const INPUT: &str = include_str!("inputs/day16.txt");

fn parse(line: &str) -> (isize, Vec<(&str, isize)>) {
    let mut parts = line.splitn(2, ':');
    let name = parts
        .next()
        .unwrap()
        .trim_start_matches("Sue ")
        .parse()
        .unwrap();
    let compounds = parts
        .next()
        .unwrap()
        .split(',')
        .map(|p| {
            let mut parts = p.split(": ");
            (
                parts.next().unwrap().trim(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<(&str, isize)>>();
    (name, compounds)
}

const EXPECTED_VALUES: &str = "children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1";

static EXPECTED: OnceLock<FxHashMap<&'static str, isize>> = OnceLock::new();

fn expected() -> &'static FxHashMap<&'static str, isize> {
    EXPECTED.get_or_init(|| {
        let mut m = FxHashMap::default();
        for line in EXPECTED_VALUES.lines() {
            let mut parts = line.split(": ");
            let name = parts.next().unwrap().trim();
            let count: isize = parts.next().unwrap().parse().unwrap();
            m.insert(name, count);
        }
        m
    })
}

fn find_match<F>(input: &str, f: F) -> isize
where
    F: Fn(&str, isize, isize) -> bool,
{
    input
        .lines()
        .map(parse)
        .find(|(_, compounds)| {
            compounds
                .iter()
                .all(|(name, count)| f(name, *count, *expected().get(name).unwrap()))
        })
        .unwrap()
        .0
}

fn p1(input: &str) -> isize {
    let is_match = |_: &str, count: isize, expected: isize| -> bool { count == expected };
    find_match(input, is_match)
}

fn p2(input: &str) -> isize {
    // closure that returns true when the compound indicates this Sue should be skipped
    let is_match = |name: &str, count: isize, expected: isize| -> bool {
        match name {
            // for cats and trees, the real Sue has MORE than the expected count
            "cats" | "trees" => count > expected,
            // for pomeranians and goldfish, the real Sue has FEWER than the expected count
            "pomeranians" | "goldfish" => count < expected,
            // all others must match exactly
            _ => count == expected,
        }
    };

    find_match(input, is_match)
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
