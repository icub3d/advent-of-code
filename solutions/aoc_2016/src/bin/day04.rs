use std::time::Instant;

use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day04.txt");

#[derive(Debug)]
struct RoomInfo<'a> {
    name: &'a str,
    id: usize,
    checksum: &'a str,
}

impl<'a> RoomInfo<'a> {
    fn new(input: &'a str) -> Self {
        let mut parts = input.split('[');
        let left = parts.next().unwrap();
        let checksum = parts.next().unwrap().trim_end_matches(']');
        let (name, id) = left.rsplit_once('-').unwrap();
        Self {
            name,
            id: id.parse().unwrap(),
            checksum,
        }
    }

    fn verify(&self) -> bool {
        self.name
            .chars()
            .filter(|c| *c != '-')
            .sorted()
            .dedup_with_count()
            .sorted_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)))
            .map(|(_, letter)| letter)
            .take(5)
            .collect::<String>()
            == self.checksum
    }

    fn decipher(&self) -> String {
        self.name
            .chars()
            .map(|c| match c {
                '-' => ' ',
                _ => (b'a' + ((c as u8 - b'a' + (self.id % 26) as u8) % 26)) as char,
            })
            .collect()
    }
}

type Input<'a> = Vec<RoomInfo<'a>>;

fn parse_input(input: &'_ str) -> Input<'_> {
    input.lines().map(RoomInfo::new).collect()
}

fn p1(input: &Input) -> usize {
    input.iter().filter(|r| r.verify()).map(|r| r.id).sum()
}

fn p2(input: &Input) -> String {
    input
        .iter()
        .filter(|r| r.verify())
        .find(|r| r.decipher() == "northpole object storage")
        .map(|r| r.id)
        .unwrap()
        .to_string()
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
