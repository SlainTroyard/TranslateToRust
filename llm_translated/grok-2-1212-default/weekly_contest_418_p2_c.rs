use std::io::{self, BufRead};
use std::collections::VecDeque;

fn dfs(x: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[x] = true;
    for &nxt in &adj[x] {
        if !visited[nxt] {
            dfs(nxt, adj, visited);
        }
    }
}

fn remaining_methods(n: usize, k: usize, invocations: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut adj = vec![Vec::new(); n];
    let mut suspicious = vec![false; n];

    for invocation in invocations {
        let a = invocation[0];
        let b = invocation[1];
        adj[a].push(b);
    }

    dfs(k, &adj, &mut suspicious);

    for invocation in invocations {
        if !suspicious[invocation[0]] && suspicious[invocation[1]] {
            return (0..n).collect();
        }
    }

    (0..n).filter(|&i| !suspicious[i]).collect()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse input
    let first_line: Vec<usize> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (n, k, invocations_size) = (first_line[0], first_line[1], first_line[2]);

    let mut invocations = Vec::new();
    for _ in 0..invocations_size {
        let line: Vec<usize> = lines.next().unwrap()?.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        invocations.push(line);
    }

    // Process and get result
    let result = remaining_methods(n, k, &invocations);

    // Print output
    for &num in &result {
        print!("{} ", num);
    }
    println!();

    Ok(())
}