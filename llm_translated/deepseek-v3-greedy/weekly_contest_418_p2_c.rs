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

    let mut ans = Vec::new();
    for i in 0..n {
        if !suspicious[i] {
            ans.push(i);
        }
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();
    let invocations_size: usize = parts.next().unwrap().parse().unwrap();

    let mut invocations = Vec::new();
    for _ in 0..invocations_size {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let a: usize = parts.next().unwrap().parse().unwrap();
        let b: usize = parts.next().unwrap().parse().unwrap();
        invocations.push(vec![a, b]);
    }

    let ans = remaining_methods(n, k, &invocations);
    for &i in &ans {
        print!("{} ", i);
    }
    println!();
}