use std::{collections::VecDeque, time::Instant};

use rustc_hash::FxHashMap;

const INPUT: &str = include_str!("inputs/day10.txt");

enum Location {
    Output(usize),
    Bot(usize),
}

impl Location {
    fn new(what: &str, value: &str) -> Self {
        match what {
            "output" => Location::Output(value.parse().unwrap()),
            "bot" => Location::Bot(value.parse().unwrap()),
            _ => panic!("invalid location: {} {}", what, value),
        }
    }

    fn apply(
        &self,
        bots: &mut FxHashMap<usize, Vec<usize>>,
        outputs: &mut FxHashMap<usize, Vec<usize>>,
        ready: &mut VecDeque<usize>,
        value: usize,
    ) {
        match self {
            Location::Bot(id) => {
                let values = bots.entry(*id).or_default();
                values.push(value);
                if values.len() >= 2 {
                    ready.push_back(*id);
                }
            }
            Location::Output(id) => outputs.entry(*id).or_default().push(value),
        }
    }
}

struct BotRule {
    low: Location,
    high: Location,
}

impl BotRule {
    fn new(low_what: &str, low_value: &str, high_what: &str, high_value: &str) -> Self {
        Self {
            low: Location::new(low_what, low_value),
            high: Location::new(high_what, high_value),
        }
    }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Input<'a> = (Vec<(usize, usize)>, FxHashMap<usize, BotRule>);

fn parse_input(input: &'_ str) -> Result<Input<'_>> {
    input.lines().try_fold(
        (Vec::new(), FxHashMap::default()),
        |(mut values, mut rules), line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            match parts[0] {
                "value" => {
                    values.push((parts[5].parse::<usize>()?, parts[1].parse::<usize>()?));
                    Ok((values, rules))
                }
                "bot" => {
                    rules.insert(
                        parts[1].parse::<usize>()?,
                        BotRule::new(parts[5], parts[6], parts[10], parts[11]),
                    );
                    Ok((values, rules))
                }

                _ => Err(Box::from(format!("invalid line: {}", line))),
            }
        },
    )
}

fn sim<F>((values, rules): &Input, mut stop: F) -> FxHashMap<usize, Vec<usize>>
where
    F: FnMut(usize, usize, usize) -> bool,
{
    let mut outputs: FxHashMap<usize, Vec<usize>> = FxHashMap::default();
    let mut bots: FxHashMap<usize, Vec<usize>> =
        values
            .iter()
            .fold(FxHashMap::default(), |mut acc, &(k, v)| {
                acc.entry(k).or_default().push(v);
                acc
            });

    let mut ready = bots
        .iter()
        .filter(|(_, v)| v.len() >= 2)
        .map(|(k, _)| *k)
        .collect::<VecDeque<usize>>();

    while let Some(id) = ready.pop_front() {
        let values = bots.get(&id).unwrap();
        let low = *values.iter().min().unwrap();
        let high = *values.iter().max().unwrap();

        if stop(low, high, id) {
            return outputs;
        }

        let rule = rules.get(&id).unwrap();
        rule.low.apply(&mut bots, &mut outputs, &mut ready, low);
        rule.high.apply(&mut bots, &mut outputs, &mut ready, high);
    }
    outputs
}

fn p1(input: &Input) -> usize {
    let mut solution = 0;
    sim(input, |low, high, bot| {
        if low == 17 && high == 61 {
            solution = bot;
            true
        } else {
            false
        }
    });
    solution
}

fn p2(input: &Input) -> usize {
    let outputs = sim(input, |_, _, _| false);
    outputs
        .iter()
        .filter(|(k, _)| [0, 1, 2].contains(*k))
        .flat_map(|(_, v)| v)
        .product()
}

fn main() -> Result<()> {
    let now = Instant::now();
    let input = parse_input(INPUT)?;
    let solution = p1(&input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(&input);
    println!("p2 {:?} {}", now.elapsed(), solution);

    Ok(())
}
