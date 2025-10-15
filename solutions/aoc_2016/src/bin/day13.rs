use std::{collections::VecDeque, time::Instant};

use rustc_hash::FxHashSet;

const INPUT: &str = include_str!("inputs/day13.txt");

type Input<'a> = isize;

fn parse_input(input: &'_ str) -> Input<'_> {
    input.trim().parse().unwrap()
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn wall(&self, favorite: isize) -> bool {
        let Point { x, y } = self;
        let n = x * x + 3 * x + 2 * x * y + y + y * y + favorite;
        !n.count_ones().is_multiple_of(2)
    }

    fn neighbors(&self, favorite: isize) -> Vec<Self> {
        [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|(dx, dy)| Point::new(self.x + dx, self.y + dy))
            .filter(|Point { x, y }| *x >= 0 && *y >= 0)
            .filter(|p| !p.wall(favorite))
            .collect()
    }
}

fn p1(input: &Input) -> usize {
    let start = Point::new(1, 1);
    let end = Point::new(31, 39);

    let mut visited: FxHashSet<Point> = FxHashSet::default();
    let mut frontier: VecDeque<(Point, usize)> = VecDeque::new();

    visited.insert(start.clone());
    frontier.push_back((start.clone(), 0));

    while let Some((point, steps)) = frontier.pop_front() {
        if point == end {
            return steps;
        }

        for neighbor in point.neighbors(*input) {
            if visited.insert(neighbor.clone()) {
                frontier.push_back((neighbor.clone(), steps + 1));
            }
        }
    }

    panic!("not found")
}

fn p2(input: &Input) -> usize {
    let start = Point::new(1, 1);

    let mut visited: FxHashSet<Point> = FxHashSet::default();
    let mut frontier: VecDeque<(Point, usize)> = VecDeque::new();

    visited.insert(start.clone());
    frontier.push_back((start.clone(), 0));

    while let Some((point, steps)) = frontier.pop_front() {
        // This has the effect of terminating all paths that are greater than 50.
        if steps == 50 {
            continue;
        }

        for neighbor in point.neighbors(*input) {
            if visited.insert(neighbor.clone()) {
                frontier.push_back((neighbor.clone(), steps + 1));
            }
        }
    }

    visited.len()
}

fn main() {
    let now = Instant::now();
    let input = parse_input(INPUT);
    let solution = p1(&input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(&input);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
