use std::{error::Error, time::Instant};

#[derive(Debug)]
enum Value {
    Literal(isize),
    Register(usize),
}

impl Value {
    fn parse(input: &str) -> Self {
        match input.parse::<isize>() {
            Ok(i) => Self::Literal(i),
            _ => Self::Register(convert_register(input)),
        }
    }

    fn evaluate(&self, registers: &[isize]) -> isize {
        match self {
            Self::Literal(i) => *i,
            Self::Register(r) => registers[*r],
        }
    }
}

fn convert_register(input: &str) -> usize {
    match input {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => panic!("invalid value: {}", input),
    }
}

#[derive(Debug)]
enum Instruction {
    Cpy(Value, usize),
    Inc(usize),
    Dec(usize),
    Out(Value),
    Jnz(Value, isize),
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let parts = input.split_whitespace().collect::<Vec<&str>>();
        match parts[..] {
            ["cpy", x, y] => Instruction::Cpy(Value::parse(x), convert_register(y)),
            ["inc", x] => Instruction::Inc(convert_register(x)),
            ["dec", x] => Instruction::Dec(convert_register(x)),
            ["out", x] => Instruction::Out(Value::parse(x)),
            ["jnz", x, y] => Instruction::Jnz(Value::parse(x), y.parse().unwrap()),
            _ => panic!("invalid instruction: {}", input),
        }
    }
}

const INPUT: &str = include_str!("inputs/day25.txt");
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Input<'a> = Vec<Instruction>;

fn parse_input(input: &'_ str) -> Result<Input<'_>> {
    Ok(input.lines().map(Instruction::parse).collect())
}

fn sim(input: &Input, a: isize) -> bool {
    let mut pc: isize = 0;
    let mut registers = [0; 4];
    registers[0] = a;
    let mut tested = 0;
    let mut expect = false;

    while pc >= 0 && pc < input.len() as isize {
        match &input[pc as usize] {
            Instruction::Cpy(v, r) => registers[*r] = v.evaluate(&registers),
            Instruction::Inc(r) => registers[*r] += 1,
            Instruction::Dec(r) => registers[*r] -= 1,
            Instruction::Out(v) => {
                if v.evaluate(&registers) != expect.into() {
                    return false;
                }
                if tested == 100 {
                    return true;
                }
                expect = !expect;
                tested += 1;
            }
            Instruction::Jnz(v, i) => {
                if v.evaluate(&registers) != 0 {
                    pc += i;
                    continue;
                }
            }
        }
        pc += 1;
    }
    false
}

fn p1(input: &Input) -> Result<usize> {
    Ok((0..)
        .map(|a| (a, sim(input, a)))
        .find(|(_, found)| *found)
        .map(|(a, _)| a as usize)
        .unwrap())
}

fn p1_decode_input() -> usize {
    let mut a = 0;
    loop {
        let n = a + 7 * 365;
        let x = n ^ (n >> 1);
        if (x & (x + 1)) == 0 && 1 & n == 0 {
            return a;
        }
        a += 1;
    }
}

fn main() -> Result<()> {
    let now = Instant::now();
    let input = parse_input(INPUT)?;
    let solution = p1(&input)?;
    println!("p1 {:?} {}", now.elapsed(), solution);
    let now = Instant::now();
    let solution = p1_decode_input();
    println!("p1_decoded {:?} {}", now.elapsed(), solution);

    Ok(())
}

