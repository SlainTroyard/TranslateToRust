```rust
use std::io;
use std::collections::VecDeque;

fn dfs(
    x: usize, 
    adj: &Vec<Vec<usize>>, 
    visited: &mut Vec<bool>
) {
    visited[x] = true;
    for &next in &adj[x] {
        if !visited[next] {
            dfs(next, adj, visited);
        }
    }
}

fn remaining_methods(
    n: usize, 
    k: usize, 
    invocations: &Vec<(usize, usize)>
) -> Vec<usize> {

    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut suspicious = vec![false; n];
    
    // building the adjacency list
    for &(a, b) in invocations.iter(){
        adj[a].push(b);
    }

    dfs(k,&adj, &mut suspicious)
}