use std::io;

/// Translates the C code for LeetCode Weekly Contest 418 Problem 2 into idiomatic Rust.
///
/// The problem requires finding methods not reachable from a starting point 'k' after processing invocations.
/// The solution uses DFS to mark reachable nodes and checks for specific invocation conditions.
///
/// # Input Handling
/// Reads integers from stdin in the format:
/// n k invocations_size
/// a_0 b_0
/// a_1 b_1
/// ...
/// a_{invocations_size-1} b_{invocations_size-1}
///
/// # Output
/// Prints space-separated integers of non-reachable methods or all methods if a specific condition is met.
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("Invalid integer"));

    let n = tokens.next().expect("Missing n");
    let k = tokens.next().expect("Missing k");
    let invocations_size = tokens.next().expect("Missing invocations_size");

    let mut invocations = Vec::with_capacity(invocations_size);
    for _ in 0..invocations_size {
        let a = tokens.next().expect("Missing a");
        let b = tokens.next().expect("Missing b");
        invocations.push((a, b));
    }

    let result = remaining_methods(n, k, &invocations);

    // Output the result with space-separated integers followed by a newline
    for (i, &num) in result.iter().enumerate() {
        print!("{}", num);
        if i < result.len() - 1 {
            print!(" ");
        }
    }
    println!();
}

/// Computes the remaining methods not reachable from 'k' or returns all methods if a condition is met.
///
/// # Arguments
/// * `n` - Number of methods.
/// * `k` - Starting method for DFS.
/// * `invocations` - List of method invocations (a, b) indicating a calls b.
///
/// # Returns
/// A vector of method indices not reachable from 'k', unless an invalid invocation exists.
fn remaining_methods(n: usize, k: usize, invocations: &[(usize, usize)]) -> Vec<usize> {
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n]; // Adjacency list
    let mut suspicious: Vec<bool> = vec![false; n]; // Tracks reachable nodes

    // Build adjacency list
    for &(a, b) in invocations {
        adj[a].push(b);
    }

    // Mark reachable nodes via DFS starting at 'k'
    dfs(k, &adj, &mut suspicious);

    // Check for any invalid invocation (a not reachable but b is)
    for &(a, b) in invocations {
        if !suspicious[a] && suspicious[b] {
            return (0..n).collect(); // Return all nodes if found
        }
    }

    // Collect non-reachable nodes
    (0..n).filter(|&i| !suspicious[i]).collect()
}

/// Depth-first search to mark reachable nodes starting from 'x'.
///
/// # Arguments
/// * `x` - Current node to explore.
/// * `adj` - Adjacency list of nodes.
/// * `visited` - Mutable vector marking visited nodes.
fn dfs(x: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[x] = true;
    for &nxt in &adj[x] {
        if !visited[nxt] {
            dfs(nxt, adj, visited);
        }
    }
}