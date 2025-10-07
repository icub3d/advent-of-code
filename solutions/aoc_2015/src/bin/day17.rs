use std::time::Instant;

const INPUT: &str = include_str!("inputs/day17.txt");

fn p1(input: &str) -> usize {
    let containers = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    p1_helper(&containers, 150)
}

fn p1_helper(containers: &[isize], remaining: isize) -> usize {
    if remaining == 0 {
        return 1;
    }

    containers
        .iter()
        .enumerate()
        .filter(|(_, container)| **container <= remaining)
        .map(|(i, container)| p1_helper(&containers[i + 1..], remaining - container))
        .sum()
}

fn p2(input: &str) -> usize {
    let containers = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut min_size = usize::MAX;
    let mut min_count = 0;
    let mut used = vec![];
    p2_helper(&containers, 150, &mut min_size, &mut min_count, &mut used);
    min_count
}

fn p2_helper(
    containers: &[isize],
    remaining: isize,
    min_size: &mut usize,
    min_count: &mut usize,
    used: &mut Vec<isize>,
) {
    if remaining == 0 && *min_size > used.len() {
        // We found a smaller min
        *min_count = 1;
        *min_size = used.len();
        return;
    } else if remaining == 0 && *min_size == used.len() {
        // We found another min.
        *min_count += 1;
        return;
    }

    containers
        .iter()
        .enumerate()
        .filter(|(_, container)| **container <= remaining)
        .for_each(|(i, container)| {
            used.push(*container);
            p2_helper(
                &containers[i + 1..],
                remaining - container,
                min_size,
                min_count,
                used,
            );
            used.pop();
        });
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
