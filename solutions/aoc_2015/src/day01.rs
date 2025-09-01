const INPUT: &'static str = include_str!("inputs/day01.txt");

pub fn p1(input: &str) -> anyhow::Result<i32> {
    let floor = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum::<i32>();
    Ok(floor)
}

pub fn p2(input: &str) -> anyhow::Result<i32> {
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
    Ok(first_negative)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2() {
        assert_eq!(p2("()())").unwrap(), 5);
    }
}

pub fn solve() -> anyhow::Result<()> {
    println!("p1: {}", p1(INPUT)?);
    println!("p2: {}", p2(INPUT)?);
    Ok(())
}
