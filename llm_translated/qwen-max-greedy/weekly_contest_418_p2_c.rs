use std::io::{self, BufRead, Write};

fn dfs(x: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[x] = true;
    for &nxt in &adj[x] {
        if !visited[nxt] {
            dfs(nxt, adj, visited);
        }
    }
}

fn remaining_methods(n: usize, k: usize, invocations: &Vec<(usize, usize)>, return_size: &mut usize) -> Vec<usize> {
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut suspicious: Vec<bool> = vec![false; n];

    for &(a, b) in invocations.iter() {
        adj[a].push(b);
    }

    dfs(k, &adj, &mut suspicious);

    for &(a, b) in invocations.iter() {
        if !suspicious[a] && suspicious[b] {
            *return_size = n;
            return (0..n).collect();
        }
    }

    let count = suspicious.iter().filter(|&&v| !v).count();
    *return_size = count;
    (0..n).filter(|&i| !suspicious[i]).collect()
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let invocations_size: usize = iter.next().unwrap().parse().unwrap();

    let mut invocations: Vec<(usize, usize)> = Vec::with_capacity(invocations_size);
    for _ in 0..invocations_size {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        invocations.push((a, b));
    }

    let mut return_size: usize = 0;
    let ans = remaining_methods(n, k, &invocations, &mut return_size);

    for (i, &val) in ans.iter().enumerate() {
        write!(out, "{}", val).unwrap();
        if i < return_size - 1 {
            write!(out, " ").unwrap();
        }
    }
    writeln!(out).unwrap();
}