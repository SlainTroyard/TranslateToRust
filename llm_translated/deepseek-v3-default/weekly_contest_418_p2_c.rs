use std::collections::VecDeque;
use std::io::{self, BufRead};

fn dfs(x: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[x] = true;
    for &nxt in &adj[x] {
        if !visited[nxt] {
            dfs(nxt, adj, visited);
        }
    }
}

fn remaining_methods(
    n: usize,
    k: usize,
    invocations: &Vec<Vec<usize>>,
) -> Vec<usize> {
    let mut adj = vec![Vec::new(); n];
    let mut suspicious = vec![false; n];

    for invocation in invocations {
        let a = invocation[0];
        let b = invocation[1];
        adj[a].push(b);
    }

    dfs(k, &adj, &mut suspicious);

    for invocation in invocations {
        let a = invocation[0];
        let b = invocation[1];
        if !suspicious[a] && suspicious[b] {
            return (0..n).collect();
        }
    }

    (0..n).filter(|&i| !suspicious[i]).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let n: usize = first_line_iter.next().unwrap().parse().unwrap();
    let k: usize = first_line_iter.next().unwrap().parse().unwrap();
    let invocations_size: usize = first_line_iter.next().unwrap().parse().unwrap();
    
    let mut invocations = Vec::new();
    for _ in 0..invocations_size {
        let line = lines.next().unwrap().unwrap();
        let mut line_iter = line.split_whitespace();
        let a: usize = line_iter.next().unwrap().parse().unwrap();
        let b: usize = line_iter.next().unwrap().parse().unwrap();
        invocations.push(vec![a, b]);
    }
    
    let result = remaining_methods(n, k, &invocations);
    
    for &item in &result {
        print!("{} ", item);
    }
    println!();
}