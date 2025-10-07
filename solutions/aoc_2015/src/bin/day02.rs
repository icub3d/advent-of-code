const INPUT: &str = include_str!("inputs/day02.txt");

// A helper function to parse dimensions from a line like "2x3x4"
fn parse_dims(line: &str) -> (usize, usize, usize) {
    let mut parts = line.split('x').map(|n| n.parse::<usize>().unwrap());
    let a = parts.next().unwrap();
    let b = parts.next().unwrap();
    let c = parts.next().unwrap();
    (a, b, c)
}

fn p1(input: &str) -> usize {
    input
        .lines()
        .map(parse_dims)
        // Convert (l, w, h) to the areas of the sides
        .map(|(l, w, h)| (l * w, w * h, l * h))
        // Calculate the total wrapping paper needed
        .map(|(x, y, z)| 2 * (x + y + z) + x.min(y).min(z))
        .sum()
}

fn p2(input: &str) -> usize {
    input
        .lines()
        .map(parse_dims)
        .map(|(l, w, h)| {
            // Calculate the volume and the smallest perimeter. The smallest perimeter is found by summing all the perimeters and subtracting the longest side and then doubling the result.
            let volume = l * w * h;
            let perimeter = 2 * (l + w + h - l.max(w).max(h));
            volume + perimeter
        })
        .sum()
}

fn main() {
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
    fn test_p1() {
        let input = "2x3x4\n1x1x10\n";
        assert_eq!(p1(input), 101);
    }

    #[test]
    fn test_p2() {
        let input = "2x3x4\n1x1x10\n";
        assert_eq!(p2(input), 48);
    }
}
