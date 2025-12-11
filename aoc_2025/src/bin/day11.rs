use std::time::Instant;

use cached::proc_macro::cached;
use rustc_hash::FxHashMap;

const INPUT: &str = include_str!("inputs/day11.txt");

// Turn our input into a mapping of devices to their neighbors.
fn parse(input: &str) -> FxHashMap<&str, Vec<&str>> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (name, outputs) = l.split_once(": ").unwrap();
            (name, outputs.split_whitespace().collect())
        })
        .collect()
}

// Use dynamic programming to find the count of paths from the current node to the end. Each nodes
// path is the sum of the paths of it's neighbors to the end.
fn dp<'a>(
    neighbors: &FxHashMap<&'a str, Vec<&'a str>>,
    paths: &mut FxHashMap<&'a str, usize>,
    cur: &'a str,
    end: &str,
) -> usize {
    // If we've counted the paths for this node before, no need to do it again.
    if let Some(p) = paths.get(cur) {
        return *p;
    }
    // If we've reached the end, we've found one path.
    if cur == end {
        return 1;
    }

    // For some reason some of them have no neighbors? Regardless sum all the paths of our
    // neighbors and that will be our total.
    let total = match neighbors.get(cur) {
        Some(vv) => vv.iter().map(|next| dp(neighbors, paths, next, end)).sum(),
        None => 0,
    };
    paths.insert(cur, total);

    total
}

fn p1(input: &str) -> usize {
    // Now that we can efficiently find a path, just run the dp.
    let edges = parse(input);
    dp(&edges, &mut FxHashMap::default(), "you", "out")
}

fn p2(input: &str) -> usize {
    // Note that the puzzle tells us that the graph is a DAG (goes one direction and doesn't cycle
    // back). This means that there must be one of two paths between the "interesting" nodes.
    //
    //   srv -> dac -> fft -> out
    //   srv -> fft -> dac -> out
    //
    // Each options total paths will be the product of the paths between each. One won't have a
    // solution and will return zero so we can just sum them up.
    let edges = parse(input);
    //   srv -> dac -> fft -> out
    (dp(&edges, &mut FxHashMap::default(), "svr", "dac")
        * dp(&edges, &mut FxHashMap::default(), "dac", "fft")
        * dp(&edges, &mut FxHashMap::default(), "fft", "out"))
    //   srv -> fft -> dac -> out
        + (dp(&edges, &mut FxHashMap::default(), "svr", "fft")
            * dp(&edges, &mut FxHashMap::default(), "fft", "dac")
            * dp(&edges, &mut FxHashMap::default(), "dac", "out"))
}

#[cached(
    key = "(&'static str, bool, bool)",
    convert = "{ (cur, seen_dac, seen_fft) }"
)]
fn dp_paulson(
    neighbors: &FxHashMap<&'static str, Vec<&'static str>>,
    cur: &'static str,
    seen_dac: bool,
    seen_fft: bool,
) -> usize {
    if cur == "out" {
        if seen_dac && seen_fft {
            return 1;
        }
        return 0;
    }

    let seen_dac = seen_dac || cur == "dac";
    let seen_fft = seen_fft || cur == "fft";

    match neighbors.get(cur) {
        Some(vv) => vv
            .iter()
            .map(|next| dp_paulson(neighbors, next, seen_dac, seen_fft))
            .sum(),
        None => 0,
    }
}

fn p2_paulson(input: &'static str) -> usize {
    let edges = parse(input);
    dp_paulson(&edges, "svr", false, false)
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2_paulson(INPUT);
    println!("p2_paulson {:?} {}", now.elapsed(), solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("inputs/day11-sample.txt");

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 5);
    }

    const INPUT2: &str = include_str!("inputs/day11-sample2.txt");
    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT2), 2);
    }
}
