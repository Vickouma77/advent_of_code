use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Advent of Code 2025 - Day 7: Laboratories (Part 2)
///
/// Quantum tachyon manifold: A single particle takes BOTH paths at each splitter.
/// Each split creates a new timeline. Count the total number of timelines
/// after the particle completes all possible journeys.
///
/// Key insight: Track how many timelines are at each column position.
/// When a timeline hits a splitter, it becomes 2 timelines (left and right).

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Parse the grid
    let grid: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    // Find the starting position 'S'
    let mut start_col = 0;
    let mut current_row = 0;
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == 'S' {
                start_col = col;
                current_row = row;
            }
        }
    }

    // Track timelines: column -> number of timelines at that column
    // Start with 1 timeline at the starting column
    let mut timelines: HashMap<usize, u128> = HashMap::new();
    timelines.insert(start_col, 1);

    // Simulate the particle moving downward
    while current_row < rows - 1 {
        current_row += 1;

        let mut next_timelines: HashMap<usize, u128> = HashMap::new();

        for (&col, &count) in &timelines {
            if col >= cols {
                continue;
            }

            match grid[current_row][col] {
                '^' => {
                    // Each timeline splits into 2 timelines (left and right)
                    if col > 0 {
                        *next_timelines.entry(col - 1).or_insert(0) += count;
                    }
                    if col + 1 < cols {
                        *next_timelines.entry(col + 1).or_insert(0) += count;
                    }
                }
                _ => {
                    // Timeline continues downward
                    *next_timelines.entry(col).or_insert(0) += count;
                }
            }
        }

        timelines = next_timelines;

        // If no timelines left, stop
        if timelines.is_empty() {
            break;
        }
    }

    // Sum all timelines across all positions
    let total_timelines: u128 = timelines.values().sum();

    println!("Total timelines: {}", total_timelines);
    Ok(())
}
