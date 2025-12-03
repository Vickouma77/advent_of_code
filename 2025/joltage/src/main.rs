use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Advent of Code 2025 - Day 3: Joltage (Part 2)
///
/// Problem: For each line, find the largest 12-digit number that can be formed
/// by selecting exactly 12 digits while maintaining their original order.
///
/// Example: "987654321111111" → select 12 digits → "987654321111"
/// Strategy: Greedily pick the largest digit available at each position,
/// ensuring enough digits remain to complete the 12-digit number.

/// Finds the largest number formed by selecting exactly `k` digits in order.
fn max_k_digits(s: &str, k: usize) -> Option<String> {
    // Extract all digits from the string
    let digits: Vec<char> = s.chars().filter(|c| c.is_ascii_digit()).collect();

    // Need at least k digits
    if digits.len() < k {
        return None;
    }

    let mut result = String::with_capacity(k);
    let mut start = 0; // Current search start position

    // Pick k digits one by one
    for remaining in (1..=k).rev() {
        // Find the largest digit in the valid range
        // We must leave enough digits for the remaining positions
        let end = digits.len() - remaining + 1;

        // Find the position of the maximum digit in range [start, end)
        let mut best_pos = start;
        for i in start..end {
            if digits[i] > digits[best_pos] {
                best_pos = i;
            }
        }

        // Add this digit to result and move start past it
        result.push(digits[best_pos]);
        start = best_pos + 1;
    }

    Some(result)
}

fn main() -> io::Result<()> {
    let file = File::open("joltage.txt")?;
    let reader = BufReader::new(file);

    let mut total: u128 = 0;

    // Process each line (each "bank")
    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() { continue; }

        match max_k_digits(&line, 12) {
            Some(max_val) => {
                let num: u128 = max_val.parse().unwrap();
                println!("Bank {:>3}: max = {}", idx + 1, max_val);
                total += num;
            }
            None => {
                println!("Bank {:>3}: (not enough digits)", idx + 1);
            }
        }
    }

    println!("Total output joltage = {}", total);
    Ok(())
}
