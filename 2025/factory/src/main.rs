use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Advent of Code 2025 - Day 10: Factory
///
/// Each machine has indicator lights (initially off) that must match a target pattern.
/// Buttons toggle specific lights (XOR). Find minimum total button presses across all machines.
///
/// Key insight: Pressing a button twice = no effect, so each button is pressed 0 or 1 times.
/// This is finding the minimum Hamming weight solution to a system of linear equations over GF(2).
/// 
/// Strategy: Enumerate all 2^n subsets of buttons (n ≤ ~20 typically), checking if the
/// combination produces the target pattern, tracking the minimum number of presses.

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total_presses: u64 = 0;

    for line in reader.lines().filter_map(|l| l.ok()).filter(|l| !l.is_empty()) {
        // ═══════════════════════════════════════════════════════════════════════════
        // PHASE 1: Parse the indicator light diagram [.##.]
        // ═══════════════════════════════════════════════════════════════════════════
        let bracket_start = line.find('[').unwrap();
        let bracket_end = line.find(']').unwrap();
        let diagram = &line[bracket_start + 1..bracket_end];
        
        // Target state as a bitmask: bit i = 1 means light i should be ON
        let mut target: u64 = 0;
        for (i, c) in diagram.chars().enumerate() {
            if c == '#' {
                target |= 1 << i;
            }
        }

        // ═══════════════════════════════════════════════════════════════════════════
        // PHASE 2: Parse button wiring schematics (0,3,4) (1,2) etc.
        // Each button is a bitmask of which lights it toggles
        // ═══════════════════════════════════════════════════════════════════════════
        let mut buttons: Vec<u64> = Vec::new();
        let rest = &line[bracket_end + 1..];
        
        // Find all parenthesized groups
        let mut i = 0;
        let chars: Vec<char> = rest.chars().collect();
        while i < chars.len() {
            if chars[i] == '(' {
                // Find matching close paren
                let start = i + 1;
                while i < chars.len() && chars[i] != ')' {
                    i += 1;
                }
                let content: String = chars[start..i].iter().collect();
                
                // Parse the light indices
                let mut button_mask: u64 = 0;
                for part in content.split(',') {
                    let idx: usize = part.trim().parse().unwrap();
                    button_mask |= 1 << idx;
                }
                buttons.push(button_mask);
            }
            i += 1;
        }

        // ═══════════════════════════════════════════════════════════════════════════
        // PHASE 3: Find minimum presses via brute force over button subsets
        // Each subset represents which buttons to press once
        // XOR all selected button masks and check if result equals target
        // ═══════════════════════════════════════════════════════════════════════════
        let num_buttons = buttons.len();
        let mut min_presses = u64::MAX;

        for subset in 0..(1u64 << num_buttons) {
            // XOR together all buttons in this subset
            let mut state: u64 = 0;
            for b in 0..num_buttons {
                if (subset >> b) & 1 == 1 {
                    state ^= buttons[b];
                }
            }

            // Check if this produces the target
            if state == target {
                let presses = subset.count_ones() as u64;
                min_presses = min_presses.min(presses);
            }
        }

        // If no solution found, this machine can't be configured (shouldn't happen per puzzle)
        if min_presses == u64::MAX {
            eprintln!("Warning: No solution found for machine with diagram {}", diagram);
            min_presses = 0;
        }

        total_presses += min_presses;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // FINAL ANSWER: Sum of minimum presses across all machines
    // ═══════════════════════════════════════════════════════════════════════════
    println!("{}", total_presses);
    Ok(())
}
