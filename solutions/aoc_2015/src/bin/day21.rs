use std::time::Instant;

#[derive(Debug)]
struct Player {
    health: i32,
    damage: i32,
    armor: i32,
}

impl Player {
    fn parse(input: &str) -> Self {
        let lines = input
            .lines()
            .map(|l| {
                let parts = l.split(' ').collect::<Vec<&str>>();
                parts[parts.len() - 1].parse::<i32>().unwrap()
            })
            .collect::<Vec<i32>>();

        Self::new(lines[0], lines[1], lines[2])
    }

    fn new(health: i32, damage: i32, armor: i32) -> Self {
        Self {
            health,
            damage,
            armor,
        }
    }

    fn beats(&self, boss: &Player) -> bool {
        self.turns_to_kill(boss) <= boss.turns_to_kill(self)
    }

    // Number of turns `self` needs to kill `target`, using integer ceiling.
    fn turns_to_kill(&self, target: &Player) -> i32 {
        let dmg_per_turn = (self.damage - target.armor).max(1);
        (target.health + dmg_per_turn - 1) / dmg_per_turn
    }
}

const WEAPONS: &str = "8 4 0
10 5 0
25 6 0
40 7 0
74 8 0";

const ARMOR: &str = "0 0 0
13 0 1
31 0 2
53 0 3
75 0 4
102 0 5";

const RINGS: &str = "0 0 0
25 1 0
50 2 0
100 3 0
20 0 1
40 0 2
80 0 3";

#[derive(Debug, Eq, PartialEq)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Item {
    fn parse(input: &str) -> Self {
        let parts = input
            .split(' ')
            .map(|p| p.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        Self {
            cost: parts[0],
            damage: parts[1],
            armor: parts[2],
        }
    }

    fn parse_all(input: &str) -> Vec<Self> {
        input.lines().map(Item::parse).collect()
    }
}

struct ItemCombinations {
    weapon: usize,
    armor: usize,
    ring1: usize,
    ring2: usize,
    first: bool,

    weapons: Vec<Item>,
    armors: Vec<Item>,
    rings: Vec<Item>,
}

impl ItemCombinations {
    fn new() -> Self {
        Self {
            weapon: 0,
            armor: 0,
            ring1: 0,
            ring2: 0,
            first: true,
            weapons: Item::parse_all(WEAPONS),
            armors: Item::parse_all(ARMOR),
            rings: Item::parse_all(RINGS),
        }
    }
}

impl Iterator for ItemCombinations {
    type Item = (i32, i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            Some((
                self.weapons[self.weapon].damage
                    + self.rings[self.ring1].damage
                    + self.rings[self.ring2].damage,
                self.armors[self.armor].armor
                    + self.rings[self.ring1].armor
                    + self.rings[self.ring2].armor,
                self.armors[self.armor].cost
                    + self.weapons[self.weapon].cost
                    + self.rings[self.ring1].cost
                    + self.rings[self.ring2].cost,
            ))
        } else {
            self.ring2 += 1;
            if self.ring2 >= self.rings.len() {
                self.ring1 += 1;
                self.ring2 = 0;
            }

            if self.ring1 == self.ring2 && self.ring1 != 0 {
                self.ring2 += 1;
                if self.ring2 >= self.rings.len() {
                    self.ring1 += 1;
                    self.ring2 = 0;
                }
            }

            if self.ring1 >= self.rings.len() {
                self.armor += 1;
                self.ring1 = 0;
            }

            if self.armor >= self.armors.len() {
                self.armor = 0;
                self.weapon += 1;
            }

            if self.weapon >= self.weapons.len() {
                return None;
            }
            Some((
                self.weapons[self.weapon].damage
                    + self.rings[self.ring1].damage
                    + self.rings[self.ring2].damage,
                self.armors[self.armor].armor
                    + self.rings[self.ring1].armor
                    + self.rings[self.ring2].armor,
                self.armors[self.armor].cost
                    + self.weapons[self.weapon].cost
                    + self.rings[self.ring1].cost
                    + self.rings[self.ring2].cost,
            ))
        }
    }
}

const INPUT: &str = include_str!("inputs/day21.txt");

fn p1(input: &str) -> i32 {
    let boss = Player::parse(input);
    ItemCombinations::new()
        .map(|(damage, armor, cost)| {
            let attacker = Player::new(100, damage, armor);
            (attacker, cost)
        })
        .filter(|(attacker, _)| attacker.beats(&boss))
        .map(|(_, cost)| cost)
        .min()
        .unwrap()
}

fn p2(input: &str) -> i32 {
    let boss = Player::parse(input);
    ItemCombinations::new()
        .map(|(damage, armor, cost)| {
            let attacker = Player::new(100, damage, armor);
            (attacker, cost)
        })
        .filter(|(attacker, _)| boss.beats(attacker))
        .map(|(_, cost)| cost)
        .max()
        .unwrap()
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
