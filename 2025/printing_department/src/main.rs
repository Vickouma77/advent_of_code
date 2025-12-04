use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Advent of Code 2025 - Day 4: Printing Department
///
/// Problem: Find paper rolls (@) that can be accessed by forklifts.
/// A roll is accessible if it has fewer than 4 adjacent rolls (8 directions).
///
/// Example:
/// ..@@.@@@@.
/// @@@.@.@.@@
/// The '@' at position (0,2) has neighbors at 8 positions around it.
/// Count how many of those neighbors are also '@'.
/// If count < 4, the roll is accessible.

fn main() -> io::Result<()> {
    // Read the grid from input file
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Parse the grid into a 2D vector of characters
    let grid: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    // 8 directions: up, down, left, right, and 4 diagonals
    let directions: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),  // top-left, top, top-right
        ( 0, -1),          ( 0, 1),  // left, right
        ( 1, -1), ( 1, 0), ( 1, 1),  // bottom-left, bottom, bottom-right
    ];

    let mut accessible_count = 0;

    // Check each cell in the grid
    for row in 0..rows {
        for col in 0..cols {
            // Only check paper rolls (@)
            if grid[row][col] != '@' {
                continue;
            }

            // Count adjacent paper rolls
            let mut neighbor_rolls = 0;
            for (dr, dc) in &directions {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;

                // Check bounds
                if new_row >= 0 && new_row < rows as i32 
                   && new_col >= 0 && new_col < cols as i32 {
                    if grid[new_row as usize][new_col as usize] == '@' {
                        neighbor_rolls += 1;
                    }
                }
            }

            // Accessible if fewer than 4 adjacent rolls
            if neighbor_rolls < 4 {
                accessible_count += 1;
            }
        }
    }

    println!("Accessible paper rolls: {}", accessible_count);
    Ok(())
}
