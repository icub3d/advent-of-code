use std::time::Instant;

const INPUT: &str = include_str!("inputs/day16.txt");

type Input<'a> = &'a str;

fn parse_input(input: &'_ str) -> Input<'_> {
    input.trim()
}

fn dragon_curve(a: &str, n: usize) -> String {
    let mut l = a.len();
    while l < n {
        l *= 2 + 1;
    }
    let mut s = String::with_capacity(l);
    s.push_str(a);
    while s.len() < n {
        let b = s
            .chars()
            .rev()
            .map(|c| if c == '0' { '1' } else { '0' })
            .collect::<String>();
        s.push('0');
        s.push_str(&b);
    }
    s.chars().take(n).collect()
}

fn checksum(a: &str) -> String {
    // Figure out how many times we are going to do a checksum.
    let mut n = a.len();
    n /= 2;
    while n.is_multiple_of(2) {
        n /= 2;
    }
    let chunks = a.len() / n;

    // The value of the final checksum is determined by whether the number of 1's is even or odd.
    a.as_bytes()
        .chunks(chunks)
        .map(|c| {
            if c.iter().filter(|&&c| c == b'1').count().is_multiple_of(2) {
                '1'
            } else {
                '0'
            }
        })
        .collect()
}

fn p1(input: &Input) -> String {
    let data = dragon_curve(input, 272);
    checksum(&data)
}

fn p2(input: &Input) -> String {
    let data = dragon_curve(input, 35651584);
    checksum(&data)
}

fn main() {
    let now = Instant::now();
    let input = parse_input(INPUT);
    let solution = p1(&input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(&input);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
