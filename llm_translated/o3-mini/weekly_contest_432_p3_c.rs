use std::collections::VecDeque;
use std::io::{self, BufRead};

/// Represents an edge as a tuple of (src, dest, weight)
type Edge = (i32, i32, i32);

/// Finds the maximum edge weight among all edges.
fn find_max_weight(edges: &[Edge]) -> i32 {
    let mut max_weight = 0;
    for &(_, _, weight) in edges {
        if weight > max_weight {
            max_weight = weight;
        }
    }
    max_weight
}

/// Checks if all nodes can be reached from node 0 using edges with weight <= limit.
/// Note: The edge is reversed when constructing the graph, i.e. we add an edge from
/// edges[i].1 -> edges[i].0. This is kept exactly as in the original C code.
fn check(n: usize, edges: &[Edge], limit: i32) -> bool {
    // Create an adjacency list with n vertices.
    // Each vertex will have a vector of adjacent vertices.
    let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); n];

    // Build the adjacency list using only edges with weight <= limit.
    for &(src, dest, weight) in edges {
        if weight <= limit {
            // Reverse edge as in the original C code: add edge from dest -> src.
            // Convert to usize (assuming valid vertices 0..n)
            let from = dest as usize;
            let to = src as usize;
            adj_list[from].push(to);
        }
    }

    // BFS starting from node 0.
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();

    visited[0] = true;
    queue.push_back(0);

    while let Some(current) = queue.pop_front() {
        for &adj in &adj_list[current] {
            if !visited[adj] {
                visited[adj] = true;
                queue.push_back(adj);
            }
        }
    }

    // Check if all nodes were visited.
    visited.into_iter().all(|v| v)
}

/// Performs binary search over possible maximum edge weights to find the minimal
/// maximum weight such that all nodes can be reached from node 0 using only edges
/// with weight <= that limit.
///
/// The parameter `threshold` is accepted to match the original signature,
/// though it is not used in the solution.
fn min_max_weight(n: usize, edges: &[Edge], threshold: i32) -> i32 {
    let max_weight = find_max_weight(edges);

    // Check if it's possible to reach all nodes even with max weight.
    if !check(n, edges, max_weight) {
        return -1;
    }

    let mut left = 0;
    let mut right = max_weight;

    // Binary search for the minimal maximum edge weight that still lets us reach all nodes.
    while left < right {
        let mid = left + (right - left) / 2;
        if check(n, edges, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

/// Reads input from stdin and returns a vector of tokens as Strings.
fn read_tokens() -> Vec<String> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    reader
        .lines()
        .filter_map(|line| line.ok())
        .flat_map(|line| line.split_whitespace().map(String::from).collect::<Vec<_>>())
        .collect()
}

fn main() {
    // Read all tokens from standard input
    let tokens = read_tokens();
    let mut iter = tokens.into_iter();

    // Parse n and threshold from the first two tokens.
    let n: usize = match iter.next() {
        Some(token) => token.parse().unwrap_or_else(|_| {
            eprintln!("Error parsing n");
            std::process::exit(1);
        }),
        None => {
            eprintln!("Error reading input for n");
            std::process::exit(1);
        }
    };

    let threshold: i32 = match iter.next() {
        Some(token) => token.parse().unwrap_or_else(|_| {
            eprintln!("Error parsing threshold");
            std::process::exit(1);
        }),
        None => {
            eprintln!("Error reading input for threshold");
            std::process::exit(1);
        }
    };

    // Read the remaining tokens as edges (triplets: src, dest, weight)
    // We'll store them in a vector of (i32, i32, i32).
    let mut edges: Vec<Edge> = Vec::new();
    loop {
        // Try to read three tokens per edge.
        let src_token = match iter.next() {
            Some(token) => token,
            None => break, // End of input
        };
        let dest_token = match iter.next() {
            Some(token) => token,
            None => break,
        };
        let weight_token = match iter.next() {
            Some(token) => token,
            None => break,
        };

        let src = src_token.parse::<i32>().unwrap_or_else(|_| {
            eprintln!("Error parsing src");
            std::process::exit(1);
        });
        let dest = dest_token.parse::<i32>().unwrap_or_else(|_| {
            eprintln!("Error parsing dest");
            std::process::exit(1);
        });
        let weight = weight_token.parse::<i32>().unwrap_or_else(|_| {
            eprintln!("Error parsing weight");
            std::process::exit(1);
        });

        edges.push((src, dest, weight));
    }

    // Call the solution function and print the result.
    let result = min_max_weight(n, &edges, threshold);
    println!("{}", result);
}