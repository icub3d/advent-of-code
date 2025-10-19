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
    let distances = locations
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
                .map(|(l, r)| distances[&(**l, **r)])
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
    let distances = locations
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
                .map(|(l, r)| distances[&(**l, **r)])
                .sum::<usize>()
                + distances[&(*p[p.len() - 1], *zero)]
        })
        .min()
        .unwrap();
    Ok(min)
}

fn p2_held_karp((zero, locations, grid): &Input) -> Result<usize> {
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

    // Build distance matrix
    let distances = locations
        .iter()
        .cartesian_product(locations.iter())
        .flat_map(|(l, r)| {
            let (_, dist) = dijkstra(l, neighbors, |p| p == r).unwrap();
            vec![((*l, *r), dist), ((*r, *l), dist)]
        })
        .collect::<FxHashMap<(Point, Point), usize>>();

    // Find the index of zero
    let start = locations.iter().position(|p| p == zero).unwrap();
    let n = locations.len();

    // dp[(mask, i)] = minimum distance to visit all nodes in mask, ending at node i
    // mask is a bitmask where bit j is set if node j has been visited
    let mut dp = FxHashMap::default();

    // Base case: start at zero
    dp.insert((1 << start, start), 0);

    // Iterate through all possible subsets
    for mask in 0..(1 << n) {
        for last in 0..n {
            // If last is not in the mask, skip
            if (mask & (1 << last)) == 0 {
                continue;
            }

            if let Some(&current_dist) = dp.get(&(mask, last)) {
                // Try extending to each unvisited node
                for next in 0..n {
                    if (mask & (1 << next)) != 0 {
                        continue;
                    }

                    let new_mask = mask | (1 << next);
                    let new_dist = current_dist + distances[&(locations[last], locations[next])];

                    dp.entry((new_mask, next))
                        .and_modify(|d| *d = (*d).min(new_dist))
                        .or_insert(new_dist);
                }
            }
        }
    }

    // Find minimum distance to visit all nodes and return to start
    let all_visited = (1 << n) - 1;
    let min = (0..n)
        .filter(|&i| i != start)
        .filter_map(|i| {
            dp.get(&(all_visited, i))
                .map(|&dist| dist + distances[&(locations[i], locations[start])])
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

    let now = Instant::now();
    let solution = p2_held_karp(&input)?;
    println!("p2-held-karp {:?} {}", now.elapsed(), solution);

    Ok(())
}
