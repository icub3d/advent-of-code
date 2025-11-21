use std::time::Instant;

const INPUT: &str = include_str!("inputs/day20.txt");

fn p1(input: &str) -> usize {
    let input = input.trim().parse::<usize>().unwrap();
    let mut presents = vec![0; 1_000_000];
    for elf in 1..presents.len() {
        for house in (elf..presents.len()).step_by(elf) {
            presents[house] += 10 * elf;
        }
        if presents[elf] >= input {
            return elf;
        }
    }
    0
}

fn p2(input: &str) -> usize {
    let input = input.trim().parse::<usize>().unwrap();
    let mut presents = vec![0; 1_000_000];
    for elf in 1..presents.len() {
        for house in (elf..presents.len()).step_by(elf).take(50) {
            presents[house] += 11 * elf;
        }
        if presents[elf] >= input {
            return elf;
        }
    }
    0
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
