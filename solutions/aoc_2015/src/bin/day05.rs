use std::time::Instant;

const INPUT: &'static str = include_str!("inputs/day05.txt");

pub fn p1(input: &str) -> i32 {
    let mut p1 = 0;
    for line in input.lines().map(|l| l.chars().collect::<Vec<char>>()) {
        // Check for vowels
        let vowels = line
            .clone()
            .into_iter()
            .filter(|c| "aeiou".contains(*c))
            .count();
        if vowels < 3 {
            continue;
        }

        // Check for bad pairs
        let bad = [['a', 'b'], ['c', 'd'], ['p', 'q'], ['x', 'y']];
        let found = line.windows(2).any(|w| bad.contains(&[w[0], w[1]]));
        if found {
            continue;
        }

        // Check for two of same adjacent
        let found = line.windows(2).any(|w| w[0] == w[1]);
        if found {
            p1 += 1;
        }
    }
    p1
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

pub fn p2(input: &str) -> i32 {
    let mut p2 = 0;
    for line in input.lines().map(|l| l.chars().collect::<Vec<char>>()) {
        // Look for any repeats
        let repeat = line.windows(3).any(|w| w[0] == w[2]);
        if !repeat {
            continue;
        }

        let windows = line.windows(2).collect::<Vec<&[char]>>();
        for (i, window) in windows.iter().enumerate() {
            if i < windows.len() - 2 && windows[i + 2..].contains(window) {
                p2 += 1;
                break;
            }
        }

        // let mut double = false;
        // 'outer: for i in 0..line.len() - 3 {
        //     let search = (line[i], line[i + 1]);
        //     for j in i + 2..line.len() - 1 {
        //         if search == (line[j], line[j + 1]) {
        //             double = true;
        //             break 'outer;
        //         }
        //     }
        // }

        // if double {
        //     p2 += 1;
        // }
    }
    p2
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
