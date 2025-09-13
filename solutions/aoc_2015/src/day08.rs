const INPUT: &'static str = include_str!("inputs/day08.txt");

pub fn p1(input: &str) -> anyhow::Result<usize> {
    let lines = input.lines().collect::<Vec<&str>>();
    Ok(lines.iter().map(|l| l.len() - in_memory_size(l)).sum())
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

pub fn p2(input: &str) -> anyhow::Result<usize> {
    let lines = input.lines().collect::<Vec<&str>>();
    Ok(lines.iter().map(|l| encoded_size(l) - l.len()).sum())
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

pub fn solve() -> anyhow::Result<()> {
    let now = Instant::now();
    println!("p1: {} ({:?})", p1(INPUT)?, now.elapsed());
    let now = Instant::now();
    println!("p2: {} ({:?})", p2(INPUT)?, now.elapsed());
    Ok(())
}
