const INPUT: &'static str = include_str!("inputs/day09.txt");

fn parse(input: &str) -> (HashMap<(&str, &str), usize>, Vec<&str>) {
    let distances = input
        .lines()
        .flat_map(|l| {
            let parts = l.split_ascii_whitespace().collect::<Vec<&str>>();
            let dist = parts[4].parse().unwrap();
            vec![((parts[0], parts[2]), dist), ((parts[2], parts[0]), dist)]
        })
        .collect::<HashMap<(&str, &str), usize>>();
    let cities = distances.keys().map(|(l, _)| *l).collect::<HashSet<&str>>();
    let cities = cities.into_iter().collect::<Vec<&str>>();
    (distances, cities)
}

pub fn p1(input: &str) -> usize {
    let (distances, cities) = parse(input);
    Permutation::new(cities.len())
        .map(|p| {
            p.windows(2)
                .map(|w| distances.get(&(cities[w[0]], cities[w[1]])).unwrap())
                .sum()
        })
        .min()
        .unwrap()
}

pub fn p2(input: &str) -> usize {
    let (distances, cities) = parse(input);
    Permutation::new(cities.len())
        .map(|p| {
            p.windows(2)
                .map(|w| distances.get(&(cities[w[0]], cities[w[1]])).unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use dinglebit_combinatorics::Permutation;

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
