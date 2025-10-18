use std::{iter::once, time::Instant};

use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day18.txt");

type Input<'a> = Vec<char>;

fn parse_input(input: &'_ str) -> Input<'_> {
    input.trim().chars().collect()
}

fn next<'a>(input: &'a Input) -> Input<'a> {
    once('.')
        .chain(input.iter().cloned())
        .chain(once('.'))
        .tuple_windows()
        .map(|(l, _, r)| match (l, r) {
            ('.', '^') => '^',
            ('^', '.') => '^',
            _ => '.',
        })
        .collect()
}

fn next_n(input: &Input, n: usize) -> usize {
    let mut input = input.clone();
    let mut count = input.iter().filter(|c| **c == '.').count();
    for _ in 0..n - 1 {
        input = next(&input);
        count += input.iter().filter(|c| **c == '.').count();
    }

    count
}

fn next_bits(input: u128, k: usize) -> u128 {
    let mut next = (input >> (k - 1)) & 1;
    for i in (1..k).rev() {
        next <<= 1;
        next |= ((input >> (i + 1)) & 1) ^ ((input >> (i - 1)) & 1);
    }
    next <<= 1;
    next |= (input >> 1) & 1;
    next
}

fn count_zeros(mut input: u128, k: usize) -> usize {
    let mut count = 0;
    for _ in 0..k {
        if input & 1 == 0 {
            count += 1;
        }
        input >>= 1;
    }
    count
}

fn next_n_bits(input: &Input, n: usize) -> usize {
    let k = input.len();
    let mut bits = input
        .iter()
        .fold(0_u128, |acc, c| (acc << 1) | if *c == '^' { 1 } else { 0 });
    let mut count = count_zeros(bits, k);
    for _ in 0..n - 1 {
        bits = next_bits(bits, k);
        count += count_zeros(bits, k);
    }
    count
}

fn p1(input: &Input) -> usize {
    next_n(input, 40)
}

fn p2(input: &Input) -> usize {
    next_n(input, 400000)
}

fn main() {
    let now = Instant::now();
    let input = parse_input(INPUT);
    let solution = p1(&input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(&input);
    println!("p2 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = next_n_bits(&input, 40);
    println!("p1_bits {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = next_n_bits(&input, 400000);
    println!("p2_bits {:?} {}", now.elapsed(), solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = ".^^.^.^^^^".chars().collect();
        assert_eq!(next_n_bits(&input, 10), 38);
    }
}
