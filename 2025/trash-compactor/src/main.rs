use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Advent of Code 2025 - Day 6: Trash Compactor
///
/// Problem: Parse a grid of vertically-arranged math problems.
/// Numbers are stacked vertically, with the operator (+/*) at the bottom.
/// Problems are separated by columns of spaces.
///
/// Example input:
/// 123 328  51 64
///  45 64  387 23
///   6 98  215 314
/// *   +   *   +
///
/// Problems: 123*45*6=33210, 328+64+98=490, etc.

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

    // Find problem boundaries by looking for columns that are all spaces (except operator row)
    // The last row contains the operators
    let num_rows = padded.len();
    let operator_row = &padded[num_rows - 1];

    // Find column ranges for each problem
    // A problem starts when we see non-space and ends when we see all-space column
    let mut problems: Vec<(usize, usize)> = Vec::new();
    let mut in_problem = false;
    let mut start_col = 0;

    for col in 0..max_width {
        // Check if this column is all spaces (in number rows)
        let all_spaces = padded[..num_rows - 1]
            .iter()
            .all(|row| row.chars().nth(col).unwrap_or(' ') == ' ');

        // Also check operator row
        let op_char = operator_row.chars().nth(col).unwrap_or(' ');
        let col_empty = all_spaces && (op_char == ' ');

        if !col_empty && !in_problem {
            // Start of a new problem
            in_problem = true;
            start_col = col;
        } else if col_empty && in_problem {
            // End of current problem
            problems.push((start_col, col));
            in_problem = false;
        }
    }
    // Handle last problem if it extends to end
    if in_problem {
        problems.push((start_col, max_width));
    }

    let mut grand_total: u128 = 0;

    // Process each problem
    for (start, end) in problems {
        // Extract the operator from the last row
        let op_slice: String = operator_row[start..end].chars().collect();
        let operator = if op_slice.contains('*') { '*' } else { '+' };

        // Extract numbers from each row (except last)
        let mut numbers: Vec<u128> = Vec::new();
        for row in &padded[..num_rows - 1] {
            let slice: String = row[start..end].chars().collect();
            let trimmed = slice.trim();
            if !trimmed.is_empty() {
                if let Ok(n) = trimmed.parse::<u128>() {
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

