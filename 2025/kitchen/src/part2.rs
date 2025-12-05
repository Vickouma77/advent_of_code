use std::fs;

/// Advent of Code 2025 - Day 5: Cafeteria (Part 2)
///
/// Problem: Count the total number of unique ingredient IDs that are considered
/// fresh according to all the fresh ingredient ID ranges.
///
/// Input: Fresh ingredient ID ranges (e.g., "3-5" means IDs 3, 4, 5 are fresh)
///        The available ingredient IDs section is now irrelevant.
///
/// Output: Total count of unique fresh ingredient IDs across all ranges
///
/// Key insight: Ranges can overlap, so we need to merge overlapping ranges
/// to avoid counting the same ID twice. We sort ranges by start, then merge
/// any that overlap or are adjacent.

fn main() {
    // Read the puzzle input file containing the ingredient database
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    
    // Only need the ranges section (before the blank line)
    let ranges_section = input.split("\n\n").next().unwrap();
    
    // Parse the fresh ingredient ID ranges into (start, end) tuples
    let mut ranges: Vec<(u64, u64)> = ranges_section
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect();
    
    // Sort ranges by start value to prepare for merging
    ranges.sort_by_key(|&(start, _)| start);
    
    // Merge overlapping and adjacent ranges
    // This ensures we don't count the same ID twice
    let mut merged: Vec<(u64, u64)> = Vec::new();
    
    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            // Check if current range overlaps or is adjacent to the last merged range
            // Use start <= last.1 + 1 to handle adjacent ranges (e.g., 3-5 and 6-8)
            if start <= last.1 + 1 {
                // Extend the last range if current range extends further
                last.1 = last.1.max(end);
            } else {
                // No overlap, add as a new range
                merged.push((start, end));
            }
        } else {
            // First range
            merged.push((start, end));
        }
    }
    
    // Count total fresh IDs: sum of (end - start + 1) for each merged range
    let total_fresh: u64 = merged
        .iter()
        .map(|&(start, end)| end - start + 1)
        .sum();

    println!("Total fresh ingredient IDs: {}", total_fresh);
}
