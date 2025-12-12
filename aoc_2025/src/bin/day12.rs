use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::time::Instant;

const INPUT: &str = include_str!("inputs/day12.txt");
const SHAPE_SIZE: usize = 3;
const SHAPE_COORDS: [(usize, usize); 9] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (1, 0),
    (1, 1),
    (1, 2),
    (2, 0),
    (2, 1),
    (2, 2),
];

// 1. Read problem.
// 2. Panic.
// 3. Analyze input.
// 4. Imagine puzzler's grin.
// 5. Can we still try?

// Interesting information:
//
// https://www.frontiersin.org/journals/mechanical-engineering/articles/10.3389/fmech.2022.966691/full

// A shape where the set values represent which parts of the 3x3 grid are set.
#[derive(Clone)]
struct Shape {
    shape: Vec<Vec<bool>>,
}

impl From<&&str> for Shape {
    fn from(value: &&str) -> Self {
        let mut lines = value.lines();
        // Skip first line.
        lines.next();
        let shape: Vec<Vec<bool>> = lines
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();

        Self { shape }
    }
}

impl Shape {
    fn area(&self) -> usize {
        self.shape.iter().flatten().filter(|&&b| b).count()
    }
}

struct Region {
    width: usize,
    height: usize,
    shapes: Vec<usize>,
}

impl From<&str> for Region {
    fn from(value: &str) -> Self {
        let (dims, shapes) = value.split_once(": ").unwrap();
        let (width, height) = dims.split_once("x").unwrap();

        Self {
            width: width.parse().unwrap(),
            height: height.parse().unwrap(),
            shapes: shapes
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }
}

fn parse(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let chunks = input.trim().split("\n\n").collect::<Vec<_>>();

    let shapes = chunks[..chunks.len() - 1].iter().map(Shape::from).collect();
    let regions = chunks[chunks.len() - 1].lines().map(Region::from).collect();

    (shapes, regions)
}

fn p1(input: &str) -> usize {
    // For the real input, just count "easy" regions
    let (_, regions) = parse(input);
    regions
        .iter()
        .filter(|region| {
            // It will be easy if there are enough 3x3 slots to place all the needed shapes.
            let available = (region.width / SHAPE_SIZE) * (region.height / SHAPE_SIZE);
            let needed: usize = region.shapes.iter().sum();
            available >= needed
        })
        .count()
}

// -----------------------------------------------------------------
// The rest of this is trying to solve it on just the sample input.
// -----------------------------------------------------------------

// We'll use a bit_mask to represent a shape. I played around with different sizes.
type ShapeMask = u64;

impl Shape {
    // Turn the shape into a bit mask.
    fn to_mask(&self) -> ShapeMask {
        SHAPE_COORDS.iter().fold(0, |mask, &(i, j)| {
            if self.shape[i][j] {
                mask | (1 << (i * SHAPE_SIZE + j))
            } else {
                mask
            }
        })
    }

    fn rotate(&self) -> Self {
        let rows = self.shape.len();
        let cols = self.shape[0].len();
        let mut rotated_shape = vec![vec![false; rows]; cols];
        for &(i, j) in &SHAPE_COORDS {
            rotated_shape[j][rows - 1 - i] = self.shape[i][j];
        }
        Self {
            shape: rotated_shape,
        }
    }

    fn flip_horizontal(&self) -> Self {
        let mut next = self.clone();
        next.shape.iter_mut().for_each(|row| row.reverse());
        next
    }

    fn flip_vertical(&self) -> Self {
        let mut next = self.clone();
        next.shape.reverse();
        next
    }

