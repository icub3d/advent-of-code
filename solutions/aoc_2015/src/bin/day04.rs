use md5::{Digest, Md5};
use rayon::prelude::*;

const INPUT: &str = include_str!("inputs/day04.txt");

fn find_hash(input: &str, start_size: usize) -> i32 {
    let input = input.trim();

    // We are going to track the number or early bytes we need to be zeros. If we don't have an even number, we'll want to take the first part of the next byte (left_over).
    let zeros = start_size / 2;
    let left_over = start_size % 2;

    // As the initial part of the hash outside the loop.
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());

    let result = (1..i32::MAX)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|&i| {
            // Generate the hash.
            let mut hasher = hasher.clone();
            hasher.update(i.to_string().as_bytes());
            let results = hasher.finalize();

            // A simplified version of the original checks.
            // All initial full bytes must be zero, and if there's a leftover nibble
            // the high nibble of the next byte must also be zero.
            results.iter().take(zeros).all(|&b| b == 0)
                && (left_over == 0 || (results[zeros] & 0xF0) == 0)
        });

    result.unwrap()
}

fn p1(input: &str) -> i32 {
    find_hash(input, 5)
}

fn p2(input: &str) -> i32 {
    find_hash(input, 6)
}

fn main() {
    let now = std::time::Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = std::time::Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
