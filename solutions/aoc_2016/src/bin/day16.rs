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
    checksum_calc(input, 35651584)
}

// Helper function to calculate parity of first n digits (gray code trick)
fn parity(n: usize) -> usize {
    let gray = n ^ (n >> 1);
    ((n & gray).count_ones() ^ gray as u32) as usize & 1
}

fn checksum_calc(input: &Input, size: usize) -> String {
    // Convert input to boolean array
    let original: Vec<bool> = input.chars().map(|c| c == '1').collect();
    let reversed: Vec<bool> = original.iter().rev().map(|&b| !b).collect();

    // Determine chunk length and then sum length based on input size.
    let chunk_len = size & size.wrapping_neg();
    let sum_len = size / chunk_len;

    let mut buf: Vec<Option<bool>> = Vec::new();
    let mut total_dragons = 0;
    let mut prev_parity = 0;

    (0..sum_len)
        .map(|_| {
            // Drain from the buffer
            let buffered = buf.len().min(chunk_len);
            let taken: Vec<_> = buf.drain(..buffered).collect();

            // Track the number of 1's well see and the number of dragons we'll have to create.
            let mut ones = taken.iter().filter(|&&x| x == Some(true)).count();
            let mut dragons = taken.iter().filter(|&&x| x.is_none()).count();

            // How many full groups will we need?
            let mut remaining = chunk_len - buffered;
            let dragon_groups = remaining / ((original.len() + 1) * 2);

            // The number of ones in the original + reversed is the total length because they are opposites (complements) of each other.
            dragons += dragon_groups * 2;
            ones += original.len() * dragon_groups;

            // At this point, we may need some partial dragon curve data.
            // We'll add it to the buffer, take what we need and update our counts.
            remaining %= (original.len() + 1) * 2;
            if remaining > 0 {
                // Create a full buffer, including middle bit and flipped.
                buf.extend(original.iter().map(|&b| Some(b)));
                buf.push(None);
                buf.extend(reversed.iter().map(|&b| Some(b)));
                buf.push(None);

                // Take the remaining bits we need.
                let taken: Vec<_> = buf.drain(..remaining).collect();
                ones += taken.iter().filter(|&&x| x == Some(true)).count();
                dragons += taken.iter().filter(|&&x| x.is_none()).count();
            }

            total_dragons += dragons;
            if dragons > 0 {
                let cur_parity = parity(total_dragons);
                let parity_diff = cur_parity ^ prev_parity;
                prev_parity = cur_parity;
                // Our number of 1's needs to be adjusted based on the parity difference.
                ones += parity_diff;
            }

            // Return '1' if ones is even, '0' if odd
            match ones.is_multiple_of(2) {
                true => '1',
                false => '0',
            }
        })
        .collect()
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
