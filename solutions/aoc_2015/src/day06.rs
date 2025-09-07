use std::time::Instant;

const INPUT: &'static str = include_str!("inputs/day06.txt");

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

pub fn p1(input: &str) -> anyhow::Result<usize> {
    let instructions = input
        .lines()
        .map(Instruction::parse)
        .collect::<Vec<Instruction>>();

    let mut grid = vec![vec![false; 1000]; 1000];
    for instruction in instructions {
        for y in instruction.start.y..=instruction.end.y {
            for x in instruction.start.x..=instruction.end.x {
                match instruction.instruction_type {
                    InstructionType::TurnOff => grid[y][x] = false,
                    InstructionType::TurnOn => grid[y][x] = true,
                    InstructionType::Toggle => grid[y][x] = !grid[y][x],
                }
            }
        }
    }
    Ok(grid.into_iter().flatten().filter(|x| *x).count())
}

pub fn p2(input: &str) -> anyhow::Result<usize> {
    let instructions = input
        .lines()
        .map(Instruction::parse)
        .collect::<Vec<Instruction>>();

    let mut grid = vec![vec![0 as usize; 1000]; 1000];
    for instruction in instructions {
        for y in instruction.start.y..=instruction.end.y {
            for x in instruction.start.x..=instruction.end.x {
                match instruction.instruction_type {
                    InstructionType::TurnOff => grid[y][x] = grid[y][x].saturating_sub(1),
                    InstructionType::TurnOn => grid[y][x] += 1,
                    InstructionType::Toggle => grid[y][x] += 2,
                }
            }
        }
    }
    Ok(grid.into_iter().flatten().sum())
}

pub fn solve() -> anyhow::Result<()> {
    let now = Instant::now();
    println!("p1: {} ({:?})", p1(INPUT)?, now.elapsed());
    let now = Instant::now();
    println!("p2: {} ({:?})", p2(INPUT)?, now.elapsed());
    Ok(())
}
