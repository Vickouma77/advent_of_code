use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Advent of Code 2025 - Day 9: Movie Theater (Part 2)
///
/// Red tiles form a polygon boundary. Find the largest rectangle using two
/// red tiles as corners, where the ENTIRE rectangle lies within or on the polygon.
///
/// Strategy: Coordinate compression + point-in-polygon testing.
/// - Compress coordinates to only the unique x and y values from red tiles
/// - For each compressed cell, check if it's inside the polygon
/// - Rectangle is valid if all cells in its compressed range are inside

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 1: Parse red tile positions (they form a closed loop polygon)
    // ═══════════════════════════════════════════════════════════════════════════
    let red_tiles: Vec<(i64, i64)> = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let parts: Vec<i64> = l.split(',').map(|s| s.trim().parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    let n = red_tiles.len();

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 2: Coordinate compression
    // Only track the unique x and y values - reduces grid size dramatically
    // ═══════════════════════════════════════════════════════════════════════════
    let mut x_coords: Vec<i64> = red_tiles.iter().map(|&(x, _)| x).collect();
    let mut y_coords: Vec<i64> = red_tiles.iter().map(|&(_, y)| y).collect();
    
    x_coords.sort();
    x_coords.dedup();
    y_coords.sort();
    y_coords.dedup();
    
    let x_to_idx: HashMap<i64, usize> = x_coords.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let y_to_idx: HashMap<i64, usize> = y_coords.iter().enumerate().map(|(i, &y)| (y, i)).collect();
    
    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 3: Build polygon edges for ray casting
    // Store edges as (x1, y1) -> (x2, y2) line segments
    // ═══════════════════════════════════════════════════════════════════════════
    let edges: Vec<((i64, i64), (i64, i64))> = (0..n)
        .map(|i| (red_tiles[i], red_tiles[(i + 1) % n]))
        .collect();

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 4: Point-in-polygon check using ray casting
    // A point is inside if a horizontal ray to the right crosses an odd number
    // of edges. Points ON the boundary are considered inside.
    // ═══════════════════════════════════════════════════════════════════════════
    let point_inside = |px: i64, py: i64| -> bool {
        // First check if point is on the boundary
        for &((x1, y1), (x2, y2)) in &edges {
            // Check if point is on this edge segment
            if x1 == x2 && px == x1 {
                // Vertical edge
                let (min_y, max_y) = (y1.min(y2), y1.max(y2));
                if py >= min_y && py <= max_y {
                    return true; // On boundary
                }
            } else if y1 == y2 && py == y1 {
                // Horizontal edge
                let (min_x, max_x) = (x1.min(x2), x1.max(x2));
                if px >= min_x && px <= max_x {
                    return true; // On boundary
                }
            }
        }
        
        // Ray casting: count edge crossings to the right of point
        let mut crossings = 0;
        for &((x1, y1), (x2, y2)) in &edges {
            // Only consider vertical edges (horizontal edges never cross horizontal ray)
            if x1 == x2 {
                let edge_x = x1;
                let (min_y, max_y) = (y1.min(y2), y1.max(y2));
                
                // Edge must be to the right of point and span the y level
                // Use half-open interval to avoid double-counting vertices
                if edge_x > px && py >= min_y && py < max_y {
                    crossings += 1;
                }
            }
        }
        crossings % 2 == 1
    };

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 5: Precompute which compressed cells are inside the polygon
    // A compressed cell (xi, yi) represents the region from x_coords[xi] to
    // x_coords[xi+1] and y_coords[yi] to y_coords[yi+1]
    // Check the cell's corner point to determine inside/outside
    // ═══════════════════════════════════════════════════════════════════════════
    let width = x_coords.len();
    let height = y_coords.len();
    
    let mut inside: HashSet<(usize, usize)> = HashSet::new();
    for xi in 0..width {
        for yi in 0..height {
            let px = x_coords[xi];
            let py = y_coords[yi];
            if point_inside(px, py) {
                inside.insert((xi, yi));
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 6: Find largest valid rectangle
    // Try all pairs of red tiles as corners, check if entire rectangle is inside
    // ═══════════════════════════════════════════════════════════════════════════
    let mut max_area: i64 = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];

            // Rectangle bounds (inclusive)
            let (rx1, rx2) = (x1.min(x2), x1.max(x2));
            let (ry1, ry2) = (y1.min(y2), y1.max(y2));

            // Area calculation (inclusive)
            let area = (rx2 - rx1 + 1) * (ry2 - ry1 + 1);
            if area <= max_area { continue; }

            // Get compressed indices for corners
            let xi1 = *x_to_idx.get(&rx1).unwrap();
            let xi2 = *x_to_idx.get(&rx2).unwrap();
            let yi1 = *y_to_idx.get(&ry1).unwrap();
            let yi2 = *y_to_idx.get(&ry2).unwrap();

            // Check all corner points of the rectangle in compressed space
            // IMPORTANT: We need to check that all grid points within are inside
            let mut valid = true;
            'outer: for xi in xi1..=xi2 {
                for yi in yi1..=yi2 {
                    if !inside.contains(&(xi, yi)) {
                        valid = false;
                        break 'outer;
                    }
                }
            }

            if valid {
                max_area = area;
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // FINAL ANSWER
    // ═══════════════════════════════════════════════════════════════════════════
    println!("{}", max_area);
    Ok(())
}
