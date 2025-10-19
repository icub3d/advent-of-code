use std::{error::Error, time::Instant};

const INPUT: &str = include_str!("inputs/day03.txt");

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Input<'a> = Vec<(usize, usize, usize)>;

fn parse_input(input: &'_ str) -> Result<Input<'_>> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace().map(|c| c.parse::<usize>());
            Ok((
                parts.next().unwrap()?,
                parts.next().unwrap()?,
                parts.next().unwrap()?,
            ))
        })
        .collect()
}

fn valid(a: usize, b: usize, c: usize) -> usize {
    (a + b > c && a + c > b && b + c > a) as usize
}

fn p1(input: &Input) -> usize {
    input.iter().map(|&(a, b, c)| valid(a, b, c)).sum()
}

fn p2(input: &Input) -> usize {
    input
        .chunks(3)
        .map(|chunk| (chunk[0], chunk[1], chunk[2]))
        .map(|(a, b, c)| valid(a.0, b.0, c.0) + valid(a.1, b.1, c.1) + valid(a.2, b.2, c.2))
        .sum()
}

fn main() -> Result<()> {
    let now = Instant::now();
    let input = parse_input(INPUT)?;
    let solution = p1(&input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(&input);
    println!("p2 {:?} {}", now.elapsed(), solution);

    Ok(())
}
