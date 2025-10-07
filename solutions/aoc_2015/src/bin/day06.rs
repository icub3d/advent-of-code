use std::time::Instant;

const INPUT: &str = include_str!("inputs/day06.txt");

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn parse(s: &str) -> Self {
        let parts: Vec<&str> = s.split(',').collect();
        let x = parts[0].parse().unwrap();
        let y = parts[1].parse().unwrap();
        Point { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum InstructionType {
    TurnOn,
    TurnOff,
    Toggle,
}

impl InstructionType {
    fn parse(s: &str) -> Self {
        if s.starts_with("toggle") {
            InstructionType::Toggle
        } else if s.starts_with("turn on") {
            InstructionType::TurnOn
        } else {
            InstructionType::TurnOff
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Instruction {
    instruction_type: InstructionType,
    start: Point,
    end: Point,
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let instruction_type = InstructionType::parse(s);
        let parts: Vec<&str> = s.split_whitespace().collect();
        let (start, end) = match instruction_type {
            InstructionType::TurnOff | InstructionType::TurnOn => {
                (Point::parse(parts[2]), Point::parse(parts[4]))
            }
            InstructionType::Toggle => (Point::parse(parts[1]), Point::parse(parts[3])),
        };
        Self {
            instruction_type,
            start,
            end,
        }
    }
}

fn p1(input: &str) -> usize {
    let instructions = input
        .lines()
        .map(Instruction::parse)
        .collect::<Vec<Instruction>>();

    let mut grid = vec![vec![false; 1000]; 1000];
    for instruction in instructions {
        for row in grid
            .iter_mut()
            .take(instruction.end.y + 1)
            .skip(instruction.start.y)
        {
            for cell in row[instruction.start.x..=instruction.end.x].iter_mut() {
                match instruction.instruction_type {
                    InstructionType::TurnOff => *cell = false,
                    InstructionType::TurnOn => *cell = true,
                    InstructionType::Toggle => *cell = !*cell,
                }
            }
        }
    }
    grid.into_iter().flatten().filter(|x| *x).count()
}

fn p2(input: &str) -> usize {
    let instructions = input
        .lines()
        .map(Instruction::parse)
        .collect::<Vec<Instruction>>();

    let mut grid = vec![vec![0_usize; 1000]; 1000];
    for instruction in instructions {
        for row in grid
            .iter_mut()
            .take(instruction.end.y + 1)
            .skip(instruction.start.y)
        {
            for cell in &mut row[instruction.start.x..=instruction.end.x] {
                match instruction.instruction_type {
                    InstructionType::TurnOff => *cell = cell.saturating_sub(1),
                    InstructionType::TurnOn => *cell += 1,
                    InstructionType::Toggle => *cell += 2,
                }
            }
        }
    }
    grid.into_iter().flatten().sum()
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