    // All of the above is to basically produce this. We want a unique list of all the variations.
    fn variations(&self) -> Vec<ShapeMask> {
        let mut candidates = Vec::new();
        let mut current = self.clone();
        for _ in 0..4 {
            candidates.push(current.to_mask());
            candidates.push(current.flip_horizontal().to_mask());
            candidates.push(current.flip_vertical().to_mask());
            current = current.rotate();
        }

        candidates.sort_unstable();
        candidates.dedup();
        candidates
    }
}

// Pack remaining counts into a u64 for efficient memoization.
// Assumes max 8 shapes with counts < 256 each (8 bits per count).
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct PackedRemaining(u64);

impl PackedRemaining {
    fn new(remaining: &[usize]) -> Self {
        let packed = remaining
            .iter()
            .enumerate()
            .fold(0u64, |acc, (i, &count)| acc | ((count as u64) << (i * 8)));
        Self(packed)
    }

    fn get(&self, index: usize) -> usize {
        ((self.0 >> (index * 8)) & 0xFF) as usize
    }

    fn decrement(&self, index: usize) -> Self {
        Self(self.0 - (1u64 << (index * 8)))
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl Region {
    fn bin_pack(&self, all_shapes: &[Vec<ShapeMask>]) -> bool {
        // Use u64 bit mask for board (the actual input wouldn't work here.)
        // Pre-allocate HashMap with capacity to avoid resizing
        let mut memo: FxHashMap<(u64, PackedRemaining), bool> =
            FxHashMap::with_capacity_and_hasher(10000, Default::default());
        let remaining = PackedRemaining::new(&self.shapes);
        self.dp_pack(0u64, remaining, all_shapes, &mut memo)
    }

    // Count how many valid placements exist for a shape variation on the current board
    // More efficient than checking each position with can_place
    fn count_placements(&self, board: u64, shape: &ShapeMask) -> usize {
        let mut count = 0;
        let max_r = self.height.saturating_sub(SHAPE_SIZE);
        let max_c = self.width.saturating_sub(SHAPE_SIZE);

        // For each row where the shape can fit
        for r in 0..=max_r {
            // For each column where the shape can fit
            for c in 0..=max_c {
                // Build the mask for this 3x3 region in one go
                let mut board_mask = 0u64;
                for &(sr, sc) in &SHAPE_COORDS {
                    let pos = (r + sr) * self.width + (c + sc);
                    if (board >> pos) & 1 == 1 {
                        board_mask |= 1 << (sr * SHAPE_SIZE + sc);
                    }
                }

                // If no overlap, this is a valid placement
                if board_mask & shape == 0 {
                    count += 1;
                }
            }
        }

        count
    }

    // The question we are trying to answer is "is it possible?", not what is the best.
    // We should focus our heuristic on minimizing branching.
    // heuristic: choose the shape with fewest valid placements (most constrained)
    fn heuristic(
        &self,
        board: u64,
        remaining: PackedRemaining,
        shapes: &[Vec<ShapeMask>],
    ) -> Option<usize> {
        let mut best = None;
        let mut min = usize::MAX;

        for (shape, variations) in shapes.iter().enumerate() {
            // We don't need to place any more of this shape.
            if remaining.get(shape) == 0 {
                continue;
            }

            // How many ways can this shape be placed?
            let mut placements = 0;
            for variation in variations {
                let count = self.count_placements(board, variation);
                placements += count;

                // Early exit optimization: if we already have more placements
                // than current min, stop counting for this shape
                if placements >= min {
                    break;
                }
            }

            // If we found a shape with 0 placements, this is unsolvable
            if placements == 0 {
                return None;
            }

            // Did we find a new min?
            if placements < min {
                min = placements;
                best = Some(shape);

                // If min is 1, we can't do better, return immediately
                if min == 1 {
                    return best;
                }
            }
        }

        best
    }

    fn dp_pack(
        &self,
        board: u64,
        remaining: PackedRemaining,
        shapes: &[Vec<ShapeMask>],
        memo: &mut FxHashMap<(u64, PackedRemaining), bool>,
    ) -> bool {
        // Check memo using board and packed remaining
        let key = (board, remaining);
        if let Some(&result) = memo.get(&key) {
            return result;
        }

        // If all shapes placed, success
        if remaining.is_zero() {
            return true;
        }

        // If no shape can be placed, this is unsolvable.
        let shape = match self.heuristic(board, remaining, shapes) {
            Some(idx) => idx,
            None => {
                memo.insert(key, false);
                return false;
            }
        };

        // Try all variations of the most constrained shape
        for variation in &shapes[shape] {
            for r in 0..=self.height.saturating_sub(SHAPE_SIZE) {
                for c in 0..=self.width.saturating_sub(SHAPE_SIZE) {
                    if self.can_place(board, variation, r, c) {
                        let new_board = self.place(board, variation, r, c);
                        let new_remaining = remaining.decrement(shape);

                        if self.dp_pack(new_board, new_remaining, shapes, memo) {
                            memo.insert(key, true);
                            return true;
                        }
                    }
                }
            }
        }

        // Couldn't place any remaining shape
        memo.insert(key, false);
        false
    }

    fn can_place(&self, board: u64, shape: &ShapeMask, r: usize, c: usize) -> bool {
        if r + SHAPE_SIZE > self.height || c + SHAPE_SIZE > self.width {
            return false;
        }

        // Extract 3x3 region from board and check if shape overlaps with occupied cells
        let board_region = SHAPE_COORDS.iter().fold(0, |region, &(sr, sc)| {
            let pos = (r + sr) * self.width + (c + sc);
            if (board >> pos) & 1 == 1 {
                region | (1 << (sr * SHAPE_SIZE + sc))
            } else {
                region
            }
        });

        board_region & shape == 0
    }

    fn place(&self, board: u64, shape: &ShapeMask, r: usize, c: usize) -> u64 {
        SHAPE_COORDS.iter().fold(board, |b, &(sr, sc)| {
            if (shape >> (sr * SHAPE_SIZE + sc)) & 1 == 1 {
                let pos = (r + sr) * self.width + (c + sc);
                b | (1u64 << pos)
            } else {
                b
            }
        })
    }
}

fn sample(input: &str) -> usize {
    // Get all variations of each shape and then call bin_pack for each of the regions.
    let (shapes, regions) = parse(input);
    let shape_masks: Vec<Vec<ShapeMask>> = shapes.par_iter().map(|s| s.variations()).collect();

    regions
        .par_iter()
        .filter(|r| r.bin_pack(&shape_masks))
        .count()
}

fn main() {
    // How many are impossible or easy?
    let (shapes, regions) = parse(INPUT);

    let mut easy = 0;
    let mut area_fail = 0;
    let mut needs_solving = 0;

    for region in regions.iter() {
        let available = (region.width / SHAPE_SIZE) * (region.height / SHAPE_SIZE);
        let needed: usize = region.shapes.iter().sum();

        let area = region.width * region.height;
        let volume: usize = region
            .shapes
            .iter()
            .enumerate()
            .map(|(shape_idx, &count)| count * shapes[shape_idx].area())
            .sum();

        if volume > area {
            area_fail += 1;
        } else if available >= needed {
            easy += 1;
        } else {
            needs_solving += 1;
        }
    }

    println!(
        "easy: {}, area_fail: {}, needs_solving: {}",
        easy, area_fail, needs_solving
    );

    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    // Run sample input to show "solution".
    const SAMPLE_INPUT: &str = include_str!("inputs/day12-sample.txt");
    let now = Instant::now();
    let solution = sample(SAMPLE_INPUT);
    println!("s1 {:?} {}", now.elapsed(), solution);

    // Run sample solution against actual input. Will never finish?
    // let now = Instant::now();
    // let solution = sample(INPUT);
    // println!("p1_slow {:?} {}", now.elapsed(), solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("inputs/day12-sample.txt");

    #[test]
    fn test_p1() {
        assert_eq!(sample(INPUT), 2);
    }
}
