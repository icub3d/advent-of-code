use std::time::Instant;

const INPUT: &str = include_str!("inputs/day12.txt");

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
    Jnz(Value, isize),
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let parts = input.split_whitespace().collect::<Vec<&str>>();
        match parts[..] {
            ["cpy", x, y] => Instruction::Cpy(Value::parse(x), convert_register(y)),
            ["inc", x] => Instruction::Inc(convert_register(x)),
            ["dec", x] => Instruction::Dec(convert_register(x)),
            ["jnz", x, y] => Instruction::Jnz(Value::parse(x), y.parse().unwrap()),
            _ => panic!("invalid instruction: {}", input),
        }
    }
}

type Input<'a> = Vec<Instruction>;

fn parse_input(input: &'_ str) -> Input<'_> {
    input.lines().map(Instruction::parse).collect()
}

fn p1(input: &Input) -> isize {
    let mut pc: isize = 0;
    let mut registers = [0; 4];
    while pc >= 0 && pc < input.len() as isize {
        match &input[pc as usize] {
            Instruction::Cpy(v, r) => registers[*r] = v.evaluate(&registers),
            Instruction::Inc(r) => registers[*r] += 1,
            Instruction::Dec(r) => registers[*r] -= 1,
            Instruction::Jnz(v, i) => {
                if v.evaluate(&registers) != 0 {
                    pc += i;
                    continue;
                }
            }
        }
        pc += 1;
    }
    registers[0]
}

fn p2(input: &Input) -> isize {
    let mut pc: isize = 0;
    let mut registers = [0; 4];
    registers[2] = 1;

    while pc >= 0 && pc < input.len() as isize {
        match &input[pc as usize] {
            Instruction::Cpy(v, r) => registers[*r] = v.evaluate(&registers),
            Instruction::Inc(r) => registers[*r] += 1,
            Instruction::Dec(r) => registers[*r] -= 1,
            Instruction::Jnz(v, i) => {
                if v.evaluate(&registers) != 0 {
                    pc += i;
                    continue;
                }
            }
        }
        pc += 1;
    }
    registers[0]
}

fn main() {
    let now = Instant::now();
    let input = parse_input(INPUT);
    let solution = p1(&input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(&input);
    println!("p2 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = code(false);
    println!("p1-code {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = code(true);
    println!("p2-code {:?} {}", now.elapsed(), solution);
}

fn code(p2: bool) -> isize {
    // cpy 1 a
    // cpy 1 b
    // cpy 26 d
    // jnz c 2
    // jnz 1 5
    let mut a = 1;
    let mut b = 1;
    let mut d = 26;

    // Part 2 d += 7;
    // cpy 7 c
    // inc d
    // dec c
    // jnz c -2
    if p2 {
        d += 7;
    }

    // cpy a c
    // inc a
    // dec b
    // jnz b -2
    // cpy c b
    // dec d
    // jnz d -6
    let mut c = 0;
    for _ in 0..d {
        c = a;
        a += b;
        b = c;
    }

    // cpy 19 c
    // cpy 11 d
    c = 19;
    d = 11;

    // inc a
    // dec d
    // jnz d -2
    // dec c
    // jnz c -5
    a + c * d
}
