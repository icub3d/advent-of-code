use std::{ops::Add, time::Instant};

const INPUT: &str = include_str!("inputs/day02.txt");

type Int = i32;
type Input<'a> = Vec<Vec<char>>;

fn parse_input(input: &'_ str) -> Input<'_> {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

#[derive(Clone, Copy)]
struct Point {
    x: Int,
    y: Int,
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

const DELTAS: [Point; 4] = [
    Point { x: 0, y: -1 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 1, y: 0 },
];

fn delta(direction: char) -> Point {
    match direction {
        'U' => DELTAS[0],
        'D' => DELTAS[1],
        'L' => DELTAS[2],
        _ => DELTAS[3],
    }
}

impl Point {
    fn new(x: Int, y: Int) -> Self {
        Self { x, y }
    }
}

fn find_code(pad: &[Vec<char>], input: &Input, x: Int, y: Int) -> String {
    let mut code = String::with_capacity(pad.len());
    let mut cur = Point::new(x, y);
    let len = pad.len() as Int;
    for moves in input {
        for &m in moves {
            // Get our new positions.
            let next = cur + delta(m);

            // Verify it's a valid position; we've already done bounds checking in next().
            if next.x < 0
                || next.y < 0
                || next.x >= len
                || next.y >= len
                || pad[next.y as usize][next.x as usize] == ' '
            {
                continue;
            }
            cur = next;
        }
        code.push(pad[cur.y as usize][cur.x as usize]);
    }
    code
}

fn p1(input: &Input) -> String {
    let pad = [
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];
    find_code(&pad, input, 1, 1)
}

fn p2(input: &Input) -> String {
    let pad = [
        vec![' ', ' ', '1', ' ', ' '],
        vec![' ', '2', '3', '4', ' '],
        vec!['5', '6', '7', '8', '9'],
        vec![' ', 'A', 'B', 'C', ' '],
        vec![' ', ' ', 'D', ' ', ' '],
    ];
    find_code(&pad, input, 0, 2)
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
