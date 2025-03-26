use std::io::{self, BufRead};

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

    for &(a, b) in invocations {
        adj[a].push(b);
    }

    dfs(k, &adj, &mut suspicious);

    for &(a, b) in invocations {
        if !suspicious[a] && suspicious[b] {
            *return_size = n;
            return (0..n).collect();
        }
    }

    let count = suspicious.iter().filter(|&&x| !x).count();
    *return_size = count;
    (0..n).filter(|&i| !suspicious[i]).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut input_line = String::new();

    stdin_lock.read_line(&mut input_line).expect("Failed to read line");
    let mut iter = input_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let k: usize = iter.next().unwrap().parse().expect("Failed to parse k");
    let invocations_size: usize = iter.next().unwrap().parse().expect("Failed to parse invocations size");

    let mut invocations: Vec<(usize, usize)> = Vec::with_capacity(invocations_size);
    for _ in 0..invocations_size {
        input_line.clear();
        stdin_lock.read_line(&mut input_line).expect("Failed to read line");
        let mut iter = input_line.split_whitespace();
        let a: usize = iter.next().unwrap().parse().expect("Failed to parse a");
        let b: usize = iter.next().unwrap().parse().expect("Failed to parse b");
        invocations.push((a, b));
    }

    let mut return_size: usize = 0;
    let ans = remaining_methods(n, k, &invocations, &mut return_size);

    for i in 0..return_size {
        print!("{} ", ans[i]);
    }
    println!();
}