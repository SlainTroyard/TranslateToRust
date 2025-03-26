use std::io::{self, Read};

fn remaining_methods(n: usize, k: usize, invocations: &[(usize, usize)]) -> Vec<usize> {
    // Build adjacency list from invocations
    let mut adj = vec![vec![]; n];
    for &(a, b) in invocations {
        adj[a].push(b);
    }

    // Mark all reachable methods from k using DFS
    let mut suspicious = vec![false; n];
    dfs(k, &adj, &mut suspicious);

    // Check if any invocation (a, b) where a is not suspicious but b is
    for &(a, b) in invocations {
        if !suspicious[a] && suspicious[b] {
            return (0..n).collect();
        }
    }

    // Collect all non-suspicious methods
    suspicious
        .into_iter()
        .enumerate()
        .filter(|&(_, s)| !s)
        .map(|(i, _)| i)
        .collect()
}

fn dfs(x: usize, adj: &[Vec<usize>], suspicious: &mut [bool]) {
    suspicious[x] = true;
    for &next in &adj[x] {
        if !suspicious[next] {
            dfs(next, adj, suspicious);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<usize>().unwrap());

    let n = tokens.next().unwrap();
    let k = tokens.next().unwrap();
    let invocations_size = tokens.next().unwrap();

    let mut invocations = Vec::with_capacity(invocations_size);
    for _ in 0..invocations_size {
        let a = tokens.next().unwrap();
        let b = tokens.next().unwrap();
        invocations.push((a, b));
    }

    let ans = remaining_methods(n, k, &invocations);

    // Print the result space-separated
    for (i, &num) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", num);
    }
    println!();
}