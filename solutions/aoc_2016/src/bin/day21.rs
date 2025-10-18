use std::{error::Error, time::Instant};

const INPUT: &str = include_str!("inputs/day21.txt");

type Result<T> = std::result::Result<T, Box<dyn Error>>;

macro_rules! get_char {
    ($s:expr, $i:expr) => {
        $s.as_bytes()[$i] as char
    };
}

#[derive(Debug)]
enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Instruction {
    fn parse(line: &str) -> Result<Instruction> {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        Ok(match parts[..] {
            ["swap", "position", x, "with", "position", y] => {
                Instruction::SwapPosition(x.parse()?, y.parse()?)
            }
            ["swap", "letter", x, "with", "letter", y] => Instruction::SwapLetter(
                x.chars().next().ok_or("swapx")?,
                y.chars().next().ok_or("swapy")?,
            ),
            ["rotate", "left", x, "step"] => Instruction::RotateLeft(x.parse()?),
            ["rotate", "right", x, "step"] => Instruction::RotateRight(x.parse()?),
            ["rotate", "left", x, "steps"] => Instruction::RotateLeft(x.parse()?),
            ["rotate", "right", x, "steps"] => Instruction::RotateRight(x.parse()?),
            ["rotate", "based", "on", "position", "of", "letter", x] => {
                Instruction::RotateLetter(get_char!(x, 0))
            }
            ["reverse", "positions", x, "through", y] => {
                Instruction::Reverse(x.parse()?, y.parse()?)
            }
            ["move", "position", x, "to", "position", y] => {
                Instruction::Move(x.parse()?, y.parse()?)
            }
            _ => panic!("invalid instruction: {}", line),
        })
    }

    fn apply(&self, v: &mut Vec<char>) {
        match self {
            Self::SwapPosition(x, y) => v.swap(*x, *y),
            Self::SwapLetter(a, b) => {
                let x = v.iter().position(|c| c == a).unwrap();
                let y = v.iter().position(|c| c == b).unwrap();
                v.swap(x, y)
            }
            Self::RotateLeft(x) => v.rotate_left(*x),
            Self::RotateRight(x) => v.rotate_right(*x),
            Self::RotateLetter(a) => {
                let x = v.iter().position(|c| c == a).unwrap();
                let n = (1 + x + if x >= 4 { 1 } else { 0 }) % v.len();
                v.rotate_right(n);
            }
            Self::Reverse(x, y) => {
                let mut x = *x;
                let mut y = *y;
                while x < y {
                    v.swap(x, y);
                    x += 1;
                    y -= 1;
                }
            }
            Self::Move(x, y) => {
                let c = v.remove(*x);
                v.insert(*y, c);
            }
        }
    }

    fn undo(&self, v: &mut Vec<char>) {
        match self {
            Self::SwapPosition(x, y) => v.swap(*x, *y),
            Self::SwapLetter(a, b) => {
                let x = v.iter().position(|c| c == a).unwrap();
                let y = v.iter().position(|c| c == b).unwrap();
                v.swap(x, y)
            }
            Self::RotateLeft(x) => v.rotate_right(*x),
            Self::RotateRight(x) => v.rotate_left(*x),
            Self::RotateLetter(a) => {
                for i in 0..v.len() {
                    let mut guess = v.clone();
                    guess.rotate_left(i);
                    let x = guess.iter().position(|c| c == a).unwrap();
                    let n = (1 + x + if x >= 4 { 1 } else { 0 }) % v.len();
                    guess.rotate_right(n);
                    if guess == *v {
                        v.rotate_left(i);
                        break;
                    }
                }
            }
            Self::Reverse(x, y) => {
                let mut x = *x;
                let mut y = *y;
                while x < y {
                    v.swap(x, y);
                    x += 1;
                    y -= 1;
                }
            }
            Self::Move(x, y) => {
                let c = v.remove(*y);
                v.insert(*x, c);
            }
        }
    }
}

type Input<'a> = Vec<Instruction>;

fn parse_input(input: &'_ str) -> Result<Input<'_>> {
    input
        .trim()
        .lines()
        .map(Instruction::parse)
        .collect::<Result<Vec<_>>>()
}

fn p1(input: &Input) -> Result<String> {
    let mut password = "abcdefgh".chars().collect::<Vec<_>>();
    input.iter().for_each(|i| i.apply(&mut password));
    Ok(password.iter().collect::<String>())
}

fn p2(input: &Input) -> Result<String> {
    let input = input.iter().rev().collect::<Vec<_>>();
    let mut password = "fbgdceah".chars().collect::<Vec<_>>();
    input.iter().for_each(|i| i.undo(&mut password));
    Ok(password.iter().collect::<String>())
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
