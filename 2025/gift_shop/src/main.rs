use std::fs::File;
use std::io::{self, Read};

/// Advent of Code 2025 - Day 2: Gift Shop
///
/// Problem: The gift shop has product IDs, but some are "invalid" because
/// their digits form a repeating pattern (first half equals second half).
///
/// Input: Comma-separated ranges like "100-200,300-400"
/// Output: Sum of all invalid IDs within the given ranges

/// Checks if a number's digits repeat: first half equals second half.
///
/// Examples:
/// - 1212 → "12" == "12" → true (invalid)
/// - 123123 → "123" == "123" → true (invalid)
/// - 1234 → "12" != "34" → false (valid)
/// - 123 → odd digits → false (valid)
fn repeated_twice(n: u64) -> bool {
    let s = n.to_string();
    // Must have even length AND first half must equal second half
    s.len() % 2 == 0 && s[..s.len() / 2] == s[s.len() / 2..]
}

fn main() -> io::Result<()> {
    // Read the puzzle input file
    let mut contents = String::new();
    File::open("input.txt")?.read_to_string(&mut contents)?;

    let total_sum: u64 = contents
        .trim()                             // Remove leading/trailing whitespace
        .split(',')                         // Split into ranges: ["100-200", "300-400"]
        .filter(|s| !s.is_empty())          // Skip empty strings from trailing commas
        .flat_map(|range| {                 // Expand each "start-end" into all IDs
            let mut parts = range.split('-');
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            start..=end                     // Returns iterator over start, start+1, ..., end
        })
        .filter(|&id| repeated_twice(id))   // Keep only invalid IDs (repeating pattern)
        .sum();                             // Sum all invalid IDs

    println!("Sum of invalid IDs: {total_sum}");
    Ok(())
}
