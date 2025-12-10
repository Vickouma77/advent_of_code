use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Advent of Code 2025 - Day 10: Factory (Part 2)
///
/// Buttons ADD 1 to counters. Find minimum total button presses to reach targets.
///
/// This is Integer Linear Programming: minimize sum(x) where A*x = b, x >= 0.
///
/// Key insight: Use Gaussian elimination over rationals to find the solution space,
/// then search over free variables to minimize total presses.
/// 
/// For a system with n buttons and m counters:
/// - Reduce to row echelon form
/// - Express dependent variables in terms of free variables
/// - Iterate over valid non-negative integer assignments to free vars

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total_presses: u64 = 0;

    for line in reader.lines().filter_map(|l| l.ok()).filter(|l| !l.is_empty()) {
        // ═══════════════════════════════════════════════════════════════════════════
        // PHASE 1: Parse joltage requirements {3,5,4,7}
        // ═══════════════════════════════════════════════════════════════════════════
        let brace_start = line.find('{').unwrap();
        let brace_end = line.find('}').unwrap();
        let target: Vec<i64> = line[brace_start + 1..brace_end]
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        let num_counters = target.len();

        // ═══════════════════════════════════════════════════════════════════════════
        // PHASE 2: Parse button wiring schematics into matrix form
        // ═══════════════════════════════════════════════════════════════════════════
        let mut buttons: Vec<Vec<usize>> = Vec::new();
        let bracket_end = line.find(']').unwrap();
        let rest = &line[bracket_end + 1..brace_start];

        let mut i = 0;
        let chars: Vec<char> = rest.chars().collect();
        while i < chars.len() {
            if chars[i] == '(' {
                let start = i + 1;
                while i < chars.len() && chars[i] != ')' {
                    i += 1;
                }
                let content: String = chars[start..i].iter().collect();
                let indices: Vec<usize> = content
                    .split(',')
                    .map(|s| s.trim().parse().unwrap())
                    .collect();
                buttons.push(indices);
            }
            i += 1;
        }

        let num_buttons = buttons.len();

        // Build matrix A where A[counter][button] = 1 if button affects counter
        // We solve A * x = target for x (button press counts)
        let mut matrix: Vec<Vec<i64>> = vec![vec![0; num_buttons + 1]; num_counters];
        for (b, button) in buttons.iter().enumerate() {
            for &c in button {
                if c < num_counters {
                    matrix[c][b] = 1;
                }
            }
        }
        // Augmented matrix: last column is target
        for c in 0..num_counters {
            matrix[c][num_buttons] = target[c];
        }

        // ═══════════════════════════════════════════════════════════════════════════
        // PHASE 3: Gaussian elimination to find solution structure
        // ═══════════════════════════════════════════════════════════════════════════
        let (reduced, pivot_cols) = gaussian_eliminate(matrix, num_buttons);
        
        // Find free variables (columns without pivots)
        let free_vars: Vec<usize> = (0..num_buttons)
            .filter(|c| !pivot_cols.contains(c))
            .collect();

        // ═══════════════════════════════════════════════════════════════════════════
        // PHASE 4: Search over free variable assignments
        // ═══════════════════════════════════════════════════════════════════════════
        let max_val = *target.iter().max().unwrap() as usize;
        let mut best = u64::MAX;
        
        search_free_vars(
            &reduced,
            &pivot_cols,
            &free_vars,
            num_buttons,
            max_val,
            0,
            &mut vec![0i64; num_buttons],
            0,
            &mut best,
        );

        total_presses += best;
    }

    println!("{}", total_presses);
    Ok(())
}

/// Gaussian elimination returning reduced matrix and pivot column indices
fn gaussian_eliminate(mut matrix: Vec<Vec<i64>>, num_vars: usize) -> (Vec<Vec<i64>>, Vec<usize>) {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut pivot_cols = Vec::new();
    let mut pivot_row = 0;

    for col in 0..num_vars {
        // Find pivot
        let mut found = None;
        for row in pivot_row..rows {
            if matrix[row][col] != 0 {
                found = Some(row);
                break;
            }
        }

        if let Some(pr) = found {
            // Swap rows
            matrix.swap(pivot_row, pr);
            pivot_cols.push(col);

            // Eliminate below and above
            let pivot_val = matrix[pivot_row][col];
            for row in 0..rows {
                if row != pivot_row && matrix[row][col] != 0 {
                    let factor = matrix[row][col];
                    for c in 0..cols {
                        matrix[row][c] = matrix[row][c] * pivot_val - matrix[pivot_row][c] * factor;
                    }
                }
            }

            pivot_row += 1;
        }
    }

    (matrix, pivot_cols)
}

/// Recursively search over free variable assignments
fn search_free_vars(
    reduced: &[Vec<i64>],
    pivot_cols: &[usize],
    free_vars: &[usize],
    num_buttons: usize,
    max_val: usize,
    free_idx: usize,
    solution: &mut Vec<i64>,
    current_sum: u64,
    best: &mut u64,
) {
    // Prune
    if current_sum >= *best {
        return;
    }

    // Base case: all free vars assigned
    if free_idx == free_vars.len() {
        // Compute dependent variables from reduced matrix
        let mut valid = true;
        let mut total = current_sum;

        for (row, &pivot_col) in pivot_cols.iter().enumerate() {
            // For each pivot row: pivot_val * x[pivot_col] = rhs - sum(coef * free_var)
            let pivot_val = reduced[row][pivot_col];
            if pivot_val == 0 {
                continue;
            }

            let mut rhs = reduced[row][num_buttons]; // augmented column
            for &fv in free_vars {
                rhs -= reduced[row][fv] * solution[fv];
            }

            // x[pivot_col] = rhs / pivot_val (must be non-negative integer)
            if rhs % pivot_val != 0 {
                valid = false;
                break;
            }
            let val = rhs / pivot_val;
            if val < 0 {
                valid = false;
                break;
            }
            solution[pivot_col] = val;
            total += val as u64;
            
            if total >= *best {
                valid = false;
                break;
            }
        }

        if valid {
            *best = total;
        }
        return;
    }

    // Try values for this free variable
    let fv = free_vars[free_idx];
    for val in 0..=max_val as i64 {
        if current_sum + val as u64 >= *best {
            break;
        }
        solution[fv] = val;
        search_free_vars(
            reduced, pivot_cols, free_vars, num_buttons, max_val,
            free_idx + 1, solution, current_sum + val as u64, best,
        );
    }
}
