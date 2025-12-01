use std::fs::File;
use std::io::{self,BufRead, BufReader};

/// Advent of Code 2025 - Day 1: Password Puzzle
/// 
/// This program simulates a circular dial with positions 0-99.
/// Starting at position 50, it processes rotation instructions (L/R + distance)
/// and counts how many times the dial lands on position 0.
fn main() -> io::Result<()> {
    // Read the puzzle input file containing rotation instruction
    let path = "puzzle_input.txt";

    // Open the file and create a Buffered reader for efficient line-by-line reading
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    // Initialize starting position at 50 and on a cicular dial (0-99)
    let mut position: i32 = 50;

    // Track how many times we land on positon 0 (the password)
    let mut zero_count = 0;

    // Process each rotation instruction
    for line in reader.lines() {
        let rot = line?;
        if rot.is_empty() { continue }

        // Parse the instruction: first character is the direction (L/R), rest is distance
        let direction = &rot[0..1];
        let distance: i32 = rot[1..].parse().expect("Error");


        // Apply rotation with wrap-around using rem_euclid (handles negative numbers correctly)
        // L = counterclockwise (subtract), R = clockwise (add)
        if direction == "L" {
            position = (position - distance).rem_euclid(100);
        } else {
            position = (position + distance).rem_euclid(100);
        }

        // Check if position is ) after rotation
        if position == 0 {
            zero_count += 1;
        }
    }

    // Output the final answer
    print!("Password: {}", zero_count);
    
    Ok(())
}