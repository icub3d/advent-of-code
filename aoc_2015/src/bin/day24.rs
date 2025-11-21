use std::time::Instant;

use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day24.txt");

fn p1(input: &str) -> usize {
    let mut packages = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<usize>>();
    packages.sort_unstable_by(|a, b| b.cmp(a));
    let total = packages.iter().sum::<usize>();
    let group = total / 3;

    for i in 5..=packages.len() / 3 {
        let solution = packages
            .iter()
            .combinations(i)
            .filter(|pp| pp.iter().map(|&&x| x).sum::<usize>() == group)
            .filter(|pp| can_partition_remaining(&packages, pp, 2, group))
            .map(|pp| pp.iter().map(|&&x| x).product())
            .min();

        match solution {
            Some(v) => return v,
            _ => continue,
        }
    }

    0
}

fn p2(input: &str) -> usize {
    let mut packages = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<usize>>();
    packages.sort_unstable_by(|a, b| b.cmp(a));
    let total = packages.iter().sum::<usize>();
    let group = total / 4;

    for i in 4..=packages.len() / 4 {
        let solution = packages
            .iter()
            .combinations(i)
            .filter(|pp| pp.iter().map(|&&x| x).sum::<usize>() == group)
            .filter(|pp| can_partition_remaining(&packages, pp, 3, group))
            .map(|pp| pp.iter().map(|&&x| x).product())
            .min();

        match solution {
            Some(v) => return v,
            _ => continue,
        }
    }

    0
}

fn can_partition_remaining(
    weights: &[usize],
    first: &Vec<&usize>,
    remaining: usize,
    target: usize,
) -> bool {
    let mut used = vec![false; weights.len()];
    let weights = weights
        .iter()
        .filter(|w| !first.contains(w))
        .copied()
        .collect::<Vec<usize>>();
    can_partition_k(&weights, &mut used, remaining, target)
    // the algorithm's for k=2 and k=3 is actually not faster because of the
    // size of the input. k=3 takes about 4 million iterations. dp+prune only
    // takes a dozen or so.
    //
    // match remaining {
    //     2 => can_partition_2(&weights),
    //     3 => can_partition_3(&weights),
    //     _ => can_partition_k(&weights, &mut used, remaining, target),
    // }
}

pub fn can_partition_2(weights: &[usize]) -> bool {
    let total: usize = weights.iter().sum();
    if !total.is_multiple_of(2) {
        return false;
    }
    let target = total / 2;

    // dp[x] = true if we can make sum x using some subset
    let mut dp = vec![false; target + 1];
    dp[0] = true;

    for &a in weights {
        // iterate backwards to avoid overwriting states we still need
        for x in (a..=target).rev() {
            if dp[x - a] {
                dp[x] = true;
            }
        }
    }

    dp[target]
}

pub fn can_partition_3(weights: &[usize]) -> bool {
    let total: usize = weights.iter().sum();
    if !total.is_multiple_of(3) {
        return false;
    }
    let target = total / 3;

    // Flatten 2D DP into 1D: dp[x * (target+1) + y]
    let size = (target + 1) * (target + 1);
    let mut dp = vec![false; size];
    dp[0] = true; // dp[0][0] = true

    for &a in weights {
        // Iterate backwards so we don't overwrite needed states
        for x in (0..=target).rev() {
            for y in (0..=target).rev() {
                let idx = x * (target + 1) + y;
                if !dp[idx] {
                    continue;
                }
                // Place 'a' in subset 1
                if x + a <= target {
                    dp[(x + a) * (target + 1) + y] = true;
                }
                // Place 'a' in subset 2
                if y + a <= target {
                    dp[x * (target + 1) + (y + a)] = true;
                }
                // Place in subset 3: implicit
            }
        }
    }

    dp[target * (target + 1) + target]
}

fn can_partition_k(packages: &[usize], used: &mut [bool], k: usize, target: usize) -> bool {
    if k == 0 {
        return true;
    }

    // Check to make sure we can partition based on sum of remaining weights.
    let total_remaining = packages
        .iter()
        .enumerate()
        .filter(|(i, _)| !used[*i])
        .map(|(_, &w)| w)
        .sum::<usize>();
    if total_remaining != target * k {
        return false;
    }

    // Now that we know the sum is correct, if there's only one group left, we
    // can take it. Also check for too many groups.
    if k == 1 {
        return true;
    } else if k > packages.len() {
        return false;
    }

    dfs(packages, used, 0, 0, k, target)
}

fn dfs(
    weights: &[usize],
    used: &mut [bool],
    start: usize,
    total: usize,
    groups: usize,
    target: usize,
) -> bool {
    // If we hit the target, recurse to build the next group. If we exceed it,
    // backtrack.
    if total == target {
        return can_partition_k(weights, used, groups - 1, target);
    } else if total > target {
        return false;
    }

    // Try adding each unused weight to the current group and see if it works.
    //
    // We don't check duplicates here, but in a generalized solution you could
    // keep track of a previous weight and skip any weights you've seen before
    // (assuming it's sorted, if not, you could use a HashSet).
    for i in start..weights.len() {
        // Skip used weights and weights that would exceed the target.
        if used[i] {
            continue;
        }
        let w = weights[i];
        if total + w > target {
            continue;
        }

        // Mark as used and recurse. If it works, return true. If not, backtrack
        // and try the next weight.
        used[i] = true;
        if dfs(weights, used, i + 1, total + w, groups, target) {
            used[i] = false; // restore before early return
            return true;
        }
        used[i] = false;
    }

    // No combination worked, return false.
    false
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
