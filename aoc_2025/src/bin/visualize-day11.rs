use std::collections::{HashMap, HashSet, VecDeque};
use std::env;

use catppuccin::PALETTE;
use plotters::prelude::*;
use rustc_hash::FxHashMap;

fn to_rgb(color: catppuccin::Color) -> RGBColor {
    RGBColor(color.rgb.r, color.rgb.g, color.rgb.b)
}

const INPUT: &str = include_str!("inputs/day11.txt");
const INPUT_SAMPLE: &str = include_str!("inputs/day11-sample.txt");
const INPUT_SAMPLE2: &str = include_str!("inputs/day11-sample2.txt");

fn parse(input: &str) -> FxHashMap<&str, Vec<&str>> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (name, outputs) = l.split_once(": ").unwrap();
            (name, outputs.split_whitespace().collect())
        })
        .collect()
}

// Compute levels using BFS from start node
fn compute_levels<'a>(
    graph: &FxHashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
) -> HashMap<&'a str, usize> {
    let mut levels = HashMap::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((node, level)) = queue.pop_front() {
        levels.insert(node, level);

        if let Some(neighbors) = graph.get(node) {
            for &neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, level + 1));
                }
            }
        }
    }

    levels
}

fn visualize_graph(
    input: &str,
    output_file: &str,
    start_node: &str,
    end_node: &str,
    special_nodes: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    let graph = parse(input);
    let levels = compute_levels(&graph, start_node);

    // Collect all nodes and organize by level
    let max_level = *levels.values().max().unwrap_or(&0);
    let mut nodes_by_level: Vec<Vec<&str>> = vec![Vec::new(); max_level + 1];

    for (&node, &level) in &levels {
        nodes_by_level[level].push(node);
    }

    // Sort nodes within each level, prioritizing special nodes
    for level_nodes in &mut nodes_by_level {
        level_nodes.sort_by(|a, b| {
            let a_special = special_nodes.contains(a);
            let b_special = special_nodes.contains(b);
            match (a_special, b_special) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.cmp(b),
            }
        });
    }

    // Calculate positions
    let width = 1920;
    let height = 1080;
    let margin_x = 100;
    let margin_y = 80;
    let level_width = (width - 2 * margin_x) / max_level.max(1);

    let mut positions: HashMap<&str, (i32, i32)> = HashMap::new();

    for (level, nodes) in nodes_by_level.iter().enumerate() {
        let x = margin_x + level * level_width;
        let node_count = nodes.len();
        let spacing = if node_count > 1 {
            (height - 2 * margin_y) / (node_count - 1).max(1)
        } else {
            0
        };

        for (i, &node) in nodes.iter().enumerate() {
            let y = if node_count == 1 {
                height / 2
            } else {
                margin_y + i * spacing
            };
            positions.insert(node, (x as i32, y as i32));
        }
    }

    // Setup plotting with catppuccin colors
    let palette = PALETTE.mocha.colors;
    let root = BitMapBackend::new(output_file, (width as u32, height as u32)).into_drawing_area();
    root.fill(&to_rgb(palette.base))?;

    let root = root.margin(10, 10, 10, 10);

    let mut chart =
        ChartBuilder::on(&root).build_cartesian_2d(0..width as i32, 0..height as i32)?;

    // Draw edges first
    let edge_color = to_rgb(palette.overlay0);

    for (node, neighbors) in &graph {
        if let Some(&(x1, y1)) = positions.get(node) {
            for neighbor in neighbors {
                if let Some(&(x2, y2)) = positions.get(neighbor) {
                    chart.draw_series(std::iter::once(PathElement::new(
                        vec![(x1, y1), (x2, y2)],
                        &edge_color,
                    )))?;
                }
            }
        }
    }

    // Draw nodes
    for (&node, &(x, y)) in &positions {
        let (color, size, font_size) = if node == start_node {
            (palette.green, 24, 24)
        } else if node == end_node {
            (palette.red, 24, 24)
        } else if special_nodes.contains(&node) {
            (palette.mauve, 24, 24)
        } else {
            (palette.blue, 12, 0)
        };

        let node_color = to_rgb(color);

        chart.draw_series(std::iter::once(Circle::new(
            (x, y),
            size,
            node_color.filled(),
        )))?;

        // Draw labels for special nodes
        if node == start_node || node == end_node || special_nodes.contains(&node) {
            let text_color = to_rgb(palette.text);
            chart.draw_series(std::iter::once(Text::new(
                node.to_string(),
                (x, y - 20),
                ("JetBrainsMono Nerd Font", font_size)
                    .into_font()
                    .color(&text_color),
            )))?;
        }
    }

    root.present()?;
    println!("Visualization saved to {}", output_file);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let (input, output_file, start_node) = if args.len() > 1 {
        match args[1].as_str() {
            "--sample" => (INPUT_SAMPLE, "aoc_2025/day11_graph_sample.png", "you"),
            "--sample2" => (INPUT_SAMPLE2, "aoc_2025/day11_graph_sample2.png", "svr"),
            _ => {
                eprintln!("Usage: visualize-day11 [--sample|--sample2]");
                return Ok(());
            }
        }
    } else {
        (INPUT, "aoc_2025/day11_graph.png", "svr")
    };

    let special_nodes = vec!["you", "fft", "dac"];

    visualize_graph(input, output_file, start_node, "out", &special_nodes)?;

    Ok(())
}
