use std::io::{self, BufRead};

/// Performs a depth-first search to mark visited (suspicious) methods starting from `x`.
fn dfs(x: usize, adj: &[Vec<usize>], visited: &mut [bool]) {
    visited[x] = true;
    for &nxt in &adj[x] {
        if !visited[nxt] {
            dfs(nxt, adj, visited);
        }
    }
}

/// Translated version of `remainingMethods` from C to Rust.
/// Builds an adjacency list, runs DFS from `k`, checks for suspicious conditions,
/// and returns the resulting list of methods.
fn remaining_methods(n: usize, k: usize, invocations: &[(usize, usize)]) -> Vec<usize> {
    // Adjacency list: for each method, store the list of methods it invokes
    let mut adj = vec![Vec::new(); n];
    for &(a, b) in invocations {
        // In the C code, we reallocated arrays as needed, but here we can just push in Rust
        adj[a].push(b);
    }

    // This vector tracks which methods are suspicious (reachable from k)
    let mut suspicious = vec![false; n];

    // Mark all methods reachable from k as suspicious
    dfs(k, &adj, &mut suspicious);

    // If there's a transition from a non-suspicious method to a suspicious method,
    // return all methods 0..n.
    // (mimics the early return in the C code)
    for &(a, b) in invocations {
        if !suspicious[a] && suspicious[b] {
            return (0..n).collect();
        }
    }

    // Otherwise, return only those methods that are not suspicious
    let mut ans = Vec::new();
    for i in 0..n {
        if !suspicious[i] {
            ans.push(i);
        }
    }
    ans
}

fn main() {
    // Read n, k, and invocations_size from input
    // (same format as the C code: "%d %d %d")
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut parts = line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();
    let invocations_size: usize = parts.next().unwrap().parse().unwrap();

    // Read the invocations pairs (a, b)
    // (same format as the C code, two integers per line)
    let stdin = io::stdin();
    let mut invocations = Vec::with_capacity(invocations_size);
    for _ in 0..invocations_size {
        let mut line_inv = String::new();
        stdin.read_line(&mut line_inv).unwrap();
        let mut nums = line_inv.split_whitespace();
        let a: usize = nums.next().unwrap().parse().unwrap();
        let b: usize = nums.next().unwrap().parse().unwrap();
        invocations.push((a, b));
    }

    // Compute the result according to the same logic as the C code
    let ans = remaining_methods(n, k, &invocations);

    // Print the result in the exact same format as the C code (space-separated, then newline)
    for x in ans {
        print!("{} ", x);
    }
    println!();
}