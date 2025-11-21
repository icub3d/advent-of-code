use std::time::Instant;

const INPUT: &str = include_str!("inputs/day10.txt");

fn look_and_say_len(input: &str, n: usize) -> usize {
    let mut cur = input
        .trim()
        .bytes()
        .map(|b| b - b'1' + 1) // Turn the char into a u8 (integer)
        .collect::<Vec<u8>>();
    for _ in 0..n {
        // Functional method, but slower:
        // use itertools::Itertools;
        // cur = cur
        //     .into_iter()
        //     .chunk_by(|&x| x)
        //     .into_iter()
        //     .flat_map(|(key, group)| {
        //         let count = group.count();
        //         vec![count as u8, key]
        //     })
        //     .collect();
        // let mut next = Vec::new();
        // let mut chars = cur.iter().peekable();
        //
        // OR
        // cur = cur.iter().dedup_with_count()
        //     .flat_map(|(count, &val)| vec![count as u8, val])
        //     .collect();
        let mut next = Vec::with_capacity(cur.len() * 2);
        let mut chars = cur.iter().peekable();

        while let Some(c) = chars.next() {
            let mut count = 1;
            while let Some(&n) = chars.peek() {
                if n != c {
                    break;
                }
                count += 1;
                chars.next();
            }
            next.push(count);
            next.push(*c);
        }
        cur = next;
    }
    cur.len()
}

fn p1(input: &str) -> usize {
    look_and_say_len(input, 40)
}

fn p2(input: &str) -> usize {
    look_and_say_len(input, 50)
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
