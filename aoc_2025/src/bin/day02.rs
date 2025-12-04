use rayon::prelude::*;
use std::{ops::RangeInclusive, time::Instant};

const INPUT: &str = include_str!("inputs/day02.txt");

fn parse(input: &str) -> impl Iterator<Item = RangeInclusive<usize>> {
    // Convert the input into a list of ranges.
    input.trim().split(',').map(|l| {
        l.split_once('-')
            .map(|(l, r)| l.parse().unwrap()..=r.parse().unwrap())
            .unwrap()
    })
}

fn p1(input: &str) -> usize {
    parse(input)
        .par_bridge()
        .flatten()
        .filter(|&v| {
            // Convert to string and verify both halves are equal.
            let s = v.to_string();
            if !s.len().is_multiple_of(2) {
                return false;
            }
            let (l, r) = s.split_at(s.len() / 2);
            l == r
        })
        .sum()
}

fn p2(input: &str) -> usize {
    parse(input)
        .par_bridge()
        .flatten()
        .filter(|&v| {
            // Convert to string and check each length up to half.
            let s = v.to_string();
            for d in 1..=s.len() / 2 {
                // Must equally divide.
                if !s.len().is_multiple_of(d) {
                    continue;
                }
                // Get the prefix and see if repeating it is equal to the original.
                let prefix = &s[..d];
                if prefix.repeat(s.len() / d) == s {
                    return true;
                }
            }
            false
        })
        .sum()
}

fn p2_int(input: &str) -> usize {
    parse(input)
        .par_bridge()
        .flatten()
        .filter(|&v| {
            // Try all grouping lengths up to half.
            let digits = (v.ilog10() + 1) as usize;
            for d in 1..=digits / 2 {
                // Must equally divide.
                if !digits.is_multiple_of(d) {
                    continue;
                }

                // The prefix we are looking at for this iteration.
                let prefix = v / 10_usize.pow((digits - d) as u32);

                // Repeat it for the rest of the digits
                let repeated =
                    (1..digits / d).fold(prefix, |acc, _| acc * 10_usize.pow(d as u32) + prefix);

                // If the repeated value is the same as the original, it's a match.
                if repeated == v {
                    return true;
                }
            }
            false
        })
        .sum()
}

fn main() {
    // let n = parse(INPUT).flatten().count();
    // println!("n: {n}"); // 1_479_807

    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2_int(INPUT);
    println!("p2_int {:?} {}", now.elapsed(), solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 1227775554);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 4174379265);
    }

    #[test]
    fn test_p2_int() {
        assert_eq!(p2_int(INPUT), 4174379265);
    }
}
