use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Advent of Code 2025 - Day 9: Movie Theater
///
/// Problem: Find the largest rectangle using any two red tiles as opposite corners.
/// The area of a rectangle with corners (x1,y1) and (x2,y2) is |x2-x1| * |y2-y1|.
///
/// Strategy: Try all pairs of red tiles and compute the rectangle area.

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Parse red tile positions
    let tiles: Vec<(i64, i64)> = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let parts: Vec<i64> = l.split(',').map(|s| s.trim().parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    let n = tiles.len();
    let mut max_area: i64 = 0;

    // Try all pairs of tiles as opposite corners
    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];

            // Rectangle area with these two as opposite corners (inclusive)
            let width = (x2 - x1).abs() + 1;
            let height = (y2 - y1).abs() + 1;
            let area = width * height;

            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("Largest rectangle area: {}", max_area);
    Ok(())
}

