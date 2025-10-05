use std::time::Instant;

const INPUT: &'static str = include_str!("inputs/day22.txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    health: i32,
    mana: i32,
    boss_health: i32,
    boss_damage: i32,
    shield: usize,
    poison: usize,
    recharge: usize,
}

impl State {
    fn new(input: &str) -> Self {
        let mut parts = input.lines();
        let health = parts.next().unwrap().split(' ').collect::<Vec<&str>>();
        let damage = parts.next().unwrap().split(' ').collect::<Vec<&str>>();
        Self {
            health: 50,
            mana: 500,
            boss_health: health[2].parse::<i32>().unwrap(),
            boss_damage: damage[1].parse::<i32>().unwrap(),
            shield: 0,
            poison: 0,
            recharge: 0,
        }
    }

    fn apply_effects(&mut self) {
        if self.poison > 0 {
            self.boss_health -= 3;
            self.poison -= 1;
        }
        if self.recharge > 0 {
            self.mana += 101;
            self.recharge -= 1;
        }
        if self.shield > 0 {
            self.shield -= 1;
        }
    }

    fn neighbors(&self, hard: bool) -> Vec<(Self, usize)> {
        let mut neighbors = Vec::new();

        // Apply effects and check for my own death.
        let mut next = self.clone();
        next.apply_effects();
        if hard {
            next.health -= 1;
        }
        if next.health <= 0 {
            return vec![];
        }

        // If boss dies before my spell, we just return it.
        if next.boss_health <= 0 {
            return vec![(next, 0)];
        }

        // At this point, we cast a spell.
        if next.poison == 0 && next.mana >= 173 {
            let mut spell = next.clone();
            spell.mana -= 173;
            spell.poison = 6;
            neighbors.push((spell, 173));
        }
        if next.recharge == 0 && next.mana >= 229 {
            let mut spell = next.clone();
            spell.mana -= 229;
            spell.recharge = 5;
            neighbors.push((spell, 229));
        }
        if next.shield == 0 && next.mana >= 113 {
            let mut spell = next.clone();
            spell.mana -= 113;
            spell.shield = 6;
            neighbors.push((spell, 113));
        }
        if next.mana >= 53 {
            let mut spell = next.clone();
            spell.mana -= 53;
            spell.boss_health -= 4;
            neighbors.push((spell, 53));
        }
        if next.mana >= 73 {
            let mut spell = next.clone();
            spell.mana -= 73;
            spell.boss_health -= 2;
            spell.health += 2;
            neighbors.push((spell, 73));
        }

        // Apply effects and then boss damage if boss isn't dead.
        neighbors.iter_mut().for_each(|(n, _)| {
            n.apply_effects();
            if n.boss_health <= 0 {
                return;
            }
            n.health -= match n.shield > 0 {
                true => (n.boss_damage - 7).max(1),
                false => n.boss_damage,
            };
        });

        neighbors
    }
}

pub fn p1(input: &str) -> usize {
    let start = State::new(input);

    let (_, cost) = pathfinding::directed::dijkstra::dijkstra(
        &start,
        |s| s.neighbors(false),
        |s| s.health > 0 && s.boss_health <= 0,
    )
    .unwrap();
    cost
}

pub fn p2(input: &str) -> usize {
    let start = State::new(input);

    let (_, cost) = pathfinding::directed::dijkstra::dijkstra(
        &start,
        |s| s.neighbors(true),
        |s| s.health > 0 && s.boss_health <= 0,
    )
    .unwrap();
    cost
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
