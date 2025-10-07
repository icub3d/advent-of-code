use std::time::Instant;

const INPUT: &str = include_str!("inputs/day22.txt");

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

// Spell constants to avoid magic numbers and to clarify intent.
const COST_MISSILE: i32 = 53;
const COST_DRAIN: i32 = 73;
const COST_SHIELD: i32 = 113;
const COST_POISON: i32 = 173;
const COST_RECHARGE: i32 = 229;

const DURATION_SHIELD: usize = 6;
const DURATION_POISON: usize = 6;
const DURATION_RECHARGE: usize = 5;

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

    fn neighbors(&self, hard: bool) -> Vec<(Self, i32)> {
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

        // At this point, we cast a spell. Check availability and costs.
        if next.poison == 0 && next.mana >= COST_POISON {
            let mut spell = next.clone();
            spell.mana -= COST_POISON;
            spell.poison = DURATION_POISON;
            neighbors.push((spell, COST_POISON));
        }
        if next.recharge == 0 && next.mana >= COST_RECHARGE {
            let mut spell = next.clone();
            spell.mana -= COST_RECHARGE;
            spell.recharge = DURATION_RECHARGE;
            neighbors.push((spell, COST_RECHARGE));
        }
        if next.shield == 0 && next.mana >= COST_SHIELD {
            let mut spell = next.clone();
            spell.mana -= COST_SHIELD;
            spell.shield = DURATION_SHIELD;
            neighbors.push((spell, COST_SHIELD));
        }
        if next.mana >= COST_MISSILE {
            let mut spell = next.clone();
            spell.mana -= COST_MISSILE;
            spell.boss_health -= 4;
            neighbors.push((spell, COST_MISSILE));
        }
        if next.mana >= COST_DRAIN {
            let mut spell = next.clone();
            spell.mana -= COST_DRAIN;
            spell.boss_health -= 2;
            spell.health += 2;
            neighbors.push((spell, COST_DRAIN));
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

fn p1(input: &str) -> i32 {
    let start = State::new(input);

    let (_, cost) = pathfinding::directed::dijkstra::dijkstra(
        &start,
        |s| s.neighbors(false),
        |s| s.health > 0 && s.boss_health <= 0,
    )
    .unwrap();
    cost
}

fn p2(input: &str) -> i32 {
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
