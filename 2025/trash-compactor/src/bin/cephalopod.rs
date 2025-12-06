use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Advent of Code 2025 - Day 6: Trash Compactor (Part 2)
///
/// Cephalopod math: Numbers are read column-by-column, right-to-left.
/// Each column within a problem is a digit; top row = most significant digit.
///
/// Example:
/// 123 → reading columns right-to-left: '3', '2', '1' 
///  45 → reading columns right-to-left: '5', '4', ' '
///   6 → reading columns right-to-left: '6', ' ', ' '
/// *
/// Column 3 (rightmost): 3,5,6 → number 356
/// Column 2: 2,4,' ' → number 24  
/// Column 1: 1,' ',' ' → number 1
/// Result: 356 * 24 * 1 = 8544

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Read all lines into a vector
    let lines: Vec<String> = reader
        .lines()
        .filter_map(|l| l.ok())
        .collect();

    if lines.is_empty() {
        println!("Grand total: 0");
        return Ok(());
    }

    // Find the maximum line width
    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Pad all lines to the same width
    let padded: Vec<String> = lines
        .iter()
        .map(|l| format!("{:<width$}", l, width = max_width))
        .collect();

    // The last row contains the operators
    let num_rows = padded.len();

    // Convert to char grid for easier column access
    let grid: Vec<Vec<char>> = padded.iter().map(|s| s.chars().collect()).collect();

    // Find problem boundaries (columns that are all spaces)
    let mut problems: Vec<(usize, usize)> = Vec::new();
    let mut in_problem = false;
    let mut start_col = 0;

    for col in 0..max_width {
        let col_empty = (0..num_rows).all(|row| grid[row][col] == ' ');

        if !col_empty && !in_problem {
            in_problem = true;
            start_col = col;
        } else if col_empty && in_problem {
            problems.push((start_col, col));
            in_problem = false;
        }
    }
    if in_problem {
        problems.push((start_col, max_width));
    }

    let mut grand_total: u128 = 0;

    // Process each problem
    for (start, end) in problems {
        // Extract the operator from the last row
        let operator = if (start..end).any(|c| grid[num_rows - 1][c] == '*') {
            '*'
        } else {
            '+'
        };

        // Read numbers column by column, RIGHT TO LEFT
        // Each column forms a digit of each number (top = most significant)
        let mut numbers: Vec<u128> = Vec::new();

        // Iterate columns from right to left
        for col in (start..end).rev() {
            // Build a number from digits in this column (rows 0 to num_rows-2)
            let mut num_str = String::new();
            for row in 0..(num_rows - 1) {
                let ch = grid[row][col];
                if ch.is_ascii_digit() {
                    num_str.push(ch);
                }
            }

            if !num_str.is_empty() {
                if let Ok(n) = num_str.parse::<u128>() {
                    numbers.push(n);
                }
            }
        }

        // Calculate the result
        let result: u128 = if operator == '*' {
            numbers.iter().product()
        } else {
            numbers.iter().sum()
        };

        grand_total += result;
    }

    println!("Grand total: {}", grand_total);
    Ok(())
}
