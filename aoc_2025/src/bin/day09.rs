use std::collections::VecDeque;
use std::ops::{Add, Index, IndexMut};
use std::time::Instant;

use itertools::Itertools;
// TODO: Cool trick is that you can sort of infer that the largest box will be along the horizontal lines, so you can just sort of check them to see which are largest. https://www.reddit.com/r/adventofcode/comments/1phywvn/comment/nt2nnxw/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

// TODO: There is likely a bug here where we compress boxes of bad space and make it look good. See my drawing.

// NOTE: I often break up impls to make it more understandable of how I went about solving.

const INPUT: &str = include_str!("inputs/day09.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile {
    row: isize,
    col: isize,
}

impl From<&str> for Tile {
    fn from(value: &str) -> Self {
        let (col, row) = value
            .split_once(',')
            .map(|(c, r)| (c.parse().unwrap(), r.parse().unwrap()))
            .unwrap();
        Self::new(row, col)
    }
}

impl Tile {
    fn area(&self, rhs: &Self) -> usize {
        (self.row.abs_diff(rhs.row) + 1) * (self.col.abs_diff(rhs.col) + 1)
    }
}

fn parse(input: &str) -> impl Iterator<Item = Tile> {
    input.trim().lines().map(Tile::from)
}

fn p1(input: &str) -> usize {
    // For each Tile find it's area with all other tiles and then return the max.
    let input = parse(input).collect::<Vec<_>>();
    input
        .iter()
        .enumerate()
        .flat_map(|(i, t1)| input[i + 1..].iter().map(move |t2| t1.area(t2)))
        .max()
        .unwrap()
}

// We use a "rectangle" is several places. It seemed more readable to make it in one place.
#[derive(Debug, Clone, Copy)]
struct Rect {
    r1: usize,
    c1: usize,
    r2: usize,
    c2: usize,
}

impl Rect {
    fn new(r1: usize, c1: usize, r2: usize, c2: usize) -> Self {
        Self {
            r1: r1.min(r2),
            c1: c1.min(c2),
            r2: r1.max(r2),
            c2: c1.max(c2),
        }
    }
}

// Impl add for adding our deltas.
impl Add for Tile {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Tile {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    const DELTAS: [Tile; 4] = [
        Tile { row: 0, col: 1 },
        Tile { row: 0, col: -1 },
        Tile { row: 1, col: 0 },
        Tile { row: -1, col: 0 },
    ];

    // Get neighbors within bounds (inclusive).
    fn neighbors(&self, min: Tile, max: Tile) -> impl Iterator<Item = Tile> {
        let tile = *self;
        Self::DELTAS
            .iter()
            .map(move |&delta| tile + delta)
            .filter(move |t| {
                t.row >= min.row && t.row <= max.row && t.col >= min.col && t.col <= max.col
            })
    }
}

// Track the state of each tile in the compressed grid.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TileState {
    Inside,   // Inside the boundary (also default as we'll mark outside)
    Boundary, // Path of red/green tiles
    Outside,  // Area marked by flood fill
}

// Track our compressed grid.
struct CompressedGrid {
    // Original rows and columns.
    rows: Vec<isize>,
    cols: Vec<isize>,

    // Compressed cells that represent those rows and columns.
    tiles: Vec<Vec<TileState>>,

    // Track our top left and bottom right tile for filtering neighbors.
    min: Tile,
    max: Tile,

    // Used later for faster solution.
    pref: Vec<Vec<usize>>,
}

// We use these indexing when setting stuff up.
impl Index<&Tile> for CompressedGrid {
    type Output = TileState;

    fn index(&self, tile: &Tile) -> &Self::Output {
        &self.tiles[tile.row as usize][tile.col as usize]
    }
}

impl IndexMut<&Tile> for CompressedGrid {
    fn index_mut(&mut self, tile: &Tile) -> &mut Self::Output {
        &mut self.tiles[tile.row as usize][tile.col as usize]
    }
}

