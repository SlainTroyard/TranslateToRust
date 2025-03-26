use std::io::{self, BufRead};

fn dfs(x: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[x] = true;
    for &nxt in &adj[x] {
        if !visited[nxt] {
            dfs(nxt, adj, visited);
        }
    }
}

fn remaining_methods(n: usize, k: usize, invocations: &Vec<Vec<usize>>) -> Vec<usize> {
    // Create adjacency list for the graph
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    
    // Build the graph
    for invocation in invocations {
        let a = invocation[0];
        let b = invocation[1];
        adj[a].push(b);
    }
    
    // Track suspicious methods (reachable from k)
    let mut suspicious: Vec<bool> = vec![false; n];
    
    // Perform DFS from k to mark all reachable methods
    dfs(k, &adj, &mut suspicious);
    
    // Check if there's a non-suspicious method that invokes a suspicious one
    for invocation in invocations {
        let a = invocation[0];
        let b = invocation[1];
        if !suspicious[a] && suspicious[b] {
            // If found, return all methods
            return (0..n).collect();
        }
    }
    
    // Otherwise, return only non-suspicious methods
    (0..n).filter(|&i| !suspicious[i]).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n, k, and invocationsSize
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let invocations_size: usize = iter.next().unwrap().parse().unwrap();
    
    // Read invocations
    let mut invocations: Vec<Vec<usize>> = Vec::with_capacity(invocations_size);
    for _ in 0..invocations_size {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        invocations.push(vec![a, b]);
    }
    
    // Call the function
    let ans = remaining_methods(n, k, &invocations);
    
    // Print the result
    for (i, &val) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
}