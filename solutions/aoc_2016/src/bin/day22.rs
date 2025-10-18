use std::{collections::VecDeque, error::Error, ops::Add, time::Instant};

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

const INPUT: &str = include_str!("inputs/day22.txt");

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Input<'a> = FxHashMap<Point, Node>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Node {
    used: usize,
    avail: usize,
}

impl Node {
    fn parse(input: &str) -> Result<(Point, Node)> {
        let parts = input.split_whitespace().collect::<Vec<_>>();
        let name_parts = parts[0].split('-').collect::<Vec<_>>();
        Ok((
            Point {
                x: name_parts[1][1..].parse()?,
                y: name_parts[2][1..].parse()?,
            },
            Self {
                used: parts[2].trim_end_matches('T').parse()?,
                avail: parts[3].trim_end_matches('T').parse()?,
            },
        ))
    }
}

fn parse_input(input: &'_ str) -> Result<Input<'_>> {
    input.trim().lines().skip(2).map(Node::parse).collect()
}

fn p1(input: &Input) -> Result<usize> {
    // count of AxB where criteria is met
    Ok(input
        .iter()
        .cartesian_product(input.iter())
        .filter(|((point_a, node_a), (point_b, node_b))| {
            node_a.used != 0 && point_a != point_b && node_a.used <= node_b.avail
        })
        .count())
}

fn p2(input: &Input) -> Result<usize> {
    let max_x = input.keys().map(|p| p.x).max().unwrap();

    // This is the node to the left of the node we want to move.
    let bfs_goal = Point::new(max_x - 1, 0);

    // This is the position of the empty node.
    let empty = input
        .iter()
        .find(|(_, v)| v.used == 0)
        .map(|(p, _)| p)
        .unwrap();

    // This is the size of disc that determine if it's a wall. It turns out to just be the size of
    // the empty.
    let wall_size = input.get(empty).unwrap().avail;

    // Figure out how far we need to go to move the empty node.
    let dist_to_start_neighbor = bfs(input, empty, bfs_goal, wall_size);

    // Then it's just that distance plus the number of rotations you have to do to get the node
    // moved over.
    Ok(dist_to_start_neighbor + (max_x as usize - 1) * 5 + 1)
}

fn bfs(grid: &Input, start: &Point, end: Point, wall_size: usize) -> usize {
    let mut frontier = VecDeque::new();
    frontier.push_back((*start, 0));

    let mut visited = FxHashSet::default();
    visited.insert(*start);

    let deltas = [
        Point::new(0, 1),
        Point::new(0, -1),
        Point::new(1, 0),
        Point::new(-1, 0),
    ];

    while let Some((cur, dist)) = frontier.pop_front() {
        if cur == end {
            return dist;
        }

        for delta in deltas {
            // Check to see if we have the next point on the grid.
            let next = cur + delta;
            let node = match grid.get(&next) {
                Some(n) => n,
                None => continue,
            };

            // Make sure we can move (not a wall)
            if node.used > wall_size {
                continue;
            }

            // Only add to frontier if we haven't been visited.
            if visited.insert(next) {
                frontier.push_back((next, dist + 1));
            }
        }
    }

    0
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

