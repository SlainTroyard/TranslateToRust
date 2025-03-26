// LeetCode Weekly Contest 426 Problem 3
//
// This Rust program preserves the same algorithm and I/O format as the original C code,
// but uses more idiomatic Rust for memory management and data structures.

use std::io::{self, BufRead};

/// Recursively counts the number of ways to walk in `k` steps from `node`, 
/// ensuring we do not revisit the `visited` node immediately.
fn linepots(k: i32, adjacency: &[Vec<i32>], node: i32, visited: i32) -> i32 {
    // Base case: if no steps are allowed (k < 0), return 0
    if k == -1 {
        return 0;
    }
    // Base case: if exactly 0 steps remain, count this path (1)
    if k == 0 {
        return 1;
    }
    // Start counting from the current node
    let mut answer = 1;
    // Explore all neighbors except the one we came from
    for &neighbor in &adjacency[node as usize] {
        if neighbor != visited {
            answer += linepots(k - 1, adjacency, neighbor, node);
        }
    }
    answer
}

/// Builds an adjacency list from a list of edges. 
/// Each edge is a pair [a, b] indicating an undirected connection between `a` and `b`.
fn build_adjacency(edges: &[[i32; 2]]) -> Vec<Vec<i32>> {
    let max_node = edges.iter().map(|[a, b]| a.max(*b)).max().unwrap_or(0);
    let size = (max_node + 1) as usize;
    let mut adjacency = vec![Vec::new(); size];
    for &[a, b] in edges {
        adjacency[a as usize].push(b);
        adjacency[b as usize].push(a);
    }
    adjacency
}

/// Computes the maximum target nodes array for two trees, given up to k steps in tree1
/// plus the best possible (k-1)-step walk in tree2.
fn max_target_nodes(edges1: &[[i32; 2]], edges2: &[[i32; 2]], k: i32) -> Vec<i32> {
    // Build adjacency lists
    let pots1 = build_adjacency(edges1);
    let pots2 = build_adjacency(edges2);

    // The largest node index in each adjacency list
    let len1 = pots1.len().saturating_sub(1);
    let len2 = pots2.len().saturating_sub(1);

    // Find the maximum number of ways to walk in (k-1) steps from any node in pots2
    let mut max_ways_in_tree2 = 0;
    if k > 0 {
        for i in 0..=len2 {
            let temp = linepots(k - 1, &pots2, i as i32, -1);
            if temp > max_ways_in_tree2 {
                max_ways_in_tree2 = temp;
            }
        }
    }

    // For each node in tree1, calculate the total paths by adding the best from tree2
    let mut answer = vec![0; len1 + 1];
    for i in 0..=len1 {
        answer[i] = linepots(k, &pots1, i as i32, -1) + max_ways_in_tree2;
    }

    answer
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of edges in the first tree
    let n1 = lines.next().unwrap()?.trim().parse::<usize>().unwrap();
    // Read edges1
    let mut edges1 = vec![[0i32; 2]; n1];
    for i in 0..n1 {
        let line = lines.next().unwrap()?;
        let parts: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        edges1[i] = [parts[0], parts[1]];
    }

    // Read the number of edges in the second tree
    let n2 = lines.next().unwrap()?.trim().parse::<usize>().unwrap();
    // Read edges2
    let mut edges2 = vec![[0i32; 2]; n2];
    for i in 0..n2 {
        let line = lines.next().unwrap()?;
        let parts: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        edges2[i] = [parts[0], parts[1]];
    }

    // Read k
    let k = lines.next().unwrap()?.trim().parse::<i32>().unwrap();

    // Compute the result
    let result = max_target_nodes(&edges1, &edges2, k);

    // Output the result in the same format as the C code
    for val in result {
        print!("{} ", val);
    }
    println!();

    Ok(())
}