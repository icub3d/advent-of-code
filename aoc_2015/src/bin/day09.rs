use std::time::Instant;

use itertools::Itertools;
use rustc_hash::FxHashMap;

const INPUT: &str = include_str!("inputs/day09.txt");

fn parse(input: &str) -> (FxHashMap<(&str, &str), usize>, Vec<&str>) {
    let distances = input
        .lines()
        .flat_map(|l| {
            let parts = l.split_ascii_whitespace().collect::<Vec<&str>>();
            let dist = parts[4].parse().unwrap();
            vec![((parts[0], parts[2]), dist), ((parts[2], parts[0]), dist)]
        })
        .collect::<FxHashMap<(&str, &str), usize>>();
    let cities = distances
        .keys()
        .map(|(l, _)| *l)
        .unique()
        .collect::<Vec<&str>>();
    (distances, cities)
}

fn p1(input: &str) -> usize {
    let (distances, cities) = parse(input);
    cities
        .iter()
        .permutations(cities.len())
        .map(|p| {
            p.iter()
                .tuple_windows()
                .map(|(left, right)| distances.get(&(*left, *right)).unwrap())
                .sum()
        })
        .min()
        .unwrap()
}

fn p2(input: &str) -> usize {
    let (distances, cities) = parse(input);
    cities
        .iter()
        .permutations(cities.len())
        .map(|p| {
            p.iter()
                .tuple_windows()
                .map(|(left, right)| distances.get(&(*left, *right)).unwrap())
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
