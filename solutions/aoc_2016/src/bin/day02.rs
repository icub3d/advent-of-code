use std::time::Instant;

const INPUT: &str = include_str!("inputs/day02.txt");

type Input<'a> = Vec<Vec<char>>;

fn parse_input(input: &'_ str) -> Input<'_> {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn next(&self, direction: char, len: usize) -> Self {
        match direction {
            'U' => Point::new(self.x, self.y.saturating_sub(1)),
            'D' => Point::new(self.x, self.y.saturating_add(1).min(len - 1)),
            'L' => Point::new(self.x.saturating_sub(1), self.y),
            'R' => Point::new(self.x.saturating_add(1).min(len - 1), self.y),
            _ => panic!("bad direction"),
        }
    }
}

fn find_code(pad: &[Vec<char>], input: &Input, x: usize, y: usize) -> String {
    let mut code = String::new();
    let mut cur = Point::new(x, y);
    for moves in input {
        for m in moves {
            // Get our new positions.
            let next = cur.next(*m, pad.len());

            // Verify it's a valid position; we've already done bounds checking in next().
            if pad[next.y][next.x] == ' ' {
                continue;
            }
            cur = next;
        }
        code.push(pad[cur.y][cur.x]);
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
