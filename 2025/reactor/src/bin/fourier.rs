use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Advent of Code 2025 - Day 11: Reactor (Part 2)
///
/// Count paths from "svr" to "out" that visit BOTH "dac" and "fft" (in any order).
///
/// Strategy: DFS with state = (current_node, visited_mask)
/// - Bit 0: have we visited "dac"?
/// - Bit 1: have we visited "fft"?
/// Only count paths reaching "out" with mask == 3 (both visited)

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 1: Parse device connections into adjacency list
    // ═══════════════════════════════════════════════════════════════════════════
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines().filter_map(|l| l.ok()).filter(|l| !l.is_empty()) {
        let parts: Vec<&str> = line.split(':').collect();
        let device = parts[0].trim().to_string();
        let outputs: Vec<String> = parts[1]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        graph.insert(device, outputs);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 2: Count paths from "svr" to "out" visiting both "dac" and "fft"
    // Use memoized DFS with state = (node, visited_mask)
    // ═══════════════════════════════════════════════════════════════════════════
    let mut memo: HashMap<(String, u8), u64> = HashMap::new();
    let path_count = count_paths("svr", 0, &graph, &mut memo);

    // ═══════════════════════════════════════════════════════════════════════════
    // FINAL ANSWER
    // ═══════════════════════════════════════════════════════════════════════════
    println!("{}", path_count);
    Ok(())
}

/// Recursively count paths from `node` to "out" that have both dac and fft visited
/// `mask` tracks: bit 0 = visited dac, bit 1 = visited fft
fn count_paths(
    node: &str,
    mask: u8,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<(String, u8), u64>,
) -> u64 {
    // Update mask based on current node
    let mut current_mask = mask;
    if node == "dac" {
        current_mask |= 1; // Set bit 0
    }
    if node == "fft" {
        current_mask |= 2; // Set bit 1
    }

    // Base case: reached destination
    if node == "out" {
        // Only count if we visited BOTH dac and fft (mask == 3)
        return if current_mask == 3 { 1 } else { 0 };
    }

    // Check memo
    let key = (node.to_string(), current_mask);
    if let Some(&count) = memo.get(&key) {
        return count;
    }

    // Get outputs for this device
    let outputs = match graph.get(node) {
        Some(outs) => outs,
        None => return 0, // Dead end
    };

    // Sum paths through all outputs
    let mut total: u64 = 0;
    for output in outputs {
        total += count_paths(output, current_mask, graph, memo);
    }

    // Memoize and return
    memo.insert(key, total);
    total
}
