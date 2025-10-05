const INPUT: &'static str = include_str!("inputs/day10.txt");

fn look_and_say_len(input: &str, n: usize) -> usize {
    let mut cur = input
        .trim()
        .bytes()
        .map(|b| b - b'1' + 1) // Turn the char into a u8 (integer)
        .collect::<Vec<u8>>();
    for _ in 0..n {
        let mut next = Vec::new();
        let mut pos = 0;
        let mut end = 0;

        while pos < cur.len() {
            while end < cur.len() && cur[pos] == cur[end] {
                end += 1;
            }
            next.push((end - pos) as u8);
            next.push(cur[pos]);
            pos = end;
            end = pos;
        }

        cur = next;
    }
    cur.len()
}

pub fn p1(input: &str) -> usize {
    look_and_say_len(input, 40)
}

pub fn p2(input: &str) -> usize {
    look_and_say_len(input, 50)
}

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
