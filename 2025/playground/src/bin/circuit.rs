use std::fs;

/// Advent of Code 2025 - Day 8: Playground (Part 2)
///
/// Keep connecting the closest unconnected pairs of junction boxes
/// until they're all in one circuit. Find the last pair that completes
/// the circuit and multiply their X coordinates.

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
    
    // Union-Find with path compression and size tracking
    let mut parent: Vec<usize> = (0..n).collect();
    let mut size: Vec<usize> = vec![1; n];
    
    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x { parent[x] = find(parent, parent[x]); }
        parent[x]
    }
    
    // Connect closest pairs until all are in one circuit
    let mut last_pair: (usize, usize) = (0, 0);
    let mut num_components = n;
    
    for &(_, i, j) in &pairs {
        let (ri, rj) = (find(&mut parent, i), find(&mut parent, j));
        if ri != rj {
            // Union by size
            if size[ri] < size[rj] {
                parent[ri] = rj;
                size[rj] += size[ri];
            } else {
                parent[rj] = ri;
                size[ri] += size[rj];
            }
            num_components -= 1;
            last_pair = (i, j);
            
            // Check if all connected
            if num_components == 1 {
                break;
            }
        }
    }
    
    // Get X coordinates of the last pair
    let x1 = pos[last_pair.0].0;
    let x2 = pos[last_pair.1].0;
    
    println!("Last connection: ({},{},{}) and ({},{},{})", 
             pos[last_pair.0].0, pos[last_pair.0].1, pos[last_pair.0].2,
             pos[last_pair.1].0, pos[last_pair.1].1, pos[last_pair.1].2);
    println!("Product of X coordinates: {}", x1 * x2);
}
