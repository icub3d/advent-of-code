use std::time::Instant;

const INPUT: &'static str = include_str!("inputs/day25.txt");

pub fn p1(input: &str) -> usize {
    let parts = input.split_whitespace().collect::<Vec<&str>>();
    let row = parts[15].trim_end_matches(',').parse::<usize>().unwrap();
    let col = parts[17].trim_end_matches('.').parse::<usize>().unwrap();

    let mut r = 1;
    let mut c = 1;
    let mut code: usize = 20151125;
    while r != row || c != col {
        if r == 1 {
            r = c + 1;
            c = 1
        } else {
            c += 1;
            r -= 1;
        }

        code *= 252533;
        code %= 33554393;
    }
    code
}

pub fn p2(_input: &str) -> i32 {
    0
}

pub fn solve() -> anyhow::Result<()> {
    let now = Instant::now();
    println!("p1: {} ({:?})", p1(INPUT), now.elapsed());
    let now = Instant::now();
    println!("p2: {} ({:?})", p2(INPUT), now.elapsed());
    Ok(())
}
