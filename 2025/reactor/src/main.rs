use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Advent of Code 2025 - Day 11: Reactor
///
/// Count all paths from device "you" to device "out" in a directed graph.
/// Data flows only forward through device outputs (DAG).
///
/// Strategy: DFS with memoization - count paths from each node to "out".
/// For node N: paths(N) = sum of paths(child) for all children of N
/// Base case: paths("out") = 1

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 1: Parse device connections into adjacency list
    // Format: "device: output1 output2 output3"
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
    // PHASE 2: Count paths from "you" to "out" using memoized DFS
    // ═══════════════════════════════════════════════════════════════════════════
    let mut memo: HashMap<String, u64> = HashMap::new();
    let path_count = count_paths("you", &graph, &mut memo);

    // ═══════════════════════════════════════════════════════════════════════════
    // FINAL ANSWER: Total number of distinct paths
    // ═══════════════════════════════════════════════════════════════════════════
    println!("{}", path_count);
    Ok(())
}

/// Recursively count paths from `node` to "out" with memoization
fn count_paths(node: &str, graph: &HashMap<String, Vec<String>>, memo: &mut HashMap<String, u64>) -> u64 {
    // Base case: reached the destination
    if node == "out" {
        return 1;
    }

    // Check memo
    if let Some(&count) = memo.get(node) {
        return count;
    }

    // Get outputs for this device
    let outputs = match graph.get(node) {
        Some(outs) => outs,
        None => return 0, // Dead end - no outputs defined
    };

    // Sum paths through all outputs
    let mut total: u64 = 0;
    for output in outputs {
        total += count_paths(output, graph, memo);
    }

    // Memoize and return
    memo.insert(node.to_string(), total);
    total
}
