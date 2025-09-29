use std::{collections::HashSet, time::Instant};

const INPUT: &'static str = include_str!("inputs/day19.txt");

pub fn p1(input: &str) -> usize {
    let (replacements, molecule) = input.split_once("\n\n").unwrap();
    let replacements = replacements.lines().map(|l| l.split_once(" => ").unwrap()).collect::<Vec<(&str, &str)>>();

    let mut new_molecules = HashSet::new();
    for replacement in replacements {
        for (i, _) in molecule.match_indices(replacement.0) {
            let mut new_molecule = molecule.to_string();
            new_molecule.replace_range(i..i+replacement.0.len(), replacement.1);
            new_molecules.insert(new_molecule);
        }
    }

    new_molecules.len()
}

pub fn p2(input: &str) -> usize {
    let (_, molecule) = input.split_once("\n\n").unwrap();

    let molecule_count = molecule.chars().filter(|c| c.is_ascii_uppercase()).count();
    let rn = molecule.matches("Rn").count();
    let ar = molecule.matches("Ar").count();
    let y = molecule.matches("Y").count();
    molecule_count - rn - ar - y*2 - 1
}

pub fn solve() -> anyhow::Result<()> {
    let now = Instant::now();
    println!("p1: {} ({:?})", p1(INPUT), now.elapsed());
    let now = Instant::now();
    println!("p2: {} ({:?})", p2(INPUT), now.elapsed());
    Ok(())
}
