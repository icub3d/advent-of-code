use std::time::Instant;

use itertools::Itertools;
use rustc_hash::FxHashMap;

const INPUT: &str = include_str!("inputs/day08.txt");

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let mut pp = value.split(',').map(|v| v.parse::<f64>().unwrap());
        Self {
            x: pp.next().unwrap(),
            y: pp.next().unwrap(),
            z: pp.next().unwrap(),
        }
    }
}

impl Point {
    fn distance(&self, rhs: &Point) -> f64 {
        let dx = rhs.x - self.x;
        let dy = rhs.y - self.y;
        let dz = rhs.z - self.z;

        (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
    }
}

fn parse(input: &str) -> impl Iterator<Item = Point> {
    input.lines().map(Point::from)
}

struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
    // For p2, we'll want to track how many groups we have and stop when we have just one.
    count: usize,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            count: n,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }

        self.parent[i] = self.find(self.parent[i]);
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        // If they were already connected, we are done.
        if root_i == root_j {
            return false;
        }

        // Otherwise, we want to merge them and update our count.
        let (p1, p2) = match self.size[root_i] < self.size[root_j] {
            true => (root_i, root_j),
            false => (root_j, root_i),
        };
        self.parent[p1] = p2;
        self.size[p2] += self.size[p1];
        self.count -= 1;
        true
    }
}

// BUG(FIXED): For the example input, we only want to make 10 connections.
fn p1(input: &str, limit: usize) -> usize {
    // Get our points.
    let points = parse(input).collect::<Vec<_>>();

    // Calculate all distances.
    let dists = points
        .iter()
        .enumerate()
        .flat_map(|(i1, p1)| {
            points[i1 + 1..]
                .iter()
                .enumerate()
                // BUG(FIXED): i2 is in relation to i1+1.
                .map(move |(i2, p2)| ((i1, i1 + 1 + i2), p1.distance(p2)))
        })
        .sorted_unstable_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
        .collect::<Vec<((usize, usize), f64)>>();

    // Add limit pairs of them to our set.
    let mut ds = DisjointSet::new(points.len());
    for ((i1, i2), _) in dists.into_iter().take(limit) {
        ds.union(i1, i2);
    }

    // Make groupings of connected junction boxes.
    let mut connected: FxHashMap<usize, usize> = FxHashMap::default();
    for i in 0..points.len() {
        let root = ds.find(i);
        *connected.entry(root).or_default() += 1;
    }

    // Take the three largest and return their product.
    connected
        .values()
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(3)
        .product()
}

fn p2(input: &str) -> usize {
    let points = parse(input).collect::<Vec<_>>();
    let dists = points
        .iter()
        .enumerate()
        .flat_map(|(i1, p1)| {
            points[i1 + 1..]
                .iter()
                .enumerate()
                .map(move |(i2, p2)| ((i1, i1 + 1 + i2), p1.distance(p2)))
        })
        .sorted_unstable_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
        .collect::<Vec<((usize, usize), f64)>>();

    // Union values from the set until we perform a merge that makes a single set.
    let mut ds = DisjointSet::new(points.len());
    for ((i1, i2), _) in dists {
        if ds.union(i1, i2) && ds.count == 1 {
            // Return the product of their x values.
            return (points[i1].x * points[i2].x) as usize;
        }
    }

    unreachable!()
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT, 1000);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("inputs/day08-example.txt");

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT, 10), 40);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 25272);
    }
}
