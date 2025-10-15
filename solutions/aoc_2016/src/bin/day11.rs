use std::{collections::VecDeque, hash::Hash, time::Instant};

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

const INPUT: &str = include_str!("inputs/day11.txt");

#[derive(Clone, Debug, Eq)]
struct State {
    elevator: usize,
    equipment: Vec<(usize, usize)>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        let mut se = self.equipment.clone();
        se.sort();
        let mut oe = other.equipment.clone();
        oe.sort();
        self.elevator == other.elevator && se == oe
    }
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.elevator);
        let mut equipment = self.equipment.clone();
        equipment.sort();
        equipment.iter().for_each(|&(chip, generator)| {
            state.write_usize(chip);
            state.write_usize(generator);
        })
    }
}

impl State {
    fn done(&self) -> bool {
        self.equipment
            .iter()
            .all(|&(chip, generator)| chip == 3 && generator == 3)
    }

    fn valid(&self) -> bool {
        // Get the floors that have a generator
        let floors = self
            .equipment
            .iter()
            .map(|&(_, g)| g)
            .sorted()
            .unique()
            .collect::<Vec<usize>>();

        !self
            .equipment
            .iter()
            .any(|(chip, generator)| floors.contains(chip) && generator != chip)
    }

    fn successors(&self) -> Vec<State> {
        let mut successors = Vec::new();

        let on_this_floor = self
            .equipment
            .iter()
            .enumerate()
            .flat_map(|(i, &(chip, generator))| {
                let mut v = vec![];
                if chip == self.elevator {
                    v.push((i, true));
                }
                if generator == self.elevator {
                    v.push((i, false));
                }
                v
            })
            .collect::<Vec<(usize, bool)>>();

        // Go up or down on elevator.
        for new_floor in [self.elevator + 1, self.elevator.saturating_sub(1)] {
            if new_floor > 3 || new_floor == self.elevator {
                continue;
            }
            // Add for single
            on_this_floor.iter().for_each(|&(floor, is_chip)| {
                let mut new_state = self.clone();
                new_state.elevator = new_floor;
                if is_chip {
                    new_state.equipment[floor].0 = new_floor;
                } else {
                    new_state.equipment[floor].1 = new_floor;
                }
                if new_state.valid() {
                    successors.push(new_state);
                }
            });

            // Add combinations of two
            if on_this_floor.len() < 2 {
                continue;
            }

            on_this_floor.iter().tuple_combinations().for_each(
                |(&(floor1, is_chip1), &(floor2, is_chip2))| {
                    let mut new_state = self.clone();
                    new_state.elevator = new_floor;
                    if is_chip1 {
                        new_state.equipment[floor1].0 = new_floor;
                    } else {
                        new_state.equipment[floor1].1 = new_floor;
                    }
                    if is_chip2 {
                        new_state.equipment[floor2].0 = new_floor;
                    } else {
                        new_state.equipment[floor2].1 = new_floor;
                    }
                    if new_state.valid() {
                        successors.push(new_state);
                    }
                },
            );
        }
        // Move one or two on the current floor.
        // Check if valid.
        //
        successors
    }
}

type Input<'a> = State;

fn parse_input(input: &'_ str) -> Input<'_> {
    let mut names: FxHashMap<&str, usize> = FxHashMap::default();
    let mut equipment: Vec<(usize, usize)> = Vec::new();
    input.lines().enumerate().for_each(|(floor, l)| {
        l.split_whitespace()
            .map(|c| c.trim_end_matches('.').trim_end_matches(','))
            .tuple_windows()
            .filter(|(_, b)| *b == "generator" || *b == "microchip")
            .for_each(
                |(name, typ)| match names.get(name.trim_end_matches("-compatible")) {
                    Some(i) => match typ {
                        "generator" => equipment[*i].1 = floor,
                        "microchip" => equipment[*i].0 = floor,
                        _ => panic!("invalid equipment {} {}", name, typ),
                    },
                    None => {
                        match typ {
                            "generator" => equipment.push((0, floor)),
                            "microchip" => equipment.push((floor, 0)),
                            _ => panic!("invalid equipment {} {}", name, typ),
                        };
                        names.insert(name.trim_end_matches("-compatible"), equipment.len() - 1);
                    }
                },
            )
    });

    State {
        elevator: 0,
        equipment,
    }
}

fn bfs(input: &Input) -> usize {
    let mut visited: FxHashSet<State> = FxHashSet::default();
    let mut frontier: VecDeque<(State, usize)> = VecDeque::new();

    visited.insert(input.clone());
    frontier.push_back((input.clone(), 0));

    while let Some((state, count)) = frontier.pop_front() {
        if state.done() {
            return count;
        }

        for successor in state.successors() {
            if visited.insert(successor.clone()) {
                frontier.push_back((successor.clone(), count + 1));
            }
        }
    }

    panic!("not found!")
}

fn p1(input: &Input) -> usize {
    bfs(input)
}

fn p2(input: &Input) -> usize {
    let mut input = input.clone();
    input.equipment.push((0, 0));
    input.equipment.push((0, 0));
    bfs(&input)
}

fn main() {
    let now = Instant::now();
    let input = parse_input(INPUT);
    let solution = p1(&input);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(&input);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
