use std::{error::Error, ops::Add, time::Instant};

use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use rustc_hash::FxHashMap;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add for &Point {
    type Output = Point;
    fn add(self, rhs: &Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

const INPUT: &str = include_str!("inputs/day24.txt");

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Input<'a> = (Point, Vec<Point>, FxHashMap<Point, char>);

fn parse_input(input: &'_ str) -> Result<Input<'_>> {
    let mut locations = Vec::new();
    let mut zero = Point::new(0, 0);
    let grid = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c != '.' && c != '#' {
                        if c == '0' {
                            zero = Point::new(x as isize, y as isize);
                        }
                        locations.push(Point::new(x as isize, y as isize));
                        (Point::new(x as isize, y as isize), '.')
                    } else {
                        (Point::new(x as isize, y as isize), c)
                    }
                })
                .collect::<Vec<(Point, char)>>()
        })
        .collect();
    Ok((zero, locations, grid))
}

fn p1((zero, locations, grid): &Input) -> Result<usize> {
    // Calculate the distances from each location to every other locations.
    let locations = locations.iter().sorted().cloned().collect::<Vec<Point>>();
    let deltas = [
        Point::new(0, 1),
        Point::new(0, -1),
        Point::new(1, 0),
        Point::new(-1, 0),
    ];

    // Use dijkstra (bfs) and make a map of (p1, p2) -> dist
    let neighbors = |p: &Point| {
        deltas
            .iter()
            .map(|d| p + d)
            .filter(|p| grid[p] == '.')
            .map(|p| (p, 1))
            .collect::<Vec<(Point, usize)>>()
    };
    let dists = locations
        .iter()
        .cartesian_product(locations.iter())
        .flat_map(|(l, r)| {
            let (_, dist) = dijkstra(l, neighbors, |p| p == r).unwrap();
            vec![((*l, *r), dist), ((*r, *l), dist)]
        })
        .collect::<FxHashMap<(Point, Point), usize>>();

    // Go through all the permutations and find the smallest.
    Ok(locations
        .iter()
        .permutations(locations.len())
        .filter(|p| p[0] == zero)
        .map(|p| {
            p.iter()
                .tuple_windows()
                .map(|(l, r)| dists[&(**l, **r)])
                .sum()
        })
        .min()
        .unwrap())
}

fn p2((zero, locations, grid): &Input) -> Result<usize> {
    let locations = locations.iter().sorted().cloned().collect::<Vec<Point>>();
    let deltas = [
        Point::new(0, 1),
        Point::new(0, -1),
        Point::new(1, 0),
        Point::new(-1, 0),
    ];
    let neighbors = |p: &Point| {
        deltas
            .iter()
            .map(|d| p + d)
            .filter(|p| grid[p] == '.')
            .map(|p| (p, 1))
            .collect::<Vec<(Point, usize)>>()
    };
    let dists = locations
        .iter()
        .cartesian_product(locations.iter())
        .flat_map(|(l, r)| {
            let (_, dist) = dijkstra(l, neighbors, |p| p == r).unwrap();
            vec![((*l, *r), dist), ((*r, *l), dist)]
        })
        .collect::<FxHashMap<(Point, Point), usize>>();
    let min = locations
        .iter()
        .permutations(locations.len())
        .filter(|p| p[0] == zero)
        .map(|p| {
            p.iter()
                .tuple_windows()
                .map(|(l, r)| dists[&(**l, **r)])
                .sum::<usize>()
                + dists[&(*p[p.len() - 1], *zero)]
        })
        .min()
        .unwrap();
    Ok(min)
}

fn main() -> Result<()> {
    let now = Instant::now();
    let input = parse_input(INPUT)?;
    let solution = p1(&input)?;
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(&input)?;
    println!("p2 {:?} {}", now.elapsed(), solution);

    Ok(())
}
