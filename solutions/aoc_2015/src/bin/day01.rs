const INPUT: &'static str = include_str!("inputs/day01.txt");

pub fn p1(input: &str) -> i32 {
    let floor = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum::<i32>();
    floor
}

pub fn p2(input: &str) -> i32 {
    let mut floor = 0;
    let mut first_negative = 0;
    for (i, c) in input.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if floor < 0 {
            first_negative = i as i32 + 1;
            break;
        }
    }
    first_negative
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2() {
        assert_eq!(p2("()())"), 5);
    }
}

pub fn main() {
    let now = std::time::Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);
    let now = std::time::Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
