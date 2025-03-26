use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse first line: n, k, invocationsSize
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let invocations_size: usize = iter.next().unwrap().parse().unwrap();
    
    // Parse invocations
    let mut invocations = Vec::with_capacity(invocations_size);
    for _ in 0..invocations_size {
        let line = lines.next().unwrap()?;
        let mut iter = line.split_whitespace();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        invocations.push(vec![a, b]);
    }
    
    // Create invocations_col_size (not actually used in the algorithm but kept for compatibility)
    let invocations_col_size = vec![2; invocations_size];
    
    // Call the function
    let result = remaining_methods(n, k, &invocations, invocations_size, &invocations_col_size);
    
    // Print the result
    for i in 0..result.len() {
        print!("{} ", result[i]);
    }
    println!();
    
    Ok(())
}

fn dfs(x: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[x] = true;
    for &nxt in &adj[x] {
        if !visited[nxt] {
            dfs(nxt, adj, visited);
        }
    }
}

fn remaining_methods(n: usize, k: usize, invocations: &Vec<Vec<usize>>, invocations_size: usize, _invocations_col_size: &Vec<usize>) -> Vec<usize> {
    // Initialize adjacency list
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    
    // Build adjacency list
    for i in 0..invocations_size {
        let a = invocations[i][0];
        let b = invocations[i][1];
        adj[a].push(b);
    }
    
    // Track suspicious methods (those reachable from k)
    let mut suspicious = vec![false; n];
    
    // Perform DFS from k to mark all reachable methods
    dfs(k, &adj, &mut suspicious);
    
    // Check if there's a non-suspicious method calling a suspicious one
    for i in 0..invocations_size {
        let a = invocations[i][0];
        let b = invocations[i][1];
        if !suspicious[a] && suspicious[b] {
            // If such method exists, return all methods (0 to n-1)
            return (0..n).collect();
        }
    }
    
    // Otherwise, return only the non-suspicious methods
    (0..n).filter(|&i| !suspicious[i]).collect()
}