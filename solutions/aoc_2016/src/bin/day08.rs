use std::time::Instant;

const INPUT: &str = include_str!("inputs/day08.txt");

type Input<'a> = Vec<&'a str>;

fn parse_input(input: &'_ str) -> Input<'_> {
    input.lines().collect()
}

fn p1(_input: &Input) -> usize {
    0
}

fn p2(_input: &Input) -> usize {
    0
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
