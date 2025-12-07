use std::time::Instant;

use rustc_hash::{FxHashMap, FxHashSet};

const INPUT: &[u8] = include_bytes!("inputs/day07.txt");
const INPUT_STR: &str = include_str!("inputs/day07.txt");

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn p1(input: &str) -> usize {
    let grid = parse(input);

    // Track our beams as a set.
    let mut beams = FxHashSet::default();
    beams.insert(grid[0].iter().position(|&c| c == 'S').unwrap());

    let mut splits = 0;
    for row in &grid {
        // For each row, find all the splitters.
        for (i, _) in row.iter().enumerate().filter(|(_, c)| **c == '^') {
            // If a beam is going to hit the splitter, update out count, add the splits and remove
            // ourself.
            if beams.contains(&i) {
                splits += 1;
                beams.insert(i + 1);
                beams.insert(i - 1);
                beams.remove(&i);
            }
        }
    }

    splits
}

fn p2(input: &str) -> usize {
    let grid = parse(input);

    // Track our timelines now in a map as they'll increase with each split.
    let mut beams = FxHashMap::default();
    beams.insert(grid[0].iter().position(|&c| c == 'S').unwrap(), 1);

    for row in &grid[1..] {
        row.iter()
            .enumerate()
            .filter(|(_, c)| **c == '^')
            .for_each(|(i, _)| {
                // For each splitter that's a part of a beam, we want to increment the timelines for each of the neighbors.
                if let Some(count) = beams.remove(&i) {
                    *beams.entry(i + 1).or_insert(0) += count;
                    *beams.entry(i - 1).or_insert(0) += count;
                }
            });
    }

    // Now it's just the sum of all the timelines in the map.
    beams.values().sum()
}

fn p1_fast(input: &[u8]) -> usize {
    // Get the stride and lines of the grid.
    let stride = input.iter().position(|b| *b == b'\n').unwrap() + 1;
    let lines = input.len().div_ceil(stride);

    // Track our beams as a bitmask.
    let mut beams = 0u128;
    beams |= 1u128 << input.iter().position(|b| *b == b'S').unwrap();

    // Go through each of the lines and find the beams that overlap with splitters.
    (0..lines)
        .map(|index| &input[stride * index..stride * index + stride])
        .map(|line| {
            let mut splits = 0;
            beams = line
                .iter()
                .enumerate()
                .filter(|(i, v)| **v == b'^' && beams & (1u128 << i) != 0)
                .fold(beams, |mut acc, (i, _)| {
                    // If we found a beam and splitter that overlap, increment our splits and
                    // update our mask.
                    splits += 1;
                    acc &= !(1u128 << i);
                    acc |= 1u128 << (i + 1);
                    acc |= 1u128 << (i - 1);
                    acc
                });
            splits
        })
        .sum()
}

fn p2_fast(input: &[u8]) -> usize {
    let stride = input.iter().position(|b| *b == b'\n').unwrap() + 1;
    let lines = input.len().div_ceil(stride);

    // For p2, we want to track the timelines. We'll only ever get one set of timelines per column, so we can use an array;
    let mut timelines = [0usize; 142]; // magic number, lol
    timelines[input.iter().position(|b| *b == b'S').unwrap()] = 1;

    for index in 0..lines {
        // Update our timelines when we encounter a splitter.
        let mut next_beams = timelines;
        for (i, _) in input[stride * index..stride * index + stride]
            .iter()
            .enumerate()
            .filter(|(i, v)| **v == b'^' && timelines[*i] != 0)
        {
            let count = timelines[i];
            next_beams[i] = 0;
            next_beams[i + 1] += count;
            next_beams[i - 1] += count;
        }
        timelines = next_beams;
    }

    timelines.iter().sum()
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT_STR);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT_STR);
    println!("p2 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p1_fast(INPUT);
    println!("p1_fast {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2_fast(INPUT);
    println!("p2_fast {:?} {}", now.elapsed(), solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............\n";
    const INPUT_BYTES: &[u8] = INPUT.as_bytes();

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 21);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 40);
    }

    #[test]
    fn test_p1_fast() {
        assert_eq!(p1_fast(INPUT_BYTES), 21);
    }

    #[test]
    fn test_p2_fast() {
        assert_eq!(p2_fast(INPUT_BYTES), 40);
    }
}
