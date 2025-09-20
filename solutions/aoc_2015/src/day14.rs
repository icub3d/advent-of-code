use std::time::Instant;

use itertools::Itertools;

const INPUT: &'static str = include_str!("inputs/day14.txt");

#[derive(Debug)]
struct Reindeer {
    speed: usize,
    duration: usize,
    rest: usize,
}

impl Reindeer {
    fn parse(line: &str) -> Self {
        let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
        Reindeer {
            speed: parts[3].parse().unwrap(),
            duration: parts[6].parse().unwrap(),
            rest: parts[13].parse().unwrap(),
        }
    }

    fn distance(&self, time: usize) -> usize {
        let total_duration = self.duration + self.rest;
        let runs = time / total_duration;
        let rem = time % total_duration;
        runs * self.speed * self.duration + rem.min(self.duration) * self.speed
    }
}

pub fn p1(input: &str) -> usize {
    input
        .lines()
        .map(Reindeer::parse)
        .map(|r| r.distance(2503))
        .max()
        .unwrap()
}

fn farthest_at_time(reindeer: &Vec<Reindeer>, time: usize) -> Vec<usize> {
    let mut farthest: Vec<usize> = vec![];
    let mut dist = usize::MIN;

    for cur in 0..reindeer.len() {
        let cur_dist = reindeer[cur].distance(time);
        match cur_dist.cmp(&dist) {
            std::cmp::Ordering::Equal => farthest.push(cur),
            std::cmp::Ordering::Greater => {
                farthest = vec![cur];
                dist = cur_dist;
            }
            std::cmp::Ordering::Less => continue,
        }
    }
    farthest
}

pub fn p2(input: &str) -> usize {
    let reindeer = input
        .lines()
        .map(Reindeer::parse)
        .collect::<Vec<Reindeer>>();

    (1..=2503)
        .flat_map(|time| farthest_at_time(&reindeer, time))
        .counts()
        .into_iter()
        .max_by_key(|&(_, v)| v)
        .map(|(_, v)| v)
        .unwrap()
}

pub fn solve() -> anyhow::Result<()> {
    let now = Instant::now();
    println!("p1: {} ({:?})", p1(INPUT), now.elapsed());
    let now = Instant::now();
    println!("p2: {} ({:?})", p2(INPUT), now.elapsed());
    Ok(())
}
