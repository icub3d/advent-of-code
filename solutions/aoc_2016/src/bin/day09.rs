use std::time::Instant;

const INPUT: &str = include_str!("inputs/day09.txt");

type Input<'a> = &'a str;

fn parse_input(input: &'_ str) -> Input<'_> {
    input.trim()
}

fn get_length(mut input: Input, recursive: bool) -> usize {
    let mut total = 0;

    while !input.is_empty() {
        let (left, right) = match input.split_once('(') {
            Some((left, right)) => (left, right),
            None => {
                total += input.len();
                break;
            }
        };
        total += left.len();

        let (left, right) = right.split_once(')').unwrap();
        let (len, count) = left.split_once('x').unwrap();
        let (len, count) = (
            len.parse::<usize>().unwrap(),
            count.parse::<usize>().unwrap(),
        );
        if recursive {
            total += count * get_length(&right[..len], recursive);
        } else {
            total += count * len;
        }
        input = &right[len..];
    }

    total
}

fn p1(input: Input) -> usize {
    get_length(input, false)
}

fn p2(input: Input) -> usize {
    get_length(input, true)
}

fn main() {
    let now = Instant::now();
    let input = parse_input(INPUT);
    let solution = p1(input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(input);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
