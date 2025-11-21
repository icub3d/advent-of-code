use std::time::Instant;

use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day11.txt");

fn increment(cur: &mut [u8]) {
    let mut pos = cur.len() - 1;
    loop {
        if cur[pos] != b'z' {
            cur[pos] += 1;
            break;
        }

        cur[pos] = b'a';
        if pos == 0 {
            break;
        }
        pos -= 1;
    }
}

fn has_straight(password: &[u8]) -> bool {
    password
        .windows(3)
        .any(|w| w[0] == w[1] - 1 && w[1] == w[2] - 1)
}

fn no_banned_chars(password: &[u8]) -> bool {
    !password
        .iter()
        .any(|&c| c == b'i' || c == b'o' || c == b'l')
}

fn has_two_pairs(password: &[u8]) -> bool {
    password.iter().dedup_with_count()
        .filter(|&(count, _)| count >= 2)
        .map(|(count, _)| count / 2)
        .sum::<usize>() >= 2
}

pub fn valid(password: &[u8]) -> bool {
    has_straight(password) && no_banned_chars(password) && has_two_pairs(password)
}

fn find_next_valid_password(password: &str) -> String {
    let mut cur = password.chars().map(|c| c as u8).collect::<Vec<u8>>();
    loop {
        increment(&mut cur);
        if valid(&cur) {
            break;
        }
    }

    cur.iter().map(|i| *i as char).collect()
}

fn p1(input: &str) -> String {
    find_next_valid_password(input.trim())
}

fn p2(input: &str) -> String {
    find_next_valid_password(input.trim())
}

fn main() {
    let now = Instant::now();
    let password = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), password);

    let now = Instant::now();
    let solution = p2(&password);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
