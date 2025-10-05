use std::{
    collections::{HashMap, HashSet},
    iter::once,
    time::Instant,
};

use dinglebit_combinatorics::Permutation;

const INPUT: &'static str = include_str!("inputs/day13.txt");

pub fn parse(input: &str) -> (HashMap<(&str, &str), isize>, Vec<&str>) {
    let happiness_index = input
        .lines()
        .map(|l| {
            let parts = l.split_ascii_whitespace().collect::<Vec<&str>>();
            let negate = match parts[2] {
                "gain" => 1,
                _ => -1,
            };
            (
                (parts[0], parts[10].trim_end_matches('.')),
                parts[3].parse::<isize>().unwrap() * negate,
            )
        })
        .collect::<HashMap<(&str, &str), isize>>();

    let people = happiness_index
        .keys()
        .map(|(l, _)| *l)
        .collect::<HashSet<&str>>();
    let people = people.into_iter().collect::<Vec<&str>>();
    (happiness_index, people)
}

pub fn p1(input: &str) -> isize {
    let (happiness_index, people) = parse(input);
    calculate_happiness(&happiness_index, &people)
}

pub fn p2(input: &str) -> isize {
    let (happiness_index, mut people) = parse(input);
    people.push("Me");

    calculate_happiness(&happiness_index, &people)
}

fn calculate_happiness(
    happiness_index: &HashMap<(&str, &str), isize>,
    people: &Vec<&str>,
) -> isize {
    Permutation::new(people.len())
        .map(|p| {
            p.windows(2)
                .chain(once(&[p[0], p[people.len() - 1]][..]))
                .map(|w| {
                    happiness_index
                        .get(&(people[w[0]], people[w[1]]))
                        .unwrap_or(&0)
                        + happiness_index
                            .get(&(people[w[1]], people[w[0]]))
                            .unwrap_or(&0)
                })
                .sum()
        })
        .max()
        .unwrap()
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
