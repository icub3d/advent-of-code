use std::{ops::RangeInclusive, time::Instant};

const INPUT: &str = include_str!("inputs/day05.txt");

fn parse(input: &str) -> (Vec<RangeInclusive<usize>>, impl Iterator<Item = usize>) {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    // Collect our ranges.
    let ranges = ranges
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .map(|(l, r)| l.parse().unwrap()..=r.parse().unwrap())
        .collect();

    // Return our ranges and an iterator over our items.
    (ranges, ingredients.lines().map(|l| l.parse().unwrap()))
}

fn p1(input: &str) -> usize {
    let (ranges, ingredients) = parse(input);
    // Count all ingredients that are within any range.
    ingredients
        .filter(|ingredient| ranges.iter().any(|range| range.contains(ingredient)))
        .count()
}

fn p2(input: &str) -> usize {
    let (mut ranges, _) = parse(input);

    // Sort our ranges and then merge them.
    ranges.sort_unstable_by_key(|range| *range.start());
    let mut merged = Vec::with_capacity(ranges.len());
    let mut ranges = ranges.into_iter();
    merged.push(ranges.next().unwrap());
    for range in ranges {
        let last = merged.last_mut().unwrap();
        // If they don't overlap, we can simply add the new range to our merged list.
        if last.end() < range.start() {
            merged.push(range);
            continue;
        }

        // Otherwise, merge the ranges.
        *last = *last.start()..=*last.end().max(range.end());
    }

    // With the merged list we just get the size of each range and sum them.
    merged.iter().map(|r| r.end() - r.start() + 1).sum()
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

    const INPUT: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 3);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 14);
    }
}
