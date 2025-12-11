use std::time::Instant;

const INPUT: &[u8] = include_bytes!("inputs/day06.txt");

fn p1(input: &[u8]) -> usize {
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

fn p2(input: &[u8]) -> usize {
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
    println!("p1_fast {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
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
}
