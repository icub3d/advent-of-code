use std::time::Instant;

const INPUT: &str = include_str!("inputs/day05.txt");

fn p1_rules(line: &str) -> bool {
    let mut chars = line.chars();
    let first = chars.next().unwrap();
    let vowels = if "aeiou".contains(first) { 1 } else { 0 };
    let (vowels, has_double, has_bad, _) = chars.fold(
        (vowels, false, false, first),
        |(vowels, has_double, has_bad, prev), c| {
            (
                vowels + if "aeiou".contains(c) { 1 } else { 0 },
                has_double || prev == c,
                has_bad || matches!((prev, c), ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')),
                c,
            )
        },
    );
    vowels >= 3 && has_double && !has_bad
}

fn p1(input: &str) -> usize {
    input.lines().filter(|line| p1_rules(line)).count()
}

fn p2_rules(line: &str) -> bool {
    // We can track seen pairs by checking the last two first and then inserting the second pair. Note, Vec is faster than HashSet here, even using FxHashSet.
    let mut seen = Vec::new();
    line.as_bytes()
        .windows(3)
        .scan((false, false), |(has_pair, has_repeat), w| {
            *has_repeat |= w[0] == w[2];
            *has_pair |= seen.contains(&[w[1], w[2]]);
            seen.push([w[0], w[1]]);
            Some((*has_pair, *has_repeat))
        })
        .any(|(has_pair, has_repeat)| has_pair && has_repeat)
}

fn p2(input: &str) -> usize {
    input.lines().filter(|line| p2_rules(line)).count()
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
        let tests = "ugknbfddgicrmopn\naaa\njchzalrnumimnmhp\nhaegwjzuvuyypxyu\ndvszwmarrgswjxmb\n";
        assert_eq!(p1(tests), 2);
    }

    #[test]
    fn test_p2() {
        let tests = "qjhvhtzxzqqjkmpb\nxxyxx\nuurcxstgmygtbstg\nieodomkazucvgmuy\n";
        assert_eq!(p2(tests), 2);
    }
}
