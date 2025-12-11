use std::{collections::VecDeque, time::Instant};

use rayon::iter::{ParallelBridge, ParallelIterator};
use rustc_hash::FxHashSet;
use z3::{Optimize, ast::Int};

const INPUT: &str = include_str!("inputs/day10.txt");
const EPSILON: f64 = 1e-9;

fn parse(input: &str) -> impl Iterator<Item = Machine> {
    input.trim().lines().map(Machine::from)
}

#[derive(Debug)]
struct Machine {
    lights: usize,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        let lights = parts
            .next()
            .map(|l| {
                // We rev here to make calculating below easier.
                l.trim_matches(['[', ']'])
                    .chars()
                    .rev()
                    .fold(0, |acc, c| (acc << 1) | if c == '#' { 1 } else { 0 })
            })
            .unwrap();

        let mut parts: Vec<&str> = parts.collect();
        let joltages = parts
            .pop()
            .unwrap()
            .trim_matches(['{', '}'])
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();

        let mut buttons: Vec<Vec<usize>> = parts
            .iter()
            .map(|b| {
                b.trim_matches(['(', ')'])
                    .split(',')
                    .map(|v| v.parse().unwrap())
                    .collect()
            })
            .collect();

        // Sorting seems to help here. Not sure why, was just trying stuff.
        buttons.sort_by_key(|b| std::cmp::Reverse(b.len()));

        Self {
            lights,
            buttons,
            joltages,
        }
    }
}

fn p1(input: &str) -> usize {
    parse(input)
        .map(|machine| {
            let mut frontier = VecDeque::new();
            frontier.push_back((0, 0));

            let mut seen = FxHashSet::default();
            seen.insert(0);

            while let Some((lights, dist)) = frontier.pop_front() {
                if lights == machine.lights {
                    return dist;
                }

                for neighbor in machine.buttons.iter() {
                    let neighbor = neighbor.iter().fold(lights, |acc, n| acc ^ (1 << n));
                    if seen.insert(neighbor) {
                        frontier.push_back((neighbor, dist + 1));
                    }
                }
            }
            unreachable!()
        })
        .sum()
}

struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
    dependents: Vec<usize>,
    independents: Vec<usize>,
}

impl Matrix {
    // Make a matrix, do a Gaussian elimination and setup the fixed and free variables.
    fn from_machine(machine: &Machine) -> Self {
        let rows = machine.joltages.len();
        let cols = machine.buttons.len();
        let mut data = vec![vec![0.0; cols + 1]; rows];

        // Add all of our buttons.
        for (c, button) in machine.buttons.iter().enumerate() {
            for &r in button {
                if r < rows {
                    data[r][c] = 1.0;
                }
            }
        }

        // Add our joltages to the last column
        for (r, &val) in machine.joltages.iter().enumerate() {
            data[r][cols] = val as f64;
        }

        let mut matrix = Self {
            data,
            rows,
            cols,
            dependents: Vec::new(),
            independents: Vec::new(),
        };

        matrix.gaussian_elimination();
        matrix
    }

    // https://en.wikipedia.org/wiki/Gaussian_elimination
    fn gaussian_elimination(&mut self) {
        let mut pivot = 0;

        let mut col = 0;
        while pivot < self.rows && col < self.cols {
            // Find the best pivot row for this column.
            let (best_row, best_value) = self
                .data
                .iter()
                .enumerate()
                .skip(pivot)
                .map(|(r, row)| (r, row[col].abs()))
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();

            // If the best value is zero, this is a free variable.
            if best_value < EPSILON {
                self.independents.push(col);
                col += 1;
                continue;
            }

            // Swap rows and mark this column as dependent.
            self.data.swap(pivot, best_row);
            self.dependents.push(col);

            // Normalize pivot row.
            let pivot_value = self.data[pivot][col];
            for val in &mut self.data[pivot][col..=self.cols] {
                *val /= pivot_value;
            }

            // Eliminate this column in all other rows.
            for r in 0..self.rows {
                if r != pivot {
                    let factor = self.data[r][col];
                    if factor.abs() > EPSILON {
                        let pivot_row = self.data[pivot][col..=self.cols].to_vec();
                        self.data[r][col..=self.cols]
                            .iter_mut()
                            .zip(&pivot_row)
                            .for_each(|(val, &pivot_val)| {
                                *val -= factor * pivot_val;
                            });
                    }
                }
            }

            pivot += 1;
            col += 1;
        }

        // Any remaining columns are free variables
        self.independents.extend(col..self.cols);
    }

