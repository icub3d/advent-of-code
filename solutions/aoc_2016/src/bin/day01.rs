use std::{
    ops::{Add, AddAssign, Mul},
    time::Instant,
};

use rustc_hash::FxHashSet;

const INPUT: &str = include_str!("inputs/day01.txt");

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
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

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Point {
    fn change_direction(&self, direction: char) -> Self {
        match direction {
            'R' => Point {
                x: self.y,
                y: -self.x,
            },
            'L' => Point {
                x: -self.y,
                y: self.x,
            },
            _ => panic!("invalid direction"),
        }
    }
}

struct Position {
    direction: Point,
    location: Point,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            direction: Point { x: 0, y: 1 },
            location: Point { x: 0, y: 0 },
        }
    }
}

impl Position {
    fn change_direction(&mut self, direction: char) {
        self.direction = self.direction.change_direction(direction);
    }

    fn step(&mut self) {
        self.location += self.direction;
    }

    fn walk(&mut self, direction: char, distance: isize) {
        self.direction = self.direction.change_direction(direction);
        self.location += self.direction * distance;
    }
}

type Input<'a> = Vec<(char, isize)>;

fn parse_input(input: &'_ str) -> Input<'_> {
    input
        .trim()
        .split(", ")
        .map(|l| (l.chars().next().unwrap(), l[1..].parse().unwrap()))
        .collect()
}

fn p1(input: &Input) -> isize {
    let mut cur = Position::default();
    for (direction, distance) in input {
        cur.walk(*direction, *distance);
    }
    // Taxi-cab distance from (0, 0)
    cur.location.x.abs() + cur.location.y.abs()
}

fn p2(input: &Input) -> isize {
    let mut cur = Position::default();
    let mut seen = FxHashSet::default();
    'outer: for (direction, distance) in input {
        cur.change_direction(*direction);
        for _ in 0..*distance {
            cur.step();
            if !seen.insert(cur.location) {
                break 'outer;
            }
        }
    }
    cur.location.x.abs() + cur.location.y.abs()
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
