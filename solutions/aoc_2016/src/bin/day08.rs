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
const LETTER_WIDTH: usize = 5;

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
            Instruction::RotateRow(row, distance) => self.pixels[*row].rotate_right(*distance),
            Instruction::RotateColumn(row, distance) => self.rotate_column(*row, *distance),
        }
    }

    fn rect(&mut self, width: usize, height: usize) {
        for h in 0..height.min(HEIGHT) {
            self.pixels[h][..width.min(WIDTH)].fill('#');
        }
    }

    fn rotate_column(&mut self, column: usize, distance: usize) {
        // Grab column and rotate it.
        let mut new_column = self.pixels.iter().map(|r| r[column]).collect::<Vec<_>>();
        new_column.rotate_right(distance);

        // Put the new column back in place.
        for (row, val) in self.pixels.iter_mut().zip(new_column.iter()) {
            row[column] = *val;
        }
    }

    fn decode_message(&self) -> String {
        let mut message = String::new();

        for start in (0..WIDTH).step_by(LETTER_WIDTH) {
            if start + LETTER_WIDTH > WIDTH {
                break;
            }

            let mut glyph = String::new();
            let mut has_lit_pixel = false;

            for row in 0..HEIGHT {
                let slice = &self.pixels[row][start..start + LETTER_WIDTH];
                if slice.contains(&'#') {
                    has_lit_pixel = true;
                }
                glyph.extend(slice);
                if row + 1 < HEIGHT {
                    glyph.push('\n');
                }
            }

            if !has_lit_pixel {
                continue;
            }

            let letter = decode_glyph(&glyph).unwrap_or('?');
            message.push(letter);
        }

        message
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
    screen.decode_message()
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

fn decode_glyph(pattern: &str) -> Option<char> {
    match pattern {
        "####.\n#....\n###..\n#....\n#....\n####." => Some('E'),
        ".##..\n#..#.\n#..#.\n#..#.\n#..#.\n.##.." => Some('O'),
        ".##..\n#..#.\n#..#.\n####.\n#..#.\n#..#." => Some('A'),
        "###..\n#..#.\n#..#.\n###..\n#.#..\n#..#." => Some('R'),
        ".##..\n#..#.\n#....\n#.##.\n#..#.\n.###." => Some('G'),
        "###..\n#..#.\n#..#.\n###..\n#....\n#...." => Some('P'),
        "#..#.\n#..#.\n####.\n#..#.\n#..#.\n#..#." => Some('H'),
        "#...#\n#...#\n.#.#.\n..#..\n..#..\n..#.." => Some('Y'),
        _ => None,
    }
}
