use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n, k, and invocationsSize
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();
    let invocations_size: usize = parts.next().unwrap().parse().unwrap();

    // Read invocations
    let mut invocations = Vec::with_capacity(invocations_size);
    for _ in 0..invocations_size {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let u: usize = parts.next().unwrap().parse().unwrap();
        let v: usize = parts.next().unwrap().parse().unwrap();
        invocations.push(vec![u, v]);
    }

    // Solve the problem
    let ans = remaining_methods(n, k, &invocations);

    // Print the result
    for x in ans {
        print!("{} ", x);
    }
    println!();
}

fn remaining_methods(n: usize, k: usize, invocations: &Vec<Vec<usize>>) -> Vec<usize> {
    // Build the graph
    let mut g = vec![Vec::new(); n];
    for e in invocations {
        g[e[0]].push(e[1]);
    }

    // Mark all suspicious methods using DFS
    let mut is_suspicious = vec![false; n];
    let mut stack = VecDeque::new();
    stack.push_back(k);
    while let Some(x) = stack.pop_back() {
        if !is_suspicious[x] {
            is_suspicious[x] = true;
            for &y in &g[x] {
                if !is_suspicious[y] {
                    stack.push_back(y);
                }
            }
        }
    }

    // Check if there is an edge from a non-suspicious method to a suspicious one
    for e in invocations {
        if !is_suspicious[e[0]] && is_suspicious[e[1]] {
            // Cannot remove suspicious methods, return all methods
            return (0..n).collect();
        }
    }

    // Remove all suspicious methods
    let mut ans = Vec::new();
    for i in 0..n {
        if !is_suspicious[i] {
            ans.push(i);
        }
    }
    ans
}