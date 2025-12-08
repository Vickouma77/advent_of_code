use std::fs;

/// Advent of Code 2025 - Day 8: Playground
///
/// Connect the 1000 closest pairs of junction boxes using Union-Find,
/// then multiply the sizes of the three largest circuits.

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    
    // Parse positions
    let pos: Vec<(i64, i64, i64)> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let c: Vec<i64> = l.split(',').map(|s| s.parse().unwrap()).collect();
            (c[0], c[1], c[2])
        })
        .collect();
    
    let n = pos.len();
    
    // Generate all pairs sorted by distance
    let mut pairs: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in i+1..n {
            let (dx, dy, dz) = (pos[i].0 - pos[j].0, pos[i].1 - pos[j].1, pos[i].2 - pos[j].2);
            pairs.push((dx*dx + dy*dy + dz*dz, i, j));
        }
    }
    pairs.sort_by_key(|p| p.0);
    
    // Union-Find with path compression
    let mut parent: Vec<usize> = (0..n).collect();
    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x { parent[x] = find(parent, parent[x]); }
        parent[x]
    }
    
    // Connect 1000 closest pairs
    for &(_, i, j) in pairs.iter().take(1000) {
        let (ri, rj) = (find(&mut parent, i), find(&mut parent, j));
        if ri != rj { parent[ri] = rj; }
    }
    
    // Count circuit sizes
    let mut sizes = std::collections::HashMap::new();
    for i in 0..n { *sizes.entry(find(&mut parent, i)).or_insert(0) += 1; }
    
    // Get product of 3 largest
    let mut s: Vec<u64> = sizes.values().map(|&v| v as u64).collect();
    s.sort_by(|a, b| b.cmp(a));
    
    println!("Product of three largest circuit sizes: {}", s.iter().take(3).product::<u64>());
}


