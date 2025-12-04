use std::time::Instant;

const INPUT: &str = include_str!("inputs/[NAME].txt");

fn parse(input: &str) -> impl Iterator<Item = &str> {
    // TODO did you trim today?
    input.trim().lines()
}

fn p1(input: &str) -> usize {
    let input = parse(input);
    input.count()
}

fn p2(input: &str) -> usize {
    let input = parse(input);
    input.count()
}

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
        let input = "123\n456\n789\n";
        assert_eq!(p1(input), 3);
    }

    #[test]
    fn test_p2() {
        let input = "123\n456\n789\n";
        assert_eq!(p2(input), 3);
    }
}
