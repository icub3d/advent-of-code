use std::time::Instant;

const INPUT: &str = include_str!("inputs/day25.txt");

pub fn p1(input: &str) -> u64 {
    let parts = input.split_whitespace().collect::<Vec<&str>>();
    let row = parts[15].trim_end_matches(',').parse::<u64>().unwrap();
    let col = parts[17].trim_end_matches('.').parse::<u64>().unwrap();

    // We can derive the total number of steps by finding the last full diagonal
    // before our target and then using n(n+1)/2 to find the sum of all previous
    // diagonals. We then add the last partial diagonal to get the steps.
    let last_full_diagonal = row + col - 2;
    let sum_previous_diagonals = last_full_diagonal * (last_full_diagonal + 1) / 2;
    let steps = sum_previous_diagonals + col - 1;

    let start = 20151125;
    let multiplier = 252533;
    let modulus = 33554393;

    let pow = modular_exponentiation(multiplier, steps, modulus);
    (start * pow) % modulus
}

fn modular_exponentiation(base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut b = base;

    while exponent > 0 {
        // If the current bit of the exponent is 1 (i.e., exponent is odd)
        if exponent % 2 == 1 {
            result = (result * b) % modulus;
        }

        // Square the base and take the modulus
        b = (b * b) % modulus;

        // Move to the next bit (integer division by 2)
        exponent /= 2;
    }
    result
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);
}
