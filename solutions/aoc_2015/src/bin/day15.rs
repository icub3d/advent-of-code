use std::time::Instant;

const INPUT: &'static str = include_str!("inputs/day15.txt");

#[derive(Debug, Copy, Clone)]
struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

impl Ingredient {
    fn parse(line: &str) -> Self {
        let parts = line
            .split_ascii_whitespace()
            .map(|p| p.trim_end_matches(','))
            .collect::<Vec<&str>>();
        Self {
            capacity: parts[2].parse().unwrap(),
            durability: parts[4].parse().unwrap(),
            flavor: parts[6].parse().unwrap(),
            texture: parts[8].parse().unwrap(),
            calories: parts[10].parse().unwrap(),
        }
    }

    fn score(&self, teaspoons: isize) -> Self {
        Self {
            capacity: self.capacity * teaspoons,
            durability: self.durability * teaspoons,
            flavor: self.flavor * teaspoons,
            texture: self.texture * teaspoons,
            calories: self.calories * teaspoons,
        }
    }
}

pub fn p1(input: &str) -> isize {
    let ingredients = input
        .lines()
        .map(Ingredient::parse)
        .collect::<Vec<Ingredient>>();
    calc_scores(100, &ingredients, &mut Vec::new(), false)
}

fn calc_scores(
    remaining: isize,
    ingredients: &[Ingredient],
    scores: &mut Vec<Ingredient>,
    check_calories: bool,
) -> isize {
    if ingredients.len() == 1 {
        let score = ingredients[0].score(remaining);
        scores.push(score);

        let capacity = scores.iter().map(|s| s.capacity).sum::<isize>();
        let durability = scores.iter().map(|s| s.durability).sum::<isize>();
        let texture = scores.iter().map(|s| s.texture).sum::<isize>();
        let flavor = scores.iter().map(|s| s.flavor).sum::<isize>();
        let calories = scores.iter().map(|s| s.calories).sum::<isize>();

        scores.pop();

        if capacity <= 0
            || durability <= 0
            || flavor <= 0
            || texture <= 0
            || (check_calories && calories != 500)
        {
            return -1;
        }
        return capacity * durability * texture * flavor;
    }

    (0..=remaining)
        .map(|x| {
            let score = ingredients[0].score(x);
            scores.push(score);
            let score = calc_scores(remaining - x, &ingredients[1..], scores, check_calories);
            scores.pop();
            score
        })
        .max()
        .unwrap()
}

pub fn p2(input: &str) -> isize {
    let ingredients = input
        .lines()
        .map(Ingredient::parse)
        .collect::<Vec<Ingredient>>();

    calc_scores(100, &ingredients, &mut Vec::new(), true)
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
}
