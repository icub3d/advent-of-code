use std::{collections::VecDeque, time::Instant};

use md5::{Digest, Md5};
use rustc_hash::FxHashSet;

const INPUT: &str = include_str!("inputs/day17.txt");

type Input<'a> = &'a str;

fn parse_input(input: &'_ str) -> Input<'_> {
    input.trim()
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    x: usize,
    y: usize,
    passcode: Vec<u8>,
    directions: Vec<u8>,
}

const OPEN: [u8; 5] = [b'b', b'c', b'd', b'e', b'f'];

impl State {
    fn new(x: usize, y: usize, passcode: &str) -> Self {
        Self {
            x,
            y,
            directions: Vec::new(),
            passcode: passcode.bytes().collect(),
        }
    }

    fn neighbors(&self) -> Vec<Self> {
        // Get the md5 of the state.
        let mut hasher = Md5::new();
        hasher.update(&self.passcode);
        hasher.update(&self.directions);
        let hash = format!("{:x}", hasher.finalize())
            .bytes()
            .take(4)
            .collect::<Vec<_>>();

        let mut neighbors = Vec::new();

        // Up
        if self.y > 0 && OPEN.contains(&hash[0]) {
            let mut directions = self.directions.clone();
            directions.push(b'U');
            neighbors.push(Self {
                x: self.x,
                y: self.y - 1,
                passcode: self.passcode.clone(),
                directions,
            });
        }

        // Down
        if self.y < 3 && OPEN.contains(&hash[1]) {
            let mut directions = self.directions.clone();
            directions.push(b'D');
            neighbors.push(Self {
                x: self.x,
                y: self.y + 1,
                passcode: self.passcode.clone(),
                directions,
            });
        }

        // Left
        if self.x > 0 && OPEN.contains(&hash[2]) {
            let mut directions = self.directions.clone();
            directions.push(b'L');
            neighbors.push(Self {
                x: self.x - 1,
                y: self.y,
                passcode: self.passcode.clone(),
                directions,
            });
        }

        // Right
        if self.x < 3 && OPEN.contains(&hash[3]) {
            let mut directions = self.directions.clone();
            directions.push(b'R');
            neighbors.push(Self {
                x: self.x + 1,
                y: self.y,
                passcode: self.passcode.clone(),
                directions,
            });
        }

        neighbors
    }
}

fn bfs(start: &State) -> String {
    let mut frontier = VecDeque::new();
    frontier.push_back(start.clone());

    let mut visited = FxHashSet::default();
    visited.insert(start.clone());

    while let Some(state) = frontier.pop_front() {
        if state.x == 3 && state.y == 3 {
            return state.directions.iter().map(|&b| b as char).collect();
        }

        // state
        //     .neighbors()
        //     .iter()
        //     .filter(|&n| visited.insert(n.clone()))
        //     .for_each(|n| frontier.push_back(n.clone()));

        for neighbor in state.neighbors() {
            if visited.insert(neighbor.clone()) {
                frontier.push_back(neighbor);
            }
        }
    }

    unreachable!();
}

fn p1(input: &Input) -> String {
    let state = State::new(0, 0, input);
    bfs(&state)
}

fn bfs_all(start: &State) -> usize {
    let mut frontier = VecDeque::new();
    frontier.push_back(start.clone());

    let mut visited = FxHashSet::default();
    visited.insert(start.clone());

    let mut max = 0;
    while let Some(state) = frontier.pop_front() {
        if state.x == 3 && state.y == 3 {
            max = max.max(state.directions.len());
            continue;
        }

        for neighbor in state.neighbors() {
            if visited.insert(neighbor.clone()) {
                frontier.push_back(neighbor);
            }
        }
    }

    max
}

fn p2(input: &Input) -> usize {
    bfs_all(&State::new(0, 0, input))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let state = State::new(0, 0, "ihgpwlah");
        assert_eq!(bfs(&state), "DDRRRD");

        let state = State::new(0, 0, "ulqzkmiv");
        assert_eq!(bfs(&state), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }
}