impl CompressedGrid {
    // A helper function to create a coordinate axis including one value before and one value after
    // to make a sort of boundary around the coordinates.
    fn create_axis(mut coords: Vec<isize>) -> Vec<isize> {
        let min = *coords.iter().min().unwrap();
        let max = *coords.iter().max().unwrap();
        coords.push(min - 1);
        coords.push(max + 1);
        coords.sort_unstable();
        coords.dedup();
        coords
    }

    fn new(original_tiles: &[Tile]) -> Self {
        // Get our coordinate axes for compression.
        let rows = Self::create_axis(original_tiles.iter().map(|t| t.row).collect());
        let cols = Self::create_axis(original_tiles.iter().map(|t| t.col).collect());

        // Create the "zoomed out" version of the grid.
        let tiles = vec![vec![TileState::Inside; cols.len()]; rows.len()];

        // Track the min/max so we can filter the neighbors of each tile.
        let min = Tile::new(0, 0);
        let max = Tile::new(tiles.len() as isize - 1, tiles[0].len() as isize - 1);

        // Create the grid now so we can use the helper functions.
        let mut cp = Self {
            rows,
            cols,
            tiles,
            pref: vec![],
            min,
            max,
        };

        cp.add_boundaries(original_tiles);
        cp.mark_outside();
        cp
    }

    // Add our boundary lines from tiles.
    fn add_boundaries(&mut self, original_tiles: &[Tile]) {
        for (t1, t2) in original_tiles.iter().circular_tuple_windows() {
            let rect = self.compressed_rect(t1, t2);

            // We go horizontal or vertical and can do that two different ways.
            if rect.r1 == rect.r2 {
                self.tiles[rect.r1][rect.c1..=rect.c2].fill(TileState::Boundary);
            } else {
                self.tiles[rect.r1..=rect.r2]
                    .iter_mut()
                    .for_each(|row| row[rect.c1] = TileState::Boundary);
            }
        }
    }

    // Essentially a flood fill algorithm. We use outside because we are fairly sure (0,0) is
    // outside and we have a border that should allow us to fill around.
    fn mark_outside(&mut self) {
        let mut frontier = VecDeque::new();

        // Start by marking top left corner.
        let start = Tile { row: 0, col: 0 };
        self[&start] = TileState::Outside;
        frontier.push_back(start);

        // Go through our frontier, find valid neighbors, mark them and add them to frontier.
        let min = self.min;
        let max = self.max;
        while let Some(tile) = frontier.pop_front() {
            for neighbor in tile.neighbors(min, max) {
                if self[&neighbor] == TileState::Inside {
                    self[&neighbor] = TileState::Outside;
                    frontier.push_back(neighbor);
                }
            }
        }
    }

    // We are really only interested in rectangles when solving.
    fn compressed_rect(&self, tile1: &Tile, tile2: &Tile) -> Rect {
        let (r1, c1) = (
            self.rows.binary_search(&tile1.row).unwrap(),
            self.cols.binary_search(&tile1.col).unwrap(),
        );
        let (r2, c2) = (
            self.rows.binary_search(&tile2.row).unwrap(),
            self.cols.binary_search(&tile2.col).unwrap(),
        );
        Rect::new(r1, c1, r2, c2)
    }

    fn valid(&self, rect: Rect) -> bool {
        // very simple valid check. All values inside the rectangle need to not be outside.
        (rect.r1..=rect.r2)
            .cartesian_product(rect.c1..=rect.c2)
            .all(|(r, c)| self.tiles[r][c] != TileState::Outside)
    }
}

fn p2(input: &str) -> usize {
    // Create our compression grid from the tiles.
    let tiles = parse(input).collect::<Vec<_>>();
    let grid = CompressedGrid::new(&tiles);

    // Now we can simply look through the rectangles formed and find the largest one in the *original* grid.
    tiles
        .iter()
        .enumerate()
        .flat_map(|(i, t1)| {
            tiles[i + 1..]
                .iter()
                .map(|t2| (t1.area(t2), grid.compressed_rect(t1, t2)))
        })
        .filter(|(_, rect)| grid.valid(*rect))
        .map(|(area, _)| area)
        .max()
        .unwrap()
}

// Simplify turning a TileState into a value for the prefix_sum.
impl From<TileState> for usize {
    fn from(value: TileState) -> Self {
        match value {
            TileState::Outside => 1,
            _ => 0,
        }
    }
}

