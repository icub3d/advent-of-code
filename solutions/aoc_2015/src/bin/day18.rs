use std::time::Instant;

const INPUT: &'static str = include_str!("inputs/day18.txt");

pub fn p1(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    for _ in 0..100 {
        grid = step(&grid);
    }
    grid.iter().flatten().filter(|c| **c == '#').count()
}

fn step(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, cell)| {
                    let neighbors = get_neighbors(&grid, x, y);
                    match cell {
                        '#' if neighbors == 2 || neighbors == 3 => '#',
                        '.' if neighbors == 3 => '#',
                        _ => '.',
                    }
                })
                .collect()
        })
        .collect()
}

fn get_neighbors(grid: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    (y.saturating_sub(1)..=(y + 1).min(grid.len() - 1))
        .flat_map(|ny| {
            (x.saturating_sub(1)..=(x + 1).min(grid[y].len() - 1)).map(move |nx| (nx, ny))
        })
        .filter(|(nx, ny)| (*nx, *ny) != (x, y))
        .filter(|(nx, ny)| grid[*ny][*nx] == '#')
        .count()
}

pub fn p2(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let corners = vec![
        (0, 0),
        (0, grid.len() - 1),
        (grid.len() - 1, 0),
        (grid.len() - 1, grid.len() - 1),
    ];
    for _ in 0..100 {
        for corner in &corners {
            grid[corner.1][corner.0] = '#';
        }
        grid = step(&grid);
    }
    for corner in &corners {
        grid[corner.1][corner.0] = '#';
    }
    grid.iter().flatten().filter(|c| **c == '#').count()
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
