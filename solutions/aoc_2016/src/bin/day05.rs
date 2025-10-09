use std::time::Instant;

use md5::{Digest, Md5};
use rayon::prelude::*;

const INPUT: &str = include_str!("inputs/day05.txt");

type Input<'a> = &'a str;

fn parse_input(input: &'_ str) -> Input<'_> {
    input.trim()
}

fn p1(input: &Input) -> String {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());

    let mut cur = 0;
    let mut block_size = 2048;
    let mut solution = Vec::new();

    while solution.len() < 8 {
        let found = (cur..cur + block_size)
            .into_par_iter()
            .by_exponential_blocks()
            .filter_map(|i| {
                // Generate the hash.
                let mut hasher = hasher.clone();
                hasher.update(i.to_string().as_bytes());
                let results = hasher.finalize();

                // If the hex of the hash starts with 00000, then we can use the 6th letter.
                if results[0] == 0 && results[1] == 0 && results[2] & 0xF0 == 0 {
                    Some(std::char::from_digit((results[2] & 0x0F) as u32, 16).unwrap())
                } else {
                    None
                }
            })
            .collect::<Vec<char>>();
        solution.extend(found);
        cur += block_size;
        block_size *= 2;
    }

    solution.iter().take(8).collect()
}

fn p2(input: &Input) -> String {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());

    let mut password = [' '; 8];

    let mut cur = 0;
    let mut block_size = 2048;

    while password.iter().filter(|c| **c == ' ').count() != 0 {
        let found = (cur..cur + block_size)
            .into_par_iter()
            .by_exponential_blocks()
            .filter_map(|i| {
                // Generate the hash.
                let mut hasher = hasher.clone();
                hasher.update(i.to_string().as_bytes());
                let results = hasher.finalize();

                // If the hex of the hash starts with 00000, then we can use the 6th letter.
                if results[0] == 0 && results[1] == 0 && results[2] & 0xF0 == 0 {
                    let n = (results[2] & 0x0F) as usize;
                    if n <= 7 {
                        return Some((
                            n,
                            std::char::from_digit((results[3] >> 4) as u32, 16).unwrap(),
                        ));
                    }
                }
                None
            })
            .collect::<Vec<(usize, char)>>();

        for (n, c) in found {
            if password[n] == ' ' {
                password[n] = c;
            }
        }

        cur += block_size;
        block_size *= 2;
    }
    password.iter().collect()
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
