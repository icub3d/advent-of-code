use std::{
    ops::{Index, IndexMut},
    time::Instant,
};

const INPUT: &str = include_str!("inputs/day23.txt");

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
}

struct Machine {
    a: usize,
    b: usize,
    pc: isize,
    instructions: Vec<Instruction>,
}

impl Index<Register> for Machine {
    type Output = usize;

    fn index(&self, index: Register) -> &Self::Output {
        match index {
            Register::A => &self.a,
            Register::B => &self.b,
        }
    }
}

impl IndexMut<Register> for Machine {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        match index {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
        }
    }
}

impl Machine {
    fn new(input: &str, a: usize) -> Self {
        let instructions = input.lines().map(Instruction::parse).collect::<Vec<_>>();
        Self {
            a,
            b: 0,
            pc: 0,
            instructions,
        }
    }

    fn run(&mut self) {
        while self.pc >= 0 && self.pc < self.instructions.len() as isize {
            self.evaluate()
        }
    }

    // Evaluate the instruction and update the machine state.
    fn evaluate(&mut self) {
        let instruction = self.instructions[self.pc as usize];
        match instruction {
            Instruction::Hlf(r) => {
                self[r] /= 2;
                self.pc += 1;
            }
            Instruction::Tpl(r) => {
                self[r] *= 3;
                self.pc += 1;
            }
            Instruction::Inc(r) => {
                self[r] += 1;
                self.pc += 1;
            }
            Instruction::Jmp(o) => self.pc += o,
            Instruction::Jie(r, o) => {
                if self[r].is_multiple_of(2) {
                    self.pc += o;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Jio(r, o) => {
                if self[r] == 1 {
                    self.pc += o;
                } else {
                    self.pc += 1;
                }
            }
        }
    }
}

fn p1(input: &str) -> usize {
    let mut machine = Machine::new(input, 0);
    machine.run();
    machine.b
}

fn p2(input: &str) -> usize {
    let mut machine = Machine::new(input, 1);
    machine.run();
    machine.b
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
