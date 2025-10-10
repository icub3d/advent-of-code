use std::{fmt::Display, time::Instant};

const INPUT: &str = include_str!("inputs/day08.txt");

type Input<'a> = Vec<Instruction>;

#[derive(Debug)]
enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let parts = input.split_whitespace().collect::<Vec<&str>>();
        match parts[..] {
            ["rect", dims] => {
                let (a, b) = dims.split_once('x').unwrap();
                Self::Rect(a.parse().unwrap(), b.parse().unwrap())
            }
            ["rotate", "row", row, "by", count] => {
                let (_, row) = row.split_once('=').unwrap();
                Self::RotateRow(row.parse().unwrap(), count.parse().unwrap())
            }
            ["rotate", "column", col, "by", count] => {
                let (_, col) = col.split_once('=').unwrap();
                Self::RotateColumn(col.parse().unwrap(), count.parse().unwrap())
            }
            _ => panic!("invalid instruction: {}", input),
        }
    }
}

fn parse_input(input: &'_ str) -> Input<'_> {
    input.lines().map(Instruction::parse).collect()
}

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

struct Screen {
    pixels: [[char; WIDTH]; HEIGHT],
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for h in 0..HEIGHT {
            for w in 0..WIDTH {
                let _ = write!(f, "{}", self.pixels[h][w]);
            }
            let _ = writeln!(f);
        }
        writeln!(f)
    }
}

impl Screen {
    fn new() -> Self {
        Self {
            pixels: [['.'; WIDTH]; HEIGHT],
        }
    }

    fn on(self) -> usize {
        self.pixels
            .iter()
            .map(|r| r.iter().filter(|p| **p == '#').count())
            .sum()
    }

    fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Rect(w, h) => self.rect(*w, *h),
            Instruction::RotateRow(row, distance) => self.rotate_row(*row, *distance),
            Instruction::RotateColumn(row, distance) => self.rotate_column(*row, *distance),
        }
    }

    fn rect(&mut self, width: usize, height: usize) {
        for w in 0..width.min(WIDTH) {
            for h in 0..height.min(HEIGHT) {
                self.pixels[h][w] = '#';
            }
        }
    }

    fn rotate_row(&mut self, row: usize, distance: usize) {
        // Grab row and clone it.
        let old_row = self.pixels[row];

        // Do match to figure out where each should go and place it in myself.
        for (i, b) in old_row.iter().enumerate() {
            self.pixels[row][(i + distance) % WIDTH] = *b;
        }
    }

    fn rotate_column(&mut self, column: usize, distance: usize) {
        // Grab column and clone it.
        let old_column = self.pixels.iter().map(|r| r[column]).collect::<Vec<_>>();

        // Do match to figure out where each should go and place it in myself.
        for (i, b) in old_column.iter().enumerate() {
            self.pixels[(i + distance) % HEIGHT][column] = *b;
        }
    }
}

fn p1(input: &Input) -> usize {
    let mut screen = Screen::new();
    input.iter().for_each(|i| screen.apply(i));
    screen.on()
}

fn p2(input: &Input) -> String {
    let mut screen = Screen::new();
    input.iter().for_each(|i| screen.apply(i));
    format!("\n{}", screen)
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
