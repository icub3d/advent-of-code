use std::{collections::VecDeque, time::Instant};

use md5::{Digest, Md5};
use rayon::prelude::*;

const INPUT: &str = include_str!("inputs/day14.txt");

type Input<'a> = &'a str;

fn parse_input(input: &'_ str) -> Input<'_> {
    input.trim()
}

fn get_triple_and_quintuples(
    mut hasher: Md5,
    extra_hashes: usize,
    id: usize,
) -> (Option<char>, Vec<char>) {
    hasher.update(id.to_string().as_bytes());
    let mut hash = format!("{:x}", hasher.finalize());

    (0..extra_hashes).for_each(|_| {
        let mut hasher = Md5::new();
        hasher.update(hash.as_bytes());
        hash = format!("{:x}", hasher.finalize());
    });

    let triple = hash
        .as_bytes()
        .windows(3)
        .find(|w| w[0] == w[1] && w[1] == w[2])
        .map(|w| w[0] as char);

    let quintuples = hash
        .as_bytes()
        .windows(5)
        .filter(|w| w.iter().all(|c| *c == w[0]))
        .map(|w| w[0] as char)
        .collect();

    (triple, quintuples)
}

fn find_nth(input: &Input, n: usize, extra_hashes: usize) -> usize {
    // Get our initialized hasher
    let mut hasher = Md5::new();
    hasher.update(input);

    // Fill up our window.
    let mut window = (0..22000)
        .into_par_iter()
        .map(|i| get_triple_and_quintuples(hasher.clone(), extra_hashes, i))
        .collect::<VecDeque<_>>();

    let mut found = 0;
    for id in 0.. {
        if window.len() <= 1000 {
            window.push_back(get_triple_and_quintuples(
                hasher.clone(),
                extra_hashes,
                id + 1000,
            ));
        }

        let (triple, _) = window.pop_front().unwrap();
        if let Some(triple) = triple
            && window.iter().take(1000).any(|(_, q)| q.contains(&triple))
        {
            found += 1;
            if found == n {
                return id;
            }
        }
    }

    panic!("not found")
}
fn p1(input: &Input) -> usize {
    find_nth(input, 64, 0)
}

fn p2(input: &Input) -> usize {
    find_nth(input, 64, 2016)
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
