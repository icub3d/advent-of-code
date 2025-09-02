const INPUT: &'static str = include_str!("inputs/day02.txt");

pub fn p1(input: &str) -> anyhow::Result<usize> {
    let total = input
        .lines()
        .map(|l| {
            let parts: Vec<usize> = l.split('x').map(|n| n.parse::<usize>().unwrap()).collect();
            let ab = parts[0] * parts[1];
            let bc = parts[1] * parts[2];
            let ac = parts[0] * parts[2];
            let min = ab.min(bc).min(ac);
            2 * (ab + bc + ac) + min
        })
        .sum::<usize>();
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "2x3x4\n1x1x10\n";
        assert_eq!(p1(input).unwrap(), 101);
    }

    #[test]
    fn test_p2() {
        let input = "2x3x4\n1x1x10\n";
        assert_eq!(p2(input).unwrap(), 48);
    }
}

pub fn p2(input: &str) -> anyhow::Result<usize> {
    let total = input
        .lines()
        .map(|l| {
            let mut parts: Vec<usize> = l.split('x').map(|n| n.parse::<usize>().unwrap()).collect();
            let v = parts[0] * parts[1] * parts[2];
            parts.sort();
            v + 2 * (parts[0] + parts[1])
        })
        .sum::<usize>();
    Ok(total)
}

pub fn solve() -> anyhow::Result<()> {
    println!("p1: {}", p1(INPUT)?);
    println!("p2: {}", p2(INPUT)?);
    Ok(())
}
