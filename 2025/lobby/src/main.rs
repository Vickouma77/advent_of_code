use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Advent of Code 2025 - Day 3: Lobby
///
/// Problem: For each line, find the maximum two-digit number that can be formed
/// by picking any digit as the tens place and any digit to its RIGHT as the ones place.
///
/// Example: "a1b9c3d7" has digits [1,9,3,7]
/// - Possible pairs: (1,9)=19, (1,3)=13, (1,7)=17, (9,3)=93, (9,7)=97, (3,7)=37
/// - Maximum = 97

/// Finds the maximum two-digit number from digits in a string.
/// The tens digit must appear BEFORE the ones digit in the string.
fn max_two_digit(s: &str) -> Option<u32> {
    // Extract all digits from the string
    let digits: Vec<u32> = s
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    if digits.len() < 2 {
        return None;
    }

    let mut best = 0;

    // Try every pair where i < j (tens digit before ones digit)
    for i in 0..digits.len() {
        for j in (i + 1)..digits.len() {
            let val = 10 * digits[i] + digits[j];
            if val > best {
                best = val;
            }
        }
    }

    Some(best)
}

fn main() -> io::Result<()> {
    let file = File::open("joltage.txt")?;
    let reader = BufReader::new(file);

    let mut total: u64 = 0;

    // Process each line (each "bank")
    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() { continue; }

        match max_two_digit(&line) {
            Some(max_val) => {
                println!("Bank {:>3}: max = {}", idx + 1, max_val);
                total += max_val as u64;
            }
            None => {
                println!("Bank {:>3}: (too short)", idx + 1);
            }
        }
    }

    println!("Total output joltage = {}", total);
    Ok(())
}