impl CompressedGrid {
    // Let's build a prefix sum of tiles outside the boundary. Each (r,c) pair contains how many
    // tiles were outside the rectangle from (0,0). We maintain padding (all the +1) to simplify
    // edge cases.
    fn build_prefix_sum(&mut self) {
        let cols = self.tiles[0].len();
        let rows = self.tiles.len();
        self.pref = vec![vec![0usize; cols + 1]; rows + 1];

        for r in 0..rows {
            for c in 0..cols {
                // The value of our current tile is above + left - top_left.
                let val: usize = self.tiles[r][c].into();
                self.pref[r + 1][c + 1] =
                    self.pref[r][c + 1] + self.pref[r + 1][c] - self.pref[r][c] + val;
            }
        }
    }

    fn valid_pfx(&self, rect: Rect) -> bool {
        // We'll know we are completely in the grid if our_sum + sum_top_left - sum_to_left - sum_above == 0
        let count = self.pref[rect.r2 + 1][rect.c2 + 1] + self.pref[rect.r1][rect.c1]
            - self.pref[rect.r1][rect.c2 + 1]
            - self.pref[rect.r2 + 1][rect.c1];
        count == 0
    }
}

fn p2_pfx(input: &str) -> usize {
    // Now we can do the same thing, but instead use the prefix_sum values to calculate much
    // faster.
    let tiles = parse(input).collect::<Vec<_>>();
    let mut grid = CompressedGrid::new(&tiles);
    grid.build_prefix_sum();

    tiles
        .iter()
        .enumerate()
        .flat_map(|(i, t1)| {
            tiles[i + 1..]
                .iter()
                .map(|t2| (t1.area(t2), grid.compressed_rect(t1, t2)))
        })
        .filter(|(_, rect)| grid.valid_pfx(*rect))
        .map(|(area, _)| area)
        .max()
        .unwrap()
}

pub fn p2_online(input: &str) -> usize {
    let points: Vec<Tile> = input.lines().map(Tile::from).collect();
    let edges: Vec<(&Tile, &Tile)> = points
        .windows(2)
        .map(|vertices| (&vertices[0], &vertices[1]))
        .chain([(&points[points.len() - 1], &points[0])]) // closing edge
        .collect();
    let mut possible_rects: Vec<_> = points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| points[i + 1..].iter().map(move |p2| (p1, p2, p1.area(p2))))
        .collect();
    possible_rects.sort_by_key(|(_, _, a)| *a);
    possible_rects
        .into_iter()
        .rev()
        .find(|(p1, p2, _)| {
            // all edges in the full polygon must be:
            //   - leftmost point of edge must be left of this rect OR
            //   - rightmost point of edge must be right of this rect OR
            //   - topmost point of edge must be above this rect OR
            //   - bottommost point of edge must be below this rect
            edges.iter().all(|(start, end)| {
                let before = p1.col.max(p2.col) <= start.col.min(end.col);
                let after = p1.col.min(p2.col) >= start.col.max(end.col);
                let above = p1.row.max(p2.row) <= start.row.min(end.row);
                let below = p1.row.min(p2.row) >= start.row.max(end.row);
                before || after || above || below
            })
        })
        .expect("possible should not be empty")
        .2
}

fn main() {
    let now = Instant::now();
    let solution = p1(INPUT);
    println!("p1 {:?} {}", now.elapsed(), solution);

    let now = Instant::now();
    let solution = p2(INPUT);
    println!("p2 {:?} {}", now.elapsed(), solution);
    assert!(solution == 1516172795);

    let now = Instant::now();
    let solution = p2_pfx(INPUT);
    println!("p2_pfx {:?} {}", now.elapsed(), solution);
    assert!(solution == 1516172795);

    let now = Instant::now();
    let solution = p2_online(INPUT);
    println!("p2_online {:?} {}", now.elapsed(), solution);
    assert!(solution == 1516172795);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("inputs/day09-sample.txt");

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 50);
    }

    #[test]
    fn test_p2_pfx() {
        assert_eq!(p2_pfx(INPUT), 24);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 24);
    }
}
