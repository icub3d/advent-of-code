use std::time::Instant;

// TODO see _fast solution for recap.
// 🚀 Solution 🚀
// p1 48.661µs 6757749566978
// p2 49.283µs 10603075273949
// p1_fast 15.93µs 6757749566978
// p2_fast 13.035µs 10603075273949

const INPUT: &[u8] = include_bytes!("inputs/day06.txt");

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

impl From<u8> for Operator {
    fn from(value: u8) -> Self {
        match value {
            b'*' => Operator::Mul,
            _ => Operator::Add,
        }
    }
}

// We use the operator lines to delineate where blocks of equations to solve exist.
fn chunk_operators(op: &[u8]) -> impl Iterator<Item = &[u8]> {
    let mut remainder = op;

    std::iter::from_fn(move || {
        // Are we done?
        if remainder.is_empty() {
            return None;
        }

        // Find the position of the next operator (or end).
        let end = remainder
            .iter()
            .enumerate()
            .skip(1)
            .find(|(_, b)| **b != b' ')
            .map(|(idx, _)| idx)
            .unwrap_or(remainder.len());

        // We'll return out chunk and process the remainder next.
        let chunk = &remainder[..end];
        remainder = &remainder[end..];
        Some(chunk)
    })
}

// Parse the given input by making blocks of equations and passing the blocks of numbers to the given mapper.
fn parse<F>(input: &[u8], mut mapper: F) -> impl Iterator<Item = (Vec<usize>, Operator)>
where
    F: FnMut(&[&[u8]]) -> Vec<usize>,
{
    let grid = input
        .split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();
    let len = grid.len();
    // use our operator to determine the size of each block.
    chunk_operators(grid[len - 1]).scan(0, move |cur, op| {
        // Get the block from each of the rest of the lines.
        let numbers = grid[..len - 1]
            .iter()
            .map(|line| &line[*cur..*cur + op.len()])
            .collect::<Vec<_>>();

        // Update our scanner and return the mapped numbers and the operator.
        *cur += op.len();
        let op = Operator::from(op[0]);
        Some((mapper(&numbers), op))
    })
}

// Our mapper here is just to read all the "lines" and turn them into numbers.
fn p1_mapper(block: &[&[u8]]) -> Vec<usize> {
    block
        .iter()
        .map(|n| {
            n.trim_ascii()
                .iter()
                .fold(0, |acc, b| acc * 10 + (b - b'0') as usize)
        })
        .collect::<Vec<_>>()
}

fn p1(input: &[u8]) -> usize {
    // Get our operations and sum the results.
    parse(input, p1_mapper)
        .map(|(numbers, op)| match op {
            Operator::Add => numbers.iter().sum::<usize>(),
            Operator::Mul => numbers.iter().product(),
        })
        .sum()
}

// A helper function to turn a vertical slice of a block into a number.
fn make_vertical_number(values: &[&[u8]], index: usize) -> usize {
    let mut v = 0;
    for vv in values {
        if vv[index] == b' ' {
            continue;
        }
        v = v * 10 + (vv[index] - b'0') as usize;
    }
    v
}

// Our mapper here should make the vertical numbers but we should ignore the empty line at the end (if we have one).
fn p2_mapper(block: &[&[u8]]) -> Vec<usize> {
    (0..block[0].len())
        .filter(|i| !block.iter().all(|vv| vv[*i] == b' '))
        .map(|i| make_vertical_number(block, i))
        .collect::<Vec<_>>()
}

fn p2(input: &[u8]) -> usize {
    // Do the same as p1 but with the vertical mapper.
    parse(input, p2_mapper)
        .map(|(numbers, op)| match op {
            Operator::Add => numbers.iter().sum::<usize>(),
            Operator::Mul => numbers.iter().product(),
        })
        .sum()
}

fn p1_fast(input: &[u8]) -> usize {
    // Determine how long each line will be.
    let stride = input.iter().position(|&b| b == b'\n').unwrap() + 1;

    // Find out how many lines there are and where our data and ops are.
    let lines = input.len().div_ceil(stride);
    let data = lines - 1;
    let ops = &input[data * stride..];

    // Track our total for the entire solution here.
    let mut total = 0;

    // Track where the current block ends.
    let mut end = ops.len();

    // Iterate backwards through the operator line
    while let Some(cur) = ops[..end].iter().rposition(|&b| !b.is_ascii_whitespace()) {
        let b = ops[cur];

        // Calculate value using row-wise numbers
        let mut block = if b == b'*' { 1 } else { 0 };
        for row in 0..data {
            // Calculate the value at this row.
            let val = input[(row * stride) + cur..(row * stride) + end]
                .iter()
                .filter(|c| c.is_ascii_digit())
                .fold(0, |acc, c| acc * 10 + (c - b'0') as usize);

            if b == b'*' {
                block *= val;
            } else {
                block += val;
            }
        }

        // Update our total and move our position.
        total += block;
        end = cur;
    }

    total
}

fn p2_fast(input: &[u8]) -> usize {
    // Determine how long each line will be.
    let stride = input.iter().position(|&b| b == b'\n').unwrap() + 1;

    // Find out how many lines there are and where our data and ops are.
    let lines = input.len().div_ceil(stride);
    let data = lines - 1;
    let ops = &input[data * stride..];

    // Track our total for the entire solution here.
    let mut total = 0;

    // Track where the current block ends.
    let mut end = ops.len();

    // Iterate backwards through the operator line
    while let Some(cur) = ops[..end].iter().rposition(|&b| !b.is_ascii_whitespace()) {
        let b = ops[cur];

        let mut block = if b == b'*' { 1 } else { 0 };

        // We want to ignore the white space on all the blocks but the first (last) block.
        if total != 0 {
            end -= 1;
        }
        // Calculate value using column-wise numbers
        for col in cur..end {
            let mut val = 0;
            for row in 0..data {
                let c = input[row * stride + col];
                if c.is_ascii_digit() {
                    val = val * 10 + (c - b'0') as usize;
                }
            }
            if b == b'*' {
                block *= val;
            } else {
                block += val;
            }
        }

        // Update our total and move our position.
        total += block;
        end = cur;
    }

    total
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p1_fast(INPUT);
    println!("p1_fast {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2_fast(INPUT);
    println!("p2_fast {:?} {}", now.elapsed(), solution);
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &[u8] =
        "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n".as_bytes();

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 4277556);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 3263827);
    }

    #[test]
    fn test_p1_fast() {
        assert_eq!(p1_fast(INPUT), 4277556);
    }

    #[test]
    fn test_p2_fast() {
        assert_eq!(p2_fast(INPUT), 3263827);
    }
}
