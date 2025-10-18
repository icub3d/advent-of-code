use std::time::Instant;

const INPUT: &str = include_str!("inputs/day19.txt");

type Input<'a> = usize;

fn parse_input(input: &'_ str) -> Input<'_> {
    input.trim().parse().unwrap()
}

fn p1(input: Input) -> usize {
    let highest_ones_bit = usize::BITS as usize - input.leading_zeros() as usize - 1;
    let l = input - (1 << highest_ones_bit);
    2 * l + 1
}

fn p2(input: Input) -> usize {
    let mut k = 1;
    while k * 3 <= input {
        k *= 3;
    }

    if k == input {
        input
    } else if input - k <= k {
        input - k
    } else {
        2 * input - 3 * k
    }
}

fn main() {
    let now = Instant::now();
    let input = parse_input(INPUT);
    let solution = p1(input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(input);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
