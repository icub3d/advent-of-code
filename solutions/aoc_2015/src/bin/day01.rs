const INPUT: &str = include_str!("inputs/day01.txt");

// Convert each character to a floor change and then sum them up.
fn p1(input: &str) -> isize {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum()
}

// Use scan to keep track of the current floor and find the first position where it goes below 0.
fn p2(input: &str) -> usize {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .scan(0, |floor, change| {
            *floor += change;
            Some(*floor)
        })
        .position(|floor| floor < 0)
        .map(|pos| pos + 1)
        .unwrap()
}

pub fn main() {
    let now = std::time::Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);
    let now = std::time::Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2() {
        assert_eq!(p2("()())"), 5);
    }
}
