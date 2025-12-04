use std::time::Instant;

const INPUT: &str = include_str!("inputs/day03.txt");

fn parse(input: &str) -> impl Iterator<Item = Vec<usize>> {
    // Turn each line into a list of digits.
    input.lines().map(|l| {
        l.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    })
}

// Use two pointers (left, right) where left is the largest digit we've seen and right is sliding through rest of bank. Update our left if we ever find a larger number.
fn two_pointer(bank: &[usize]) -> usize {
    bank[1..]
        .iter()
        .scan(bank[0], |left, &right| {
            // Get the value we will emit before we update left.
            let cur = *left * 10 + right;

            // If right is bigger, we found a new left.
            if right > *left {
                *left = right;
            }

            Some(cur)
        })
        .max()
        .unwrap()

    // let mut left = 0;
    // let mut right = 1;
    // let mut largest = 0;
    // while right < bank.len() {
    //     largest = largest.max(bank[left] * 10 + bank[right]);
    //     if bank[right] > bank[left] {
    //         left = right;
    //     }
    //     right += 1;
    // }
    // largest
}

fn p1(input: &str) -> usize {
    parse(input).map(|bank| two_pointer(&bank)).sum()
}

fn greedy(bank: &[usize], remaining: usize, total: usize) -> usize {
    // If we've reached the end, we are done.
    if remaining == 0 {
        return total;
    }

    // Find the *first* largest value we can use (max_by_key returns last, so rev).
    let (i, largest) = bank[..bank.len() - remaining + 1]
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|(_, v)| *v)
        .unwrap();

    // Call recursively for the rest of the bank.
    greedy(
        &bank[i + 1..],
        remaining - 1,
        total + largest * 10usize.pow(remaining as u32 - 1),
    )
}

fn p2(input: &str) -> usize {
    parse(input).map(|bank| greedy(&bank, 12, 0)).sum()
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
    use indoc::indoc;

    #[test]
    fn test_p1() {
        let input = indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        "};
        assert_eq!(p1(input), 357);
    }

    #[test]
    fn test_p2() {
        let input = indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        "};
        assert_eq!(p2(input), 3121910778619);
    }
}
