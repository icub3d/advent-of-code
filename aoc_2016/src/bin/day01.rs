use std::{
    error::Error,
    ops::{AddAssign, Mul},
    time::Instant,
};

use rustc_hash::{FxBuildHasher, FxHashSet};

// isize vs i32 --> i32 is marginally faster
type Int = i32;

const INPUT: &str = include_str!("inputs/day01.txt");

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: Int,
    y: Int,
}

impl Point {
    fn new(x: Int, y: Int) -> Self {
        Self { x, y }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<Int> for Point {
    type Output = Point;

    fn mul(self, rhs: Int) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

/// Allow for change direction via add_assign for char.
impl AddAssign<char> for Point {
    fn add_assign(&mut self, rhs: char) {
        *self = match rhs {
            'R' => Point::new(self.y, -self.x),
            _ => Point::new(-self.y, self.x),
        };
    }
}

struct State {
    direction: Point,
    location: Point,
}

impl Default for State {
    fn default() -> Self {
        Self {
            direction: Point::new(0, 1),
            location: Point::new(0, 0),
        }
    }
}

type Input<'a> = Vec<(char, Int)>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn parse_input(input: &'_ str) -> Result<Input<'_>> {
    input
        .trim()
        .split(", ")
        .map(|line| {
            let mut chars = line.chars();
            Ok((
                chars.next().ok_or("empty input")?,
                chars.as_str().parse::<Int>()?,
            ))
        })
        .collect()
}

fn p1(input: &Input) -> Int {
    let end = input
        .iter()
        .fold(State::default(), |mut state, &(direction, distance)| {
            state.direction += direction;
            state.location += state.direction * distance;
            state
        });
    // Taxi-cab distance from (0, 0)
    end.location.x.abs() + end.location.y.abs()
}

fn p2(input: &Input) -> Int {
    // Initialize hash with capacity saves about 25%. You obviously need to find a sweet spot for
    // capacity. I also tried with Vec as that's sometimes faster, not in this case though.
    let mut seen = FxHashSet::with_capacity_and_hasher(256, FxBuildHasher);
    let mut cur = State::default();
    'outer: for &(direction, distance) in input {
        cur.direction += direction;
        for _ in 0..distance {
            cur.location += cur.direction;
            if !seen.insert(cur.location) {
                break 'outer;
            }
        }
    }
    // Taxi-cab distance from (0, 0)
    cur.location.x.abs() + cur.location.y.abs()
}

fn main() -> Result<()> {
    let now = Instant::now();
    let input = parse_input(INPUT)?;
    let solution = p1(&input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(&input);
    println!("p2 {:?} {}", now.elapsed(), solution);

    Ok(())
}
