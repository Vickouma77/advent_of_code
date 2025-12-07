use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Advent of Code 2025 - Day 7: Laboratories
///
/// Problem: Simulate tachyon beams traveling through a manifold.
/// - Beams start at 'S' and travel downward
/// - Empty space (.) allows beams to pass through
/// - Splitters (^) stop the beam and emit two new beams (left and right)
/// - Count how many times a beam is split

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
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == 'S' {
                start_col = col;
            }
        }
    }

    // Track active beams as (row, col) - all beams travel downward
    // Use a set to handle beams merging at the same position
    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(start_col);

    let mut split_count = 0;
    let mut current_row = 0;

    // Find the row where S is located
    for row in 0..rows {
        if grid[row].iter().any(|&c| c == 'S') {
            current_row = row;
            break;
        }
    }

    // Simulate beams moving downward
    while current_row < rows - 1 {
        current_row += 1;

        let mut next_beams: HashSet<usize> = HashSet::new();

        for &col in &beams {
            if col >= cols {
                continue;
            }

            match grid[current_row][col] {
                '^' => {
                    // Beam hits a splitter - split into left and right
                    split_count += 1;
                    if col > 0 {
                        next_beams.insert(col - 1);
                    }
                    if col + 1 < cols {
                        next_beams.insert(col + 1);
                    }
                }
                '.' | 'S' => {
                    // Beam continues downward
                    next_beams.insert(col);
                }
                _ => {
                    // Any other character - beam continues
                    next_beams.insert(col);
                }
            }
        }

        beams = next_beams;

        // If no beams left, stop
        if beams.is_empty() {
            break;
        }
    }

    println!("Total splits: {}", split_count);
    Ok(())
}

