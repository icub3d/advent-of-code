use std::collections::HashMap;
use std::time::Instant;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, u16};
use nom::{IResult, Parser, branch::alt, combinator::map};

const INPUT: &'static str = include_str!("inputs/day07.txt");

#[derive(Debug, Copy, Clone)]
enum Operand<'a> {
    Value(u16),
    Variable(&'a str),
}

impl<'a> Operand<'a> {
    fn parse(input: &'a str) -> IResult<&'a str, Operand<'a>> {
        alt((map(u16, Operand::Value), map(alpha1, Operand::Variable))).parse(input)
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
    fn parse(input: &'a str) -> IResult<&'a str, (Instruction<'a>, &'a str)> {
        alt((
            // NOT operand -> variable
            map(
                (tag("NOT "), Operand::parse, tag(" -> "), alpha1),
                |(_, operand, _, var)| (Instruction::Not(operand), var),
            ),
            // operand LSHIFT operand -> variable
            map(
                (
                    Operand::parse,
                    tag(" LSHIFT "),
                    Operand::parse,
                    tag(" -> "),
                    alpha1,
                ),
                |(op1, _, op2, _, var)| (Instruction::LShift(op1, op2), var),
            ),
            // operand RSHIFT operand -> variable
            map(
                (
                    Operand::parse,
                    tag(" RSHIFT "),
                    Operand::parse,
                    tag(" -> "),
                    alpha1,
                ),
                |(op1, _, op2, _, var)| (Instruction::RShift(op1, op2), var),
            ),
            // operand AND operand -> variable
            map(
                (
                    Operand::parse,
                    tag(" AND "),
                    Operand::parse,
                    tag(" -> "),
                    alpha1,
                ),
                |(op1, _, op2, _, var)| (Instruction::And(op1, op2), var),
            ),
            // operand OR operand -> variable
            map(
                (
                    Operand::parse,
                    tag(" OR "),
                    Operand::parse,
                    tag(" -> "),
                    alpha1,
                ),
                |(op1, _, op2, _, var)| (Instruction::Or(op1, op2), var),
            ),
            // operand -> variable (direct assignment)
            map(
                (Operand::parse, tag(" -> "), alpha1),
                |(operand, _, var)| (Instruction::Set(operand), var),
            ),
        ))
        .parse(input)
    }
}

pub fn p1(input: &str) -> u16 {
    let instructions = input
        .lines()
        .map(|l| Instruction::parse(l).unwrap().1)
        .map(|(l, r)| (r, l))
        .collect::<HashMap<&str, Instruction>>();
    let mut memo: HashMap<&str, u16> = HashMap::new();
    solve_circuit(&instructions, &mut memo, &"a")
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

pub fn p2(input: &str, b: u16) -> u16 {
    let instructions = input
        .lines()
        .map(|l| Instruction::parse(l).unwrap().1)
        .map(|(l, r)| (r, l))
        .collect::<HashMap<&str, Instruction>>();

    let mut memo: HashMap<&str, u16> = HashMap::new();
    memo.insert("b", b);
    let b = solve_circuit(&instructions, &mut memo, &"a");
    b
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT, solution);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
