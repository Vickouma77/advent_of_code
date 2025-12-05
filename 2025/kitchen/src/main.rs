use std::fs;

/// Advent of Code 2025 - Day 5: Cafeteria
///
/// Problem: The kitchen's new inventory management system stores ingredient freshness
/// as ID ranges. We need to determine which available ingredients are fresh.
///
/// Input: A database with two sections separated by a blank line:
///   1. Fresh ingredient ID ranges (e.g., "3-5" means IDs 3, 4, 5 are fresh)
///   2. Available ingredient IDs to check
///
/// Output: Count of available ingredient IDs that fall within any fresh range
///
/// Key insight: Ranges are inclusive and can overlap - an ingredient is fresh
/// if it falls into ANY of the given ranges.

fn main() {
    // Read the puzzle input file containing the ingredient database
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    
    // Split input into ranges and ingredient IDs sections (separated by blank line)
    let parts: Vec<&str> = input.split("\n\n").collect();
    let ranges_section = parts[0];
    let ingredients_section = parts[1];
    
    // Parse the fresh ingredient ID ranges into (start, end) tuples
    // Each line is formatted as "start-end" where both bounds are inclusive
    let ranges: Vec<(u64, u64)> = ranges_section
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect();
    
    // Parse available ingredient IDs (one per line after the blank line)
    let ingredients: Vec<u64> = ingredients_section
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();
    
    // Count fresh ingredients: those that fall into at least one range
    // An ingredient ID is fresh if: start <= id <= end for any range
    let fresh_count = ingredients
        .iter()
        .filter(|&&id| ranges.iter().any(|&(start, end)| id >= start && id <= end))
        .count();

    println!("Number of fresh ingredient IDs: {}", fresh_count);
}
