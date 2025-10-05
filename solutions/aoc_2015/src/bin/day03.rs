use std::collections::HashSet;

const INPUT: &'static str = include_str!("inputs/day03.txt");

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    fn next(self, c: char) -> Self {
        let mut next = self;

        match c {
            '>' => next.x += 1,
            '<' => next.x -= 1,
            '^' => next.y += 1,
            'v' => next.y -= 1,
            _ => (),
        }

        next
    }
}

pub fn p1(input: &str) -> usize {
    let mut cur = Point::new(0, 0);

    let mut seen = HashSet::new();
    seen.insert(cur);

    for c in input.chars() {
        cur = cur.next(c);
        seen.insert(cur);
    }

    seen.len()
}

pub fn p2(input: &str) -> usize {
    let mut cur_santa = Point::new(0, 0);
    let mut cur_robo_santa = Point::new(0, 0);

    let mut seen = HashSet::new();
    seen.insert(cur_santa);

    let mut santas_turn = true;
    for c in input.chars() {
        if santas_turn {
            cur_santa = cur_santa.next(c);
            seen.insert(cur_santa);
        } else {
            cur_robo_santa = cur_robo_santa.next(c);
            seen.insert(cur_robo_santa);
        }
        santas_turn = !santas_turn;
    }

    seen.len()
}

fn main() {
    let now = std::time::Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = std::time::Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
