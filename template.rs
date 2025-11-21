use std::{error::Error, time::Instant};

const INPUT: &str = include_str!("inputs/[NAME].txt");

type Int = isize;
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Input<'a> = Vec<&'a str>;

fn parse_input(input: &'_ str) -> Result<Input<'_>> {
    // TODO did you trim today?
    Ok(input.trim().lines().collect())
}

fn p1(_input: &Input) -> Result<usize> {
    Ok(0)
}

fn p2(_input: &Input) -> Result<usize> {
    Ok(0)
}

fn main() -> Result<()> {
    let now = Instant::now();
    let input = parse_input(INPUT)?;
    let solution = p1(&input)?;
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(&input)?;
    println!("p2 {:?} {}", now.elapsed(), solution);

    Ok(())
}
