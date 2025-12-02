use std::fs::File;
use std::io::{self, Read};

/// Advent of Code 2025 - Day 2: Gift Shop (Part 2)
///
/// Problem: Find IDs that are made by repeating a pattern 2+ times.
/// Unlike Part 1 (only halves), this checks ALL possible repeat patterns.
///
/// Examples:
/// - 1212 → "12" repeated 2x → true
/// - 123123 → "123" repeated 2x → true
/// - 111 → "1" repeated 3x → true
/// - 121212 → "12" repeated 3x → true
/// - 1234 → no repeating pattern → false

/// Checks if a string is made by repeating a smaller pattern 2+ times.
fn is_repeating(s: &str) -> bool {
    let len = s.len();
    if len < 2 { return false; }

    // Try each possible pattern length (1 to len/2)
    for pattern_len in 1..=(len / 2) {
        // Pattern must divide evenly into the string
        if len % pattern_len != 0 { continue; }

        let pattern = &s[..pattern_len];
        let repeats = len / pattern_len;

        // Check if repeating the pattern gives us the original string
        if pattern.repeat(repeats) == s {
            return true;
        }
    }
    false
}

fn main() -> io::Result<()> {
    // Read the puzzle input
    let mut contents = String::new();
    File::open("input.txt")?.read_to_string(&mut contents)?;

    let mut total_sum: u128 = 0;
    let mut total_count: u64 = 0;

    // Process each comma-separated range (e.g., "100-200,300-400")
    for range in contents.trim().split(',') {
        let range = range.trim();
        if range.is_empty() { continue; }

        // Parse "start-end" into two numbers
        let mut parts = range.split('-');
        let start: u128 = parts.next().unwrap().trim().parse().expect("bad start");
        let end: u128 = parts.next().unwrap().trim().parse().expect("bad end");

        // Check each ID in the range
        for id in start..=end {
            if is_repeating(&id.to_string()) {
                total_sum += id;
                total_count += 1;
            }
        }
    }

    println!("Sum of invalid IDs: {}", total_sum);
    println!("Count of invalid IDs: {}", total_count);
    Ok(())
}
