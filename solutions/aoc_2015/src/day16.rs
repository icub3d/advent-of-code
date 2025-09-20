use std::{collections::HashMap, time::Instant};

const INPUT: &'static str = include_str!("inputs/day16.txt");

fn parse<'a>(line: &'a str) -> (isize, Vec<(&'a str, isize)>) {
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

const EXPECTED: &'static str = "children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1";

pub fn p1(input: &str) -> isize {
    let is_match = |_: &str, count: isize, expected: isize| -> bool { count == expected };
    find_match(input, is_match)
}

pub fn p2(input: &str) -> isize {
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

fn find_match<F>(input: &str, f: F) -> isize
where
    F: Fn(&str, isize, isize) -> bool,
{
    let expected = EXPECTED
        .lines()
        .map(|l| {
            let mut parts = l.split(": ");
            (
                parts.next().unwrap(),
                parts.next().unwrap().parse::<isize>().unwrap(),
            )
        })
        .collect::<HashMap<&str, isize>>();

    input
        .lines()
        .map(parse)
        .find(|(_, compounds)| {
            compounds
                .iter()
                .all(|(name, count)| f(name, *count, *expected.get(name).unwrap()))
        })
        .unwrap()
        .0
}

pub fn solve() -> anyhow::Result<()> {
    let now = Instant::now();
    println!("p1: {} ({:?})", p1(INPUT), now.elapsed());
    let now = Instant::now();
    println!("p2: {} ({:?})", p2(INPUT), now.elapsed());
    Ok(())
}
