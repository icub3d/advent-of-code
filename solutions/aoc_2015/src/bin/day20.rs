use std::time::Instant;

const INPUT: &'static str = include_str!("inputs/day20.txt");

fn find_elves(n: usize) -> Vec<usize> {
    let mut elves = Vec::new();
    let limit = (n as f64).sqrt() as usize;

    for i in 1..=limit {
        if n % i == 0 {
            elves.push(i);
            if i * i != n {
                // Avoid adding the same divisor twice for perfect squares
                elves.push(n / i);
            }
        }
    }

    elves
}

pub fn p1(input: &str) -> usize {
    let input = input.trim().parse::<usize>().unwrap();
    (1_usize..)
        .find(|i| find_elves(*i).iter().sum::<usize>() * 10 >= input)
        .unwrap()
}

pub fn p2(input: &str) -> usize {
    let input = input.trim().parse::<usize>().unwrap();
    (1_usize..)
        .find(|i| {
            find_elves(*i)
                .iter()
                .filter(|n| i / **n <= 50)
                .map(|n| *n * 11)
                .sum::<usize>()
                >= input
        })
        .unwrap()
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