    // Check if the given values for our independent variables are valid. If so, return the total button presses.
    fn valid(&self, values: &[usize]) -> Option<usize> {
        // We start with how many times we've pressed the free variables.
        let mut total = values.iter().sum::<usize>();

        // Calculate dependent variable values based on independent variables.
        for row in 0..self.dependents.len() {
            // Calculate this dependent by subtracting the sum of the free variable pushes from the solution.
            let val = self
                .independents
                .iter()
                .enumerate()
                .fold(self.data[row][self.cols], |acc, (i, &col)| {
                    acc - self.data[row][col] * (values[i] as f64)
                });

            // We need non-negative, whole numbers for a valid solution.
            if val < -EPSILON {
                return None;
            }
            let rounded = val.round();
            if (val - rounded).abs() > EPSILON {
                return None;
            }

            total += rounded as usize;
        }

        Some(total)
    }
}

fn dfs(matrix: &Matrix, idx: usize, values: &mut [usize], min: &mut usize, max: usize) {
    // When we've assigned all independent variables, check if it's a valid solution.
    if idx == matrix.independents.len() {
        if let Some(total) = matrix.valid(values) {
            *min = (*min).min(total);
        }
        return;
    }

    // Try different values for the current independent variable.
    let total: usize = values[..idx].iter().sum();
    for val in 0..max {
        // Optimization: If we ever go above our min, we can't possibly do better.
        if total + val >= *min {
            break;
        }
        values[idx] = val;
        dfs(matrix, idx + 1, values, min, max);
    }
}

fn p2(input: &str) -> usize {
    parse(input)
        .par_bridge()
        .map(|machine| {
            let matrix = Matrix::from_machine(&machine);

            // Now we can DFS over a much smaller solution space.
            let max = *machine.joltages.iter().max().unwrap() + 1;
            let mut min = usize::MAX;
            let mut values = vec![0; matrix.independents.len()];

            dfs(&matrix, 0, &mut values, &mut min, max);

            min
        })
        .sum()
}

fn p2_z3(input: &str) -> usize {
    parse(input)
        .par_bridge()
        .map(|machine| {
            // Create the optimizer and some shared values.
            let optimizer = Optimize::new();
            let zero = Int::from_i64(0);

            // Create our joltages.
            let mut joltages = machine
                .joltages
                .iter()
                .map(|_| Int::from_i64(0))
                .collect::<Vec<_>>();

            // Create our buttons and add them as expressions to our joltages.
            let buttons = machine
                .buttons
                .iter()
                .enumerate()
                .map(|(i, button)| {
                    // Create our integer for this buttons presses and assert it be greater than zero.
                    let var = Int::fresh_const(&format!("button-{}", i));
                    optimizer.assert(&var.ge(&zero));

                    // Add this button to each of it's connect joltages.
                    button.iter().for_each(|index| {
                        joltages[*index] = &joltages[*index] + &var;
                    });
                    var
                })
                .collect::<Vec<_>>();

            // Make all of joltages sum to the button presses.
            joltages.iter().enumerate().for_each(|(i, joltage)| {
                optimizer.assert(&joltage.eq(Int::from_i64(machine.joltages[i] as i64)))
            });

            // Tell the optimizer that we are optimizing on the mimimum number of accumulated presses.
            let total_presses = buttons.iter().fold(Int::from_i64(0), |acc, x| acc + x);
            optimizer.minimize(&total_presses);

            // Presumably this should work, so ignoring error checking. But we check to do the
            // solve and then get the result and return it.
            match optimizer.check(&[]) {
                z3::SatResult::Sat => optimizer
                    .get_model()
                    .unwrap()
                    .eval(&total_presses, true)
                    .unwrap()
                    .as_i64()
                    .unwrap() as usize,
                _ => unreachable!(),
            }
        })
        .sum()
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);
    assert_eq!(solution, 486);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
    assert_eq!(solution, 17820);

    let now = Instant::now();
    let solution = p2_z3(INPUT);
    println!("p2_z3 {:?} {}", now.elapsed(), solution);
    assert_eq!(solution, 17820);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("inputs/day10-sample.txt");

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 7);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 33);
    }

    #[test]
    fn test_p2_z3() {
        assert_eq!(p2_z3(INPUT), 33);
    }
}
