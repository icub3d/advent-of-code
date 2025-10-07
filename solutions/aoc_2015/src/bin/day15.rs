use std::{
    iter::{Sum, once},
    ops::Add,
    time::Instant,
};

const INPUT: &str = include_str!("inputs/day15.txt");

#[derive(Default, Debug, Copy, Clone)]
struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

impl<'a> Add<&'a Ingredient> for Ingredient {
    type Output = Ingredient;

    fn add(self, rhs: &'a Ingredient) -> Self::Output {
        Ingredient {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

impl<'a> Sum<&'a Ingredient> for Ingredient {
    fn sum<I: Iterator<Item = &'a Ingredient>>(iter: I) -> Self {
        iter.fold(Ingredient::default(), |acc, x| acc + x)
    }
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

    fn valid(&self, check_calories: bool) -> bool {
        self.capacity > 0
            && self.durability > 0
            && self.flavor > 0
            && self.texture > 0
            && (!check_calories || self.calories == 500)
    }

    fn total_score(&self) -> isize {
        self.capacity * self.durability * self.flavor * self.texture
    }
}

fn calc_scores(
    remaining: isize,
    ingredients: &[Ingredient],
    scores: &mut Vec<Ingredient>,
    check_calories: bool,
) -> isize {
    if ingredients.len() == 1 {
        let score = ingredients[0].score(remaining);
        let totals = scores.iter().chain(once(&score)).sum::<Ingredient>();
        return match totals.valid(check_calories) {
            true => totals.total_score(),
            false => -1,
        };
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

fn p1(input: &str) -> isize {
    let ingredients = input
        .lines()
        .map(Ingredient::parse)
        .collect::<Vec<Ingredient>>();
    calc_scores(100, &ingredients, &mut Vec::new(), false)
}

fn p2(input: &str) -> isize {
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
