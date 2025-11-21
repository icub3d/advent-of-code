use std::{collections::HashSet, time::Instant};

const INPUT: &str = include_str!("inputs/day19.txt");

fn p1(input: &str) -> usize {
    let (replacements, molecule) = input.split_once("\n\n").unwrap();
    let replacements = replacements
        .lines()
        .map(|l| l.split_once(" => ").unwrap())
        .collect::<Vec<(&str, &str)>>();

    let mut new_molecules = HashSet::new();
    for replacement in replacements {
        for (i, _) in molecule.match_indices(replacement.0) {
            let mut new_molecule = molecule.to_string();
            new_molecule.replace_range(i..i + replacement.0.len(), replacement.1);
            new_molecules.insert(new_molecule);
        }
    }

    new_molecules.len()
}

fn p2(input: &str) -> usize {
    let (_, molecule) = input.split_once("\n\n").unwrap();

    let molecule_count = molecule.chars().filter(|c| c.is_ascii_uppercase()).count();
    let rn = molecule.matches("Rn").count();
    let ar = molecule.matches("Ar").count();
    let y = molecule.matches("Y").count();
    molecule_count - rn - ar - y * 2 - 1
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
