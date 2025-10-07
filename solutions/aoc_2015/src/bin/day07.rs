use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("inputs/day07.txt");

#[derive(Debug, Copy, Clone)]
enum Operand<'a> {
    Value(u16),
    Variable(&'a str),
}

impl<'a> Operand<'a> {
    fn parse(input: &'a str) -> Operand<'a> {
        match input.parse::<u16>() {
            Ok(value) => Operand::Value(value),
            Err(_) => Operand::Variable(input),
        }
    }
}

#[derive(Debug)]
enum Instruction<'a> {
    Set(Operand<'a>),
    And(Operand<'a>, Operand<'a>),
    Or(Operand<'a>, Operand<'a>),
    LShift(Operand<'a>, Operand<'a>),
    RShift(Operand<'a>, Operand<'a>),
    Not(Operand<'a>),
}

impl<'a> Instruction<'a> {
    fn parse(input: &'a str) -> (Instruction<'a>, &'a str) {
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.as_slice() {
            // NOT operand -> variable
            ["NOT", operand, "->", var] => (Instruction::Not(Operand::parse(operand)), var),
            // operand LSHIFT operand -> variable
            [op1, "LSHIFT", op2, "->", var] => (
                Instruction::LShift(Operand::parse(op1), Operand::parse(op2)),
                var,
            ),
            // operand RSHIFT operand -> variable
            [op1, "RSHIFT", op2, "->", var] => (
                Instruction::RShift(Operand::parse(op1), Operand::parse(op2)),
                var,
            ),
            // operand AND operand -> variable
            [op1, "AND", op2, "->", var] => (
                Instruction::And(Operand::parse(op1), Operand::parse(op2)),
                var,
            ),
            // operand OR operand -> variable
            [op1, "OR", op2, "->", var] => (
                Instruction::Or(Operand::parse(op1), Operand::parse(op2)),
                var,
            ),
            // operand -> variable (direct assignment)
            [operand, "->", var] => (Instruction::Set(Operand::parse(operand)), var),
            _ => panic!("Invalid instruction: {}", input),
        }
    }
}

fn p1(input: &str) -> u16 {
    let instructions = input
        .lines()
        .map(Instruction::parse)
        .map(|(l, r)| (r, l))
        .collect::<HashMap<&str, Instruction>>();
    let mut memo: HashMap<&str, u16> = HashMap::new();
    solve_circuit(&instructions, &mut memo, "a")
}

fn solve_circuit<'a>(
    instructions: &'a HashMap<&'a str, Instruction<'a>>,
    memo: &mut HashMap<&'a str, u16>,
    variable: &'a str,
) -> u16 {
    if let Some(v) = memo.get(variable) {
        return *v;
    }
    let instruction = instructions.get(variable).unwrap();

    macro_rules! eval_operand {
        ($operand:expr) => {
            match $operand {
                Operand::Value(value) => *value,
                Operand::Variable(var) => solve_circuit(instructions, memo, var),
            }
        };
    }

    let result = match instruction {
        Instruction::Set(l) => eval_operand!(l),
        Instruction::And(l, r) => eval_operand!(l) & eval_operand!(r),
        Instruction::Or(l, r) => eval_operand!(l) | eval_operand!(r),
        Instruction::RShift(l, r) => eval_operand!(l) >> eval_operand!(r),
        Instruction::LShift(l, r) => eval_operand!(l) << eval_operand!(r),
        Instruction::Not(l) => !eval_operand!(l),
    };
    memo.insert(variable, result);
    result
}

fn p2(input: &str, b: u16) -> u16 {
    let instructions = input
        .lines()
        .map(Instruction::parse)
        .map(|(l, r)| (r, l))
        .collect::<HashMap<&str, Instruction>>();

    let mut memo: HashMap<&str, u16> = HashMap::new();
    memo.insert("b", b);
    solve_circuit(&instructions, &mut memo, "a")
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT, solution);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
