use std::{collections::VecDeque, hash::Hash, time::Instant};

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

const INPUT: &str = include_str!("inputs/day11.txt");
const FLOOR_COUNT: usize = 4;
const TOP_FLOOR: usize = FLOOR_COUNT - 1;

#[derive(Clone, Debug, Eq)]
struct State {
    elevator: usize,
    equipment: Vec<(usize, usize)>,
}

// States where the equipment is the same but in different orders are equivalent.
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.elevator == other.elevator && self.equipment == other.equipment
    }
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.elevator);
        self.equipment.iter().for_each(|&(chip, generator)| {
            state.write_usize(chip);
            state.write_usize(generator);
        })
    }
}

impl State {
    fn normalize(&mut self) {
        self.equipment.sort_unstable();
    }

    fn done(&self) -> bool {
        self.equipment
            .iter()
            .all(|&(chip, generator)| chip == TOP_FLOOR && generator == TOP_FLOOR)
    }

    fn valid(&self) -> bool {
        let mut generators = [0usize; FLOOR_COUNT];
        self.equipment
            .iter()
            .for_each(|&(_, generator)| generators[generator] += 1);

        self.equipment
            .iter()
            .all(|&(chip, generator)| generator == chip || generators[chip] == 0)
    }

    fn has_lower_items(&self) -> bool {
        self.equipment
            .iter()
            .any(|&(chip, generator)| chip < self.elevator || generator < self.elevator)
    }

    fn successors(&self) -> Vec<State> {
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

        let mut up_single = Vec::new();
        let mut up_double = Vec::new();
        let mut down_single = Vec::new();
        let mut down_double = Vec::new();

        let try_move = |items: &[(usize, bool)], new_floor: usize| -> Option<State> {
            let mut new_state = self.clone();
            new_state.elevator = new_floor;
            items.iter().for_each(|&(idx, is_chip)| {
                if is_chip {
                    new_state.equipment[idx].0 = new_floor;
                } else {
                    new_state.equipment[idx].1 = new_floor;
                }
            });
            new_state.normalize();
            if new_state.valid() {
                Some(new_state)
            } else {
                None
            }
        };

        if self.elevator < TOP_FLOOR {
            let target = self.elevator + 1;
            for &item in &on_this_floor {
                if let Some(state) = try_move(&[item], target) {
                    if up_single.iter().any(|s| s == &state) {
                        continue;
                    }
                    up_single.push(state);
                }
            }
            for (&item1, &item2) in on_this_floor.iter().tuple_combinations() {
                if let Some(state) = try_move(&[item1, item2], target) {
                    if up_double.iter().any(|s| s == &state) {
                        continue;
                    }
                    up_double.push(state);
                }
            }
        }

        if self.elevator > 0 && self.has_lower_items() {
            let target = self.elevator - 1;
            for &item in &on_this_floor {
                if let Some(state) = try_move(&[item], target) {
                    if down_single.iter().any(|s| s == &state) {
                        continue;
                    }
                    down_single.push(state);
                }
            }
            for (&item1, &item2) in on_this_floor.iter().tuple_combinations() {
                if let Some(state) = try_move(&[item1, item2], target) {
                    if down_double.iter().any(|s| s == &state) {
                        continue;
                    }
                    down_double.push(state);
                }
            }
        }

        let mut successors = Vec::new();
        if !up_double.is_empty() {
            successors.extend(up_double);
        } else {
            successors.extend(up_single);
        }

        if !down_single.is_empty() {
            successors.extend(down_single);
        } else {
            successors.extend(down_double);
        }

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
            // We are looking for pairs like "hydrogen-compatible microchip" or "lithium generator"
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

    let mut state = State {
        elevator: 0,
        equipment,
    };
    state.normalize();
    state
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
                frontier.push_back((successor, count + 1));
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
    input.normalize();
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
