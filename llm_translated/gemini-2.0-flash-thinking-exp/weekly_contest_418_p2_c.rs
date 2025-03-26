// Problem: Weekly Contest 418 Problem 2
use std::io::{self, BufRead};
use std::vec;

fn dfs(x: i32, adj: &[Vec<i32>], visited: &mut [i32]) {
    visited[x as usize] = 1;
    if let Some(neighbors) = adj.get(x as usize) {
        for &nxt in neighbors {
            if visited[nxt as usize] == 0 {
                dfs(nxt, adj, visited);
            }
        }
    }
}

fn remaining_methods(n: i32, k: i32, invocations: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut adj: Vec<Vec<i32>> = vec![Vec::new(); n as usize];

    for invocation in invocations {
        let a = invocation[0];
        let b = invocation[1];
        adj[a as usize].push(b);
    }

    let mut suspicious: Vec<i32> = vec![0; n as usize];
    dfs(k, &adj, &mut suspicious);

    for invocation in invocations {
        let a = invocation[0];
        let b = invocation[1];
        if suspicious[a as usize] == 0 && suspicious[b as usize] == 1 {
            let mut ans: Vec<i32> = Vec::new();
            for j in 0..n {
                ans.push(j);
            }
            return ans;
        }
    }

    let mut ans: Vec<i32> = Vec::new();
    for i in 0..n {
        if suspicious[i as usize] == 0 {
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
    let n: i32 = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    let invocations_size: i32 = parts.next().unwrap().parse().unwrap();

    let mut invocations: Vec<Vec<i32>> = Vec::new();
    for _ in 0..invocations_size {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();
        invocations.push(vec![a, b]);
    }

    let ans = remaining_methods(n, k, &invocations);

    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
    println!();
}