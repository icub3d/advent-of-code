use std::time::Instant;

use dinglebit_combinatorics::Combination;

const INPUT: &'static str = include_str!("inputs/day24.txt");

pub fn p1(input: &str) -> usize {
    let packages = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<usize>>();
    let total = packages.iter().sum::<usize>();
    let group = total / 3;
    println!("total: {:?}", total);
    println!("group_total: {:?}", group);
    println!("weights: ({}) {:?}", packages.len(), packages);

    for i in 5..=packages.len() / 3 {
        let solution = Combination::new(packages.len(), i)
            .map(|c| c.iter().map(|j| packages[*j]).collect::<Vec<_>>())
            .filter(|pp| pp.iter().sum::<usize>() == group)
            .map(|pp| pp.iter().product())
            .min();

        match solution {
            Some(v) => return v,
            _ => continue,
        }
    }

    0
}

pub fn p2(input: &str) -> usize {
    let packages = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<usize>>();
    let total = packages.iter().sum::<usize>();
    let group = total / 4;

    for i in 4..=packages.len() / 4 {
        let solution = Combination::new(packages.len(), i)
            .map(|c| c.iter().map(|j| packages[*j]).collect::<Vec<_>>())
            .filter(|pp| pp.iter().sum::<usize>() == group)
            .map(|pp| pp.iter().product())
            .min();

        match solution {
            Some(v) => return v,
            _ => continue,
        }
    }

    0
}

pub fn p1_rec(input: &str) -> usize {
    let mut packages = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<usize>>();
    packages.sort_by(|a, b| b.cmp(a));
    let total = packages.iter().sum::<usize>();
    let group = total / 3;

    p1_helper(&packages, group, 0, 1, 0)
}

pub fn p1_helper(packages: &[usize], req: usize, index: usize, qe: usize, weight: usize) -> usize {
    if req == weight {
        return qe;
    }

    if index >= packages.len() || weight > req {
        return usize::MAX;
    }

    let left = p1_helper(
        packages,
        req,
        index + 1,
        qe * packages[index],
        weight + packages[index],
    );
    let right = p1_helper(packages, req, index + 1, qe, weight);
    if left == usize::MAX {
        right
    } else if left == usize::MAX {
        left
    } else {
        left.min(right)
    }
}

pub fn solve() -> anyhow::Result<()> {
    let now = Instant::now();
    println!("p1_rec: {} ({:?})", p1_rec(INPUT), now.elapsed());
    let now = Instant::now();
    println!("p1: {} ({:?})", p1(INPUT), now.elapsed());
    let now = Instant::now();
    println!("p2: {} ({:?})", p2(INPUT), now.elapsed());
    Ok(())
}
