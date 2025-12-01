use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Advent of Code 2025 - Day 1: Password Puzzle (Part 2)
///
/// Imagine a circular dial like a combination lock with positions 0-99.
/// The dial starts at position 50. Each instruction rotates the dial
/// left (L) or right (R) by a certain number of clicks.
///
/// Unlike Part 1 (which only counts final landing positions), Part 2 counts
/// every time the dial PASSES THROUGH position 0 during each rotation.
/// For example, "R200" from position 50 would pass through 0 twice!

fn main() -> io::Result<()> {
    // Open the puzzle input file containing rotation instructions
    let file = File::open("puzzle_input.txt")?;
    let reader = BufReader::new(file);

    // The dial starts at position 50 (middle of the 0-99 range)
    let mut position: i64 = 50;

    // Count every time the dial crosses position 0
    let mut zero_count: i64 = 0;

    // Process each rotation instruction (e.g., "L25", "R150")
    for line in reader.lines() {
        let rot = line?;
        if rot.is_empty() { continue; }

        // Parse direction (L=left/counterclockwise, R=right/clockwise) and distance
        let direction = &rot[0..1];
        let distance: i64 = rot[1..].parse().expect("Error");

        // Calculate how many steps until we first hit position 0
        // - Moving LEFT from position P: we need P steps to reach 0
        // - Moving RIGHT from position P: we need (100 - P) steps to reach 0
        let steps_to_zero = if direction == "L" {
            position.rem_euclid(100)
        } else {
            (100 - position).rem_euclid(100)
        };

        // If we're already at 0, the next hit is 100 steps away (full rotation)
        let first = if steps_to_zero == 0 { 100 } else { steps_to_zero };

        // Count how many times we cross 0:
        // - First crossing at 'first' steps, then every 100 steps after that
        if distance >= first {
            zero_count += 1 + (distance - first) / 100;
        }

        // Update the dial position after this rotation
        // rem_euclid handles wrap-around correctly (e.g., -10 mod 100 = 90)
        if direction == "L" {
            position = (position - distance).rem_euclid(100);
        } else {
            position = (position + distance).rem_euclid(100);
        }
    }

    // Output the total number of times we passed through position 0
    println!("Password: {}", zero_count);
    Ok(())
}