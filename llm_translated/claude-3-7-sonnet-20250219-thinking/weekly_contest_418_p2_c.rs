use std::io::{self, BufRead};

/// Performs a depth-first search starting from node x, marking visited nodes
fn dfs(x: usize, adj: &[Vec<usize>], visited: &mut [bool]) {
    visited[x] = true;
    for &nxt in &adj[x] {
        if !visited[nxt] {
            dfs(nxt, adj, visited);
        }
    }
}

fn remaining_methods(n: usize, k: usize, invocations: &[Vec<usize>]) -> Vec<usize> {
    // Create adjacency list for the graph
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut suspicious = vec![false; n];

    // Build the adjacency list from invocations
    for invocation in invocations {
        let a = invocation[0];
        let b = invocation[1];
        adj[a].push(b);
    }

    // Mark all methods reachable from k as suspicious
    dfs(k, &adj, &mut suspicious);

    // If there's a non-suspicious method that calls a suspicious method,
    // all methods are potentially vulnerable
    for invocation in invocations {
        if !suspicious[invocation[0]] && suspicious[invocation[1]] {
            return (0..n).collect();
        }
    }

    // Otherwise, return only non-suspicious methods
    let mut ans = Vec::new();
    for i in 0..n {
        if !suspicious[i] {
            ans.push(i);
        }
    }
    
    ans
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse first line: n, k, invocationsSize
    let first_line = lines.next().unwrap()?;
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();
    let invocations_size: usize = parts.next().unwrap().parse().unwrap();
    
    // Parse invocations (each line contains two integers)
    let mut invocations = Vec::with_capacity(invocations_size);
    for _ in 0..invocations_size {
        let line = lines.next().unwrap()?;
        let values: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        invocations.push(values);
    }
    
    // Call the solution function
    let ans = remaining_methods(n, k, &invocations);
    
    // Print the result with space-separated values, followed by a newline
    for (i, val) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
    
    Ok(())
}