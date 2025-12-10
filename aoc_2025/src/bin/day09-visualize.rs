//! # Day 9 Visualization
//!
//! This binary generates visualizations for the Advent of Code 2025 Day 9 problem.
//! It produces two PNG images:
//! - `day09_original.png`: Visualizes the original polygon defined by the input.
//! - `day09_compressed.png`: Visualizes the compressed grid representation used for the solution.
//!
//! ## Usage
//!
//! Run the visualization using cargo:
//!
//! ```sh
//! cargo run -p aoc_2025 --bin day09-visualize
//! ```
//!
//! The output images will be saved in the current working directory (usually the workspace root).

use catppuccin::PALETTE;
use itertools::Itertools;
use plotters::prelude::*;
use std::collections::VecDeque;
use std::ops::{Add, Index, IndexMut};

const INPUT: &str = include_str!("inputs/day09.txt");

fn to_rgb(color: catppuccin::Color) -> RGBColor {
    RGBColor(color.rgb.r, color.rgb.g, color.rgb.b)
}

// --- Logic from day09.rs ---

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
    #[allow(dead_code)]
    fn area(&self, rhs: &Self) -> usize {
        (self.row.abs_diff(rhs.row) + 1) * (self.col.abs_diff(rhs.col) + 1)
    }
}

fn parse(input: &str) -> impl Iterator<Item = Tile> {
    input.trim().lines().map(Tile::from)
}

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TileState {
    Inside,
    Boundary,
    Outside,
}

struct CompressedGrid {
    rows: Vec<isize>,
    cols: Vec<isize>,
    tiles: Vec<Vec<TileState>>,
    min: Tile,
    max: Tile,
    #[allow(dead_code)]
    pref: Vec<Vec<usize>>,
}

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
        let rows = Self::create_axis(original_tiles.iter().map(|t| t.row).collect());
        let cols = Self::create_axis(original_tiles.iter().map(|t| t.col).collect());
        let tiles = vec![vec![TileState::Inside; cols.len()]; rows.len()];
        let min = Tile::new(0, 0);
        let max = Tile::new(tiles.len() as isize - 1, tiles[0].len() as isize - 1);

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

    fn add_boundaries(&mut self, original_tiles: &[Tile]) {
        for (t1, t2) in original_tiles.iter().circular_tuple_windows() {
            let rect = self.compressed_rect(t1, t2);
            if rect.r1 == rect.r2 {
                self.tiles[rect.r1][rect.c1..=rect.c2].fill(TileState::Boundary);
            } else {
                self.tiles[rect.r1..=rect.r2]
                    .iter_mut()
                    .for_each(|row| row[rect.c1] = TileState::Boundary);
            }
        }
    }

    fn mark_outside(&mut self) {
        let mut frontier = VecDeque::new();
        let start = Tile { row: 0, col: 0 };
        self[&start] = TileState::Outside;
        frontier.push_back(start);

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
}

// --- Visualization ---

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tiles: Vec<Tile> = parse(INPUT).collect();
    let grid = CompressedGrid::new(&tiles);
    let palette = PALETTE.mocha.colors;

    // 1. Plot Original (Polygon)
    {
        let root =
            BitMapBackend::new("aoc_2025/day09_original.png", (1024, 1024)).into_drawing_area();
        root.fill(&to_rgb(palette.base))?;

        let min_x = tiles.iter().map(|t| t.col).min().unwrap();
        let max_x = tiles.iter().map(|t| t.col).max().unwrap();
        let min_y = tiles.iter().map(|t| t.row).min().unwrap();
        let max_y = tiles.iter().map(|t| t.row).max().unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption(
                "Day 9: Original Grid",
                ("sans-serif", 50).into_font().color(&to_rgb(palette.text)),
            )
            .margin(20)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

        // Draw inside area
        let c_inside = to_rgb(palette.green);
        chart.draw_series(
            (0..grid.rows.len() - 1)
                .cartesian_product(0..grid.cols.len() - 1)
                .filter(|&(r, c)| grid.tiles[r][c] == TileState::Inside)
                .map(|(r, c)| {
                    let x0 = grid.cols[c];
                    let y0 = grid.rows[r];
                    let x1 = grid.cols[c + 1];
                    let y1 = grid.rows[r + 1];
                    Rectangle::new([(x0, y0), (x1, y1)], c_inside.filled())
                }),
        )?;

        chart
            .configure_mesh()
            .axis_style(to_rgb(palette.overlay1))
            .label_style(("sans-serif", 15).into_font().color(&to_rgb(palette.text)))
            .draw()?;

        // Draw edges
        let edge_color = to_rgb(palette.blue);
        let mut points: Vec<_> = tiles.iter().map(|t| (t.col, t.row)).collect();
        if let Some(&first) = points.first() {
            points.push(first);
        }
        chart.draw_series(LineSeries::new(points, &edge_color))?;

        // Draw points
        let point_color = to_rgb(palette.red);
        chart.draw_series(PointSeries::of_element(
            tiles.iter().map(|t| (t.col, t.row)),
            5,
            &point_color,
            &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
        ))?;

        root.present()?;
    }

    // 2. Plot Compacted (Grid)
    {
        let root =
            BitMapBackend::new("aoc_2025/day09_compressed.png", (1024, 1024)).into_drawing_area();
        root.fill(&to_rgb(palette.base))?;

        let rows = grid.tiles.len();
        let cols = grid.tiles[0].len();

        let mut chart = ChartBuilder::on(&root)
            .caption(
                "Day 9: Compressed Grid",
                ("sans-serif", 50).into_font().color(&to_rgb(palette.text)),
            )
            .margin(20)
            .build_cartesian_2d(0..cols, 0..rows)?;

        // Colors
        let c_inside = to_rgb(palette.green);
        let c_boundary = to_rgb(palette.red);
        let _c_outside = to_rgb(palette.base); // Unused logic variable, previously commented
        let c_outside_viz = to_rgb(palette.surface1);

        for (r, row_data) in grid.tiles.iter().enumerate() {
            for (c, state) in row_data.iter().enumerate() {
                let color = match state {
                    TileState::Inside => c_inside,
                    TileState::Boundary => c_boundary,
                    TileState::Outside => c_outside_viz,
                };

                // Draw a rectangle for this cell
                chart.draw_series(std::iter::once(Rectangle::new(
                    [(c, r), (c + 1, r + 1)],
                    color.filled(),
                )))?;
            }
        }

        root.present()?;
    }

    println!("Visualization generated: day09_original.png, day09_compressed.png");
    Ok(())
}
