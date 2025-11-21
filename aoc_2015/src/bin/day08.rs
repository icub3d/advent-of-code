const INPUT: &str = include_str!("inputs/day08.txt");

fn p1(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    lines.iter().map(|l| l.len() - in_memory_size(l)).sum()
}

fn in_memory_size(s: &str) -> usize {
    let mut in_slash = false;
    s.chars()
        .map(|c| match (in_slash, c) {
            (false, '\\') => {
                in_slash = true;
                0
            }
            (true, '\\') | (true, '"') => {
                in_slash = false;
                1
            }
            (true, 'x') => {
                in_slash = false;
                -1
            }
            _ => 1,
        })
        .sum::<isize>() as usize
        - 2
}

fn p2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    lines.iter().map(|l| encoded_size(l) - l.len()).sum()
}

fn encoded_size(s: &str) -> usize {
    2 + s
        .chars()
        .map(|c| match c {
            '\\' | '"' => 2,
            _ => 1,
        })
        .sum::<usize>()
}

use std::time::Instant;

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(in_memory_size("\"\""), 0);
        assert_eq!(in_memory_size("\"abc\""), 3);
        assert_eq!(in_memory_size("\"aaa\\\"aaa\""), 7);
        assert_eq!(in_memory_size("\"\\x27\""), 1);
    }
}
