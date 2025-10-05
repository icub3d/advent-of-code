use std::time::Instant;

const INPUT: &'static str = include_str!("inputs/day23.txt");

#[derive(Debug, Eq, PartialEq)]
enum Register {
    A,
    B,
}

impl Register {
    fn parse(input: &str) -> Self {
        match input {
            "a" => Register::A,
            "b" => Register::B,
            _ => panic!("unknown register: {}", input),
        }
    }

    fn one(&self, a: usize, b: usize) -> bool {
        match self {
            Register::A => a == 1,
            Register::B => b == 1,
        }
    }

    fn even(&self, a: usize, b: usize) -> bool {
        match self {
            Register::A => a % 2 == 0,
            Register::B => b % 2 == 0,
        }
    }

    fn increment(&self, a: &mut usize, b: &mut usize) {
        match self {
            Register::A => *a += 1,
            Register::B => *b += 1,
        }
    }

    fn half(&self, a: &mut usize, b: &mut usize) {
        match self {
            Register::A => *a /= 2,
            Register::B => *b /= 2,
        }
    }

    fn triple(&self, a: &mut usize, b: &mut usize) {
        match self {
            Register::A => *a *= 3,
            Register::B => *b *= 3,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(isize),
    Jie(Register, isize),
    Jio(Register, isize),
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        let parts = input.split(" ").collect::<Vec<_>>();
        match parts[0] {
            "hlf" => Instruction::Hlf(Register::parse(parts[1])),
            "tpl" => Instruction::Tpl(Register::parse(parts[1])),
            "inc" => Instruction::Inc(Register::parse(parts[1])),
            "jmp" => Instruction::Jmp(parts[1].parse::<isize>().unwrap()),
            "jie" => Instruction::Jie(
                Register::parse(parts[1].trim_end_matches(",")),
                parts[2].parse::<isize>().unwrap(),
            ),
            "jio" => Instruction::Jio(
                Register::parse(parts[1].trim_end_matches(",")),
                parts[2].parse::<isize>().unwrap(),
            ),
            _ => panic!("unknown instruction: {}", input),
        }
    }

    fn evaluate(&self, pc: &mut isize, a: &mut usize, b: &mut usize) {
        match self {
            Instruction::Hlf(r) => {
                r.half(a, b);
                *pc += 1;
            }
            Instruction::Tpl(r) => {
                r.triple(a, b);
                *pc += 1;
            }
            Instruction::Inc(r) => {
                r.increment(a, b);
                *pc += 1;
            }
            Instruction::Jmp(o) => *pc += o,
            Instruction::Jie(r, o) => {
                if r.even(*a, *b) {
                    *pc += o;
                } else {
                    *pc += 1;
                }
            }
            Instruction::Jio(r, o) => {
                if r.one(*a, *b) {
                    *pc += o;
                } else {
                    *pc += 1;
                }
            }
        }
    }
}

pub fn p1(input: &str) -> usize {
    run(input, 0)
}

pub fn p2(input: &str) -> usize {
    run(input, 1)
}

pub fn run(input: &str, a: usize) -> usize {
    let instructions = input.lines().map(Instruction::parse).collect::<Vec<_>>();
    let mut pc = 0 as isize;
    let mut a = a;
    let mut b = 0;

    while pc >= 0 && pc < instructions.len() as isize {
        instructions[pc as usize].evaluate(&mut pc, &mut a, &mut b);
    }

    b
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
