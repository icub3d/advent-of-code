use std::time::Instant;

use itertools::Itertools;
use rustc_hash::FxHashSet;

const INPUT: &str = include_str!("inputs/day04.txt");

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    fn new(row: isize, col: isize) -> Self {
        Point { row, col }
    }

    // Get all potential neighbors of this point.
    fn potential_neighbors(&self) -> impl Iterator<Item = Self> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|&(r, c)| r != 0 || c != 0)
            .map(|(dr, dc)| Point::new(self.row + dr, self.col + dc))
    }
}

fn parse(input: &str) -> FxHashSet<Point> {
    // Turn the grid into a set of points that are barrels.
    input
        .lines()
        .enumerate()
        .flat_map(|(r, l)| l.bytes().enumerate().map(move |(c, v)| (r, c, v)))
        .filter_map(|(r, c, v)| {
            if v == b'@' {
                Some(Point::new(r as isize, c as isize))
            } else {
                None
            }
        })
        .collect()
}

fn p1(input: &str) -> usize {
    // grid is a set of points where there are rolls of paper.
    let grid = parse(input);

    // Get a count of all points that have fewer than four neighbors.
    grid.iter()
        .filter(|p| p.potential_neighbors().filter(|p| grid.contains(p)).count() < 4)
        .count()
}

fn p2(input: &str) -> usize {
    let mut grid = parse(input);
    let mut total = 0;

    // Start with the initial set of points to remove.
    let mut remove = grid
        .iter()
        .filter(|p| p.potential_neighbors().filter(|p| grid.contains(p)).count() < 4)
        .cloned()
        .collect::<FxHashSet<_>>();

    // Loop as long as we are removing points.
    while !remove.is_empty() {
        total += remove.len();

        // The only potential candidates for removal are those around our remove set.
        let candidates = remove
            .iter()
            .flat_map(|p| p.potential_neighbors())
            .filter(|p| grid.contains(p) && !remove.contains(p))
            .collect::<FxHashSet<_>>();

        // Remove from main grid (set difference (grid - remove) but faster).
        grid.retain(|p| !remove.contains(p));

        // From the candidates, find which ones to remove in the next round.
        remove = candidates
            .into_iter()
            .filter(|p| p.potential_neighbors().filter(|p| grid.contains(p)).count() < 4)
            .collect();
    }

    total
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

    const INPUT: &str = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 13);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 43);
    }
}
