use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Advent of Code 2025 - Day 4: Printing Department (Part 2)
///
/// Problem: Repeatedly remove accessible paper rolls until no more can be removed.
/// A roll is accessible if it has fewer than 4 adjacent rolls (8 directions).
/// When a roll is removed, its neighbors may become accessible.

/// 8 directions: up, down, left, right, and 4 diagonals
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

/// Count adjacent paper rolls for a given position
fn count_neighbors(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut count = 0;

    for (dr, dc) in &DIRECTIONS {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols {
            if grid[new_row as usize][new_col as usize] == '@' {
                count += 1;
            }
        }
    }
    count
}

/// Find all accessible rolls (fewer than 4 neighbors)
fn find_accessible(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut accessible = Vec::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '@' && count_neighbors(grid, row, col) < 4 {
                accessible.push((row, col));
            }
        }
    }
    accessible
}

fn main() -> io::Result<()> {
    // Read the grid from input file
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Parse the grid into a 2D vector of characters
    let mut grid: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut total_removed = 0;

    // Keep removing accessible rolls until none are left
    loop {
        let accessible = find_accessible(&grid);

        if accessible.is_empty() {
            break; // No more rolls can be removed
        }

        // Remove all accessible rolls
        for (row, col) in &accessible {
            grid[*row][*col] = '.';
        }

        total_removed += accessible.len();
    }

    println!("Total rolls removed: {}", total_removed);
    Ok(())
}

