// Translated from C to Rust, preserving the same algorithm and I/O format.
// Original problem: LeetCode Weekly Contest 432 Problem 3

use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::process;

// Find the maximum weight among all edges
fn find_max_weight(edges: &Vec<[i32; 3]>) -> i32 {
    // If there are no edges, max weight is 0 by default
    edges.iter().map(|e| e[2]).max().unwrap_or(0)
}

// Check if all nodes can be reached from node 0 when using only
// edges with weight <= limit (the edges are used in reverse).
fn check(n: usize, edges: &Vec<[i32; 3]>, limit: i32) -> bool {
    // Create adjacency list, using only edges with weight <= limit,
    // and reversing the direction (adj_list[dest] gets src).
    let mut adj_list = vec![Vec::new(); n];
    for edge in edges {
        if edge[2] <= limit {
            let src = edge[0] as usize;
            let dest = edge[1] as usize;
            adj_list[dest].push(src);
        }
    }

    // BFS to see if we can visit all nodes from node 0
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();

    visited[0] = true;
    queue.push_back(0);

    while let Some(current_vertex) = queue.pop_front() {
        for &adj_vertex in &adj_list[current_vertex] {
            if !visited[adj_vertex] {
                visited[adj_vertex] = true;
                queue.push_back(adj_vertex);
            }
        }
    }

    // Check if all nodes were visited
    visited.iter().all(|&v| v)
}

// Main solution function which performs a binary search
// to find the minimum maximum edge weight.
fn min_max_weight(n: usize, edges: &Vec<[i32; 3]>, _threshold: i32) -> i32 {
    // 1) Find max weight among edges
    let max_weight = find_max_weight(edges);

    // 2) If we can't reach all nodes using all edges, return -1 immediately
    if !check(n, edges, max_weight) {
        return -1;
    }

    // 3) Binary search over the range of possible weights
    let mut left = 0;
    let mut right = max_weight;

    while left < right {
        // mid = left + (right - left) / 2
        let mid = left + (right - left) / 2;
        if check(n, edges, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn main() {
    // Read the first line containing n and threshold
    let mut first_line = String::new();
    if io::stdin().read_line(&mut first_line).unwrap_or(0) == 0 {
        eprintln!("Error reading input for n and threshold");
        process::exit(1);
    }

    // Parse n and threshold
    let mut split = first_line.split_whitespace();
    let n_str = split.next();
    let threshold_str = split.next();

    let (n, threshold) = match (n_str, threshold_str) {
        (Some(ns), Some(ts)) => {
            let n_parsed = ns.parse::<i32>();
            let t_parsed = ts.parse::<i32>();
            match (n_parsed, t_parsed) {
                (Ok(nn), Ok(tt)) => (nn, tt),
                _ => {
                    eprintln!("Error reading input for n and threshold");
                    process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Error reading input for n and threshold");
            process::exit(1);
        }
    };

    // The original C code uses a large capacity (100000) for edges and warns if exceeded
    const CAPACITY: usize = 100000;
    let mut edges = Vec::with_capacity(CAPACITY);

    // Read all remaining lines for edges (src, dest, weight) until EOF
    let stdin = io::stdin();
    let mut edges_size = 0;
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break, // If any error occurs, stop reading
        };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }

        // Split into tokens and parse exactly three integers
        let parts: Vec<_> = trimmed.split_whitespace().collect();
        if parts.len() < 3 {
            break;
        }

        let src = match parts[0].parse::<i32>() {
            Ok(val) => val,
            Err(_) => break,
        };
        let dest = match parts[1].parse::<i32>() {
            Ok(val) => val,
            Err(_) => break,
        };
        let weight = match parts[2].parse::<i32>() {
            Ok(val) => val,
            Err(_) => break,
        };

        // Capacity check (same as original C code)
        if edges_size >= CAPACITY {
            eprintln!("Warning: Maximum edge capacity reached ({} edges)", CAPACITY);
            break;
        }

        // Store the edge
        edges.push([src, dest, weight]);
        edges_size += 1;
    }

    // Call the main solution function
    let result = min_max_weight(n as usize, &edges, threshold);

    // Print the result
    println!("{}", result);
}