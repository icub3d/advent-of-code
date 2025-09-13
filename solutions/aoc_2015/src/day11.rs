const INPUT: &'static str = include_str!("inputs/day11.txt");

fn increment(cur: &Vec<u8>) -> Vec<u8> {
    let mut next = cur.clone();

    let mut pos = next.len() - 1;
    loop {
        if next[pos] != b'z' {
            next[pos] += 1;
            // If you need to optimize for longer increments, this part would help:
            // if next[pos] == b'i' || next[pos] == b'o' || next[pos] == b'l' {
            //     next[pos] += 1;
            //     for i in pos + 1..next.len() {
            //         next[i] = b'a';
            //     }
            // }
            break;
        }

        next[pos] = b'a';
        if pos == 0 {
            break;
        }
        pos -= 1;
    }

    next
}

pub fn valid(password: &Vec<u8>) -> bool {
    if !password
        .windows(3)
        .map(|w| w[0] == w[1] - 1 && w[1] == w[2] - 1)
        .any(|f| f)
    {
        return false;
    }

    if password.contains(&b'i') || password.contains(&b'o') || password.contains(&b'l') {
        return false;
    }

    let mut seen = HashSet::new();
    password.windows(2).for_each(|w| {
        if w[0] == w[1] {
            seen.insert(w[0]);
        }
    });
    seen.len() >= 2
}

fn find_next_valid_password(password: &str) -> String {
    let mut cur = password.chars().map(|c| c as u8).collect();
    loop {
        cur = increment(&cur);
        if valid(&cur) {
            break;
        }
    }

    cur.iter().map(|i| *i as char).collect()
}

pub fn p1(input: &str) -> anyhow::Result<String> {
    Ok(find_next_valid_password(input.trim()))
}

pub fn p2(input: &str) -> anyhow::Result<String> {
    Ok(find_next_valid_password(input.trim()))
}

use std::{collections::HashSet, time::Instant};

pub fn solve() -> anyhow::Result<()> {
    let now = Instant::now();
    let password = p1(INPUT)?;
    println!("p1: {} ({:?})", password, now.elapsed());
    let now = Instant::now();
    println!("p2: {} ({:?})", p2(&password)?, now.elapsed());
    Ok(())
}
