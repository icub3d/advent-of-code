use std::time::Instant;

const INPUT: &str = include_str!("inputs/day15.txt");

type Input<'a> = Vec<(usize, usize)>;

fn parse_input(input: &'_ str) -> Input<'_> {
    input
        .lines()
        .map(|l| {
            let parts = l
                .trim_end_matches('.')
                .split_whitespace()
                .collect::<Vec<_>>();
            (parts[11].parse().unwrap(), parts[3].parse().unwrap())
        })
        .collect()
}

fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a.abs(), a.signum(), 0)
    } else {
        let (g, s1, t1) = extended_gcd(b, a % b);
        (g, t1, s1 - (a / b) * t1)
    }
}

/// normalize to [0, m)
fn norm(x: i128, m: i128) -> i128 {
    let mut r = x % m;
    if r < 0 {
        r += m;
    }
    r
}

/// Generalized CRT over pairs (remainder, modulus) -> Option<(r, m)>
/// returns smallest non-negative r and combined modulus m (lcm of moduli), or None if inconsistent.
fn crt(pairs: &[(usize, usize)]) -> Option<(usize, usize)> {
    if pairs.is_empty() {
        return Some((0, 1));
    }

    let mut r0 = pairs[0].0 as i128;
    let mut m0 = pairs[0].1 as i128;
    if m0 <= 0 {
        return None;
    }
    r0 = norm(r0, m0);

    for &(ri_u, mi_u) in pairs.iter().skip(1) {
        let mut r1 = ri_u as i128;
        let m1 = mi_u as i128;
        if m1 <= 0 {
            return None;
        }
        r1 = norm(r1, m1);

        let (g, s, _t) = extended_gcd(m0, m1);
        let diff = r1 - r0;
        if diff % g != 0 {
            return None; // inconsistent
        }

        // t ≡ (diff / g) * s  (mod m1/g)
        let m1_div_g = m1 / g;
        let t_val = norm((diff / g) * s, m1_div_g);

        let new_r = r0 + m0 * t_val;
        let lcm = (m0 / g) * m1;

        r0 = norm(new_r, lcm);
        m0 = lcm;
    }

    if r0 < 0 || m0 <= 0 {
        return None;
    }
    Some((r0 as usize, m0 as usize))
}

/// Given discs as Vec<(start_pos, positions)>, compute earliest t (if any)
fn solve(discs: &[(usize, usize)]) -> Option<usize> {
    // build congruences t ≡ (m - (p + i) % m) % m, i is 1-based index
    let mut pairs: Vec<(usize, usize)> = Vec::with_capacity(discs.len());
    for (i, &(p, m)) in discs.iter().enumerate() {
        let idx = i + 1; // discs numbered from 1
        if m == 0 {
            return None;
        }
        let rem = (m - ((p + idx) % m)) % m;
        pairs.push((rem, m));
    }
    crt(&pairs).map(|(r, _m)| r)
}

fn p1(input: &Input) -> usize {
    (0..)
        .find(|time| {
            input
                .iter()
                .enumerate()
                .all(|(i, (pos, modulus))| ((time + pos + i + 1) % modulus) == 0)
        })
        .unwrap()
}

fn p2(input: &Input) -> usize {
    let mut input = input.clone();
    input.push((0, 11));
    (0..)
        .find(|time| {
            input
                .iter()
                .enumerate()
                .all(|(i, (pos, modulus))| ((time + pos + i + 1) % modulus) == 0)
        })
        .unwrap()
}

fn main() {
    let now = Instant::now();
    let input = parse_input(INPUT);
    let solution = p1(&input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = solve(&input).unwrap();
    println!("p1_crt {:?} {:?}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(&input);
    println!("p2 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let mut input = input;
    input.push((0, 11));
    let solution = solve(&input).unwrap();
    println!("p2_crt {:?} {:?}", now.elapsed(), solution);
}
