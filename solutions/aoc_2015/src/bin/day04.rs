use md5::{Digest, Md5};
use rayon::prelude::*;

const INPUT: &'static str = include_str!("inputs/day04.txt");

#[allow(dead_code)]
fn find_hash_no_format(input: &str, start_size: usize) -> i32 {
    let input = input.trim();

    let zeros = start_size / 2;
    let left_over = start_size % 2;

    'outer: for i in 1.. {
        let mut hasher = Md5::new();
        hasher.reset();
        hasher.update(input.as_bytes());
        hasher.update(format!("{}", i).as_bytes());

        let results = hasher.finalize();
        for j in 0..zeros {
            if results[j] != 0 {
                continue 'outer;
            }
        }
        if left_over > 0 {
            let rest = results[zeros] >> 4;
            if rest == 0 {
                return i;
            }
        } else {
            return i;
        }
    }

    0
}

#[allow(dead_code)]
fn find_hash(input: &str, start: &str) -> i32 {
    let input = input.trim();

    for i in 1.. {
        let mut hasher = Md5::new();
        hasher.reset();
        hasher.update(input.as_bytes());
        hasher.update(format!("{}", i).as_bytes());

        let results = hasher.finalize();
        let hex = format!("{:x}", results);
        if hex.starts_with(start) {
            return i;
        }
    }

    0
}

#[allow(dead_code)]
fn find_hash_fast(input: &str, start_size: usize) -> i32 {
    let input = input.trim();

    // We are going to track the number or early bytes we need to be zeros. If we don't have an even number, we'll want to take the first part of the next byte (left_over).
    let zeros = start_size / 2;
    let left_over = start_size % 2;

    let result = (1..i32::MAX)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|&i| {
            // Generate the hash.
            let mut hasher = Md5::new();
            hasher.update(input.as_bytes());
            hasher.update(format!("{}", i).as_bytes());
            let results = hasher.finalize();

            // Check to see if we got the first bytes correct.
            if !(results[..zeros].iter().all(|&x| x == 0)) {
                return false;
            }
            // If we have a leftover, check it as well.
            if left_over > 0 {
                if (results[zeros] >> 4) == 0 {
                    return true;
                }
            } else {
                // If we got to this point, we don't have any leftovers and all the previous bytes were zero.
                return true;
            }

            false
        });

    result.unwrap_or(0)
}

pub fn p1(input: &str) -> i32 {
    // find_hash(input, "00000")
    // find_hash_no_format(input, 5)
    find_hash_fast(input, 5)
}

pub fn p2(input: &str) -> i32 {
    // find_hash(input, "000000")
    // find_hash_no_format(input, 6)
    find_hash_fast(input, 6)
}

fn main() {
    let now = std::time::Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = std::time::Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
