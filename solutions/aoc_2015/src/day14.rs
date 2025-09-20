use std::time::Instant;

const INPUT: &'static str = include_str!("inputs/day14.txt");

pub fn p1(input: &str) -> i32 {
    0
}

pub fn p2(input: &str) -> i32 {
    0
}


pub fn solve() -> anyhow::Result<()> {
    let now = Instant::now();
    println!("p1: {} ({:?})", p1(INPUT), now.elapsed());
    let now = Instant::now();
    println!("p2: {} ({:?})", p2(INPUT), now.elapsed());
    Ok(())
}
