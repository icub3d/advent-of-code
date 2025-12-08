use std::time::Instant;

use itertools::Itertools;
use rayon::prelude::*;

const INPUT: &str = include_str!("inputs/day08.txt");

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

// Parse Point from string.
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
    // Calculate distance
    // https://en.wikipedia.org/wiki/Euclidean_distance#Higher_dimensions
    fn distance(&self, rhs: &Point) -> f64 {
        let dx = rhs.x - self.x;
        let dy = rhs.y - self.y;
        let dz = rhs.z - self.z;

        // The right distance includes sqrt(), we can (very slightly) optimize since we are only
        // interested in order.
        // (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
        dx.powi(2) + dy.powi(2) + dz.powi(2)
    }
}

fn parse(input: &str) -> impl Iterator<Item = Point> {
    input.lines().map(Point::from)
}

// https://en.wikipedia.org/wiki/Disjoint_sets
struct DisjointSet {
    parent: Vec<usize>,
    // Track the size of each set (only parent is accurate for entire group).
    size: Vec<usize>,
    // Track the number of sets.
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

    // For p3, we are interested in the "last merge", so we return true if we made a group and then p3 can check if that merge was the last.
    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i == root_j {
            return false;
        }

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

// BUG(FIXED): the test case sets a limit of 10, which was giving me the wrong answer.
fn p1(input: &str, limit: usize) -> usize {
    let points = parse(input).collect::<Vec<_>>();

    let mut dists = points
        .iter()
        .enumerate()
        .flat_map(|(i1, p1)| {
            points[i1 + 1..]
                .iter()
                .enumerate()
                // BUG(FIXED): i2 is from i1+1
                .map(move |(i2, p2)| ((i1, i1 + 1 + i2), p1.distance(p2)))
        })
        .collect::<Vec<((usize, usize), f64)>>();

    // We don't have to completely sort, just make sure 0..limit are the smallest.
    // "kth element" and then truncate (very minor optimization)
    // https://en.wikipedia.org/wiki/Quickselect
    // [ipnsort]: https://github.com/Voultapher/sort-research-rs/tree/main/ipnsort
    // [total order]: https://en.wikipedia.org/wiki/Total_order
    dists.select_nth_unstable_by(limit, |(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());
    dists.truncate(limit);

    // Add unions to our disjoint set.
    let mut ds = DisjointSet::new(points.len());
    for ((i1, i2), _) in dists {
        ds.union(i1, i2);
    }

    // We know the sizes of each group, so we can simply find the parents and sort by their size and take the product of the largest 3.
    ds.parent
        .iter()
        .enumerate()
        .filter(|(i, p)| i == *p)
        .map(|(i, _)| ds.size[i])
        .sorted_unstable_by(|a, b| b.cmp(a)) // BUG(FIXED): largest
        .take(3)
        .product()
}

// p2 is largely the same as p1, we just track when all of the values have been sorted.
fn p2(input: &str) -> usize {
    let points = parse(input).collect::<Vec<_>>();

    let mut dists = points
        .iter()
        .enumerate()
        .flat_map(|(i1, p1)| {
            points[i1 + 1..]
                .iter()
                .enumerate()
                .map(move |(i2, p2)| ((i1, i1 + 1 + i2), p1.distance(p2)))
        })
        .collect::<Vec<((usize, usize), f64)>>();

    // Sorting in parallel actually helps here.
    dists.par_sort_unstable_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());

    let mut ds = DisjointSet::new(points.len());
    for ((i1, i2), _) in dists {
        // Go until we find the last merge.
        if ds.union(i1, i2) && ds.count == 1 {
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
