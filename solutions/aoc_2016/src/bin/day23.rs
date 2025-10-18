use std::{error::Error, time::Instant};

#[derive(Debug, Copy, Clone)]
enum Value {
    Literal(isize),
    Register(usize),
}

impl Value {
    fn parse(input: &str) -> Result<Self> {
        Ok(match input.parse::<isize>() {
            Ok(i) => Self::Literal(i),
            _ => Self::Register(convert_register(input)),
        })
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

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Cpy(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, Value),
    Tgl(Value),
}

impl Instruction {
    fn parse(input: &str) -> Result<Self> {
        let parts = input.split_whitespace().collect::<Vec<&str>>();
        Ok(match parts[..] {
            ["cpy", x, y] => Instruction::Cpy(Value::parse(x)?, Value::parse(y)?),
            ["inc", x] => Instruction::Inc(Value::parse(x)?),
            ["dec", x] => Instruction::Dec(Value::parse(x)?),
            ["jnz", x, y] => Instruction::Jnz(Value::parse(x)?, Value::parse(y)?),
            ["tgl", x] => Instruction::Tgl(Value::parse(x)?),
            _ => panic!("invalid instruction: {}", input),
        })
    }
}

const INPUT: &str = include_str!("inputs/day23.txt");
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Input<'a> = Vec<Instruction>;

fn parse_input(input: &'_ str) -> Result<Input<'_>> {
    input.lines().map(Instruction::parse).collect()
}

fn p1(input: &Input) -> Result<isize> {
    let mut input = input.clone();
    let mut pc: isize = 0;
    let mut registers = [0; 4];
    registers[0] = 7;
    while pc >= 0 && pc < input.len() as isize {
        match &input[pc as usize] {
            Instruction::Cpy(v, r) => {
                if let Value::Register(r) = r {
                    registers[*r] = v.evaluate(&registers);
                }
            }
            Instruction::Inc(r) => {
                if let Value::Register(r) = r {
                    registers[*r] += 1;
                }
            }
            Instruction::Dec(r) => {
                if let Value::Register(r) = r {
                    registers[*r] -= 1;
                }
            }
            Instruction::Jnz(v, i) => {
                if v.evaluate(&registers) != 0 {
                    pc += i.evaluate(&registers);
                    continue;
                }
            }
            Instruction::Tgl(v) => {
                let delta = v.evaluate(&registers);
                let next = pc + delta;
                if next < 0 || next >= input.len() as isize {
                    pc += 1;
                    continue;
                }
                let next = next as usize;
                input[next] = match input[next] {
                    Instruction::Tgl(v) => Instruction::Inc(v),
                    Instruction::Dec(v) => Instruction::Inc(v),
                    Instruction::Inc(v) => Instruction::Dec(v),
                    Instruction::Jnz(v, i) => Instruction::Cpy(v, i),
                    Instruction::Cpy(v, i) => Instruction::Jnz(v, i),
                };
            }
        }
        pc += 1;
    }
    Ok(registers[0])
}

fn decode_input(a: isize) -> isize {
    let mut a = a;
    // cpy a b
    let mut b = a;

    // dec b
    b -= 1; // b = 11
    while b > 0 {
        // cpy a d
        let d = a;

        // cpy 0 a - reset below
        // a = 0;

        // cpy b c
        let c = b;
        // inc a
        // dec c
        // jnz c -2
        // dec d
        // jnz d -5
        a = c * d;

        // dec b
        b -= 1;

        // This part toggles the commands below to turn them into c*d
        // cpy b c
        // cpy c d
        // c = b;
        // d = c;
        // dec d
        // inc c
        // jnz d -2
        // c += d;
        // tgl c
    }
    // cpy -16 c
    // jnz 1 c

    // cpy 76 c
    // cpy 80 d
    // inc a
    // dec d
    // jnz d -2
    // dec c
    // jnz c -5
    a + (76 * 80)
}

fn p2(_input: &Input) -> Result<isize> {
    Ok(decode_input(12))
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

