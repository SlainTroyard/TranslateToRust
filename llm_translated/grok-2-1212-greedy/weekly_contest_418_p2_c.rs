use std::io::{self, BufRead};

fn dfs(x: usize, adj: &[Vec<usize>], suspicious: &mut [bool]) {
    suspicious[x] = true;
    for &nxt in &adj[x] {
        if !suspicious[nxt] {
            dfs(nxt, adj, suspicious);
        }
    }
}

fn remaining_methods(n: usize, k: usize, invocations: &[Vec<usize>]) -> Vec<usize> {
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

    let first_line: Vec<usize> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = first_line[0];
    let k = first_line[1];
    let invocations_size = first_line[2];

    let mut invocations = Vec::new();
    for _ in 0..invocations_size {
        let line: Vec<usize> = lines.next().unwrap()?.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        invocations.push(line);
    }

    let ans = remaining_methods(n, k, &invocations);

    for num in ans {
        print!("{} ", num);
    }
    println!();

    Ok(())
}