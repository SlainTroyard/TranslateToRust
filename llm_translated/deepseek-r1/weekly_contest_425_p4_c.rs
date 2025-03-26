use std::io::{self, BufRead};
use std::cmp::Ordering;

struct StackNode {
    node: usize,
    parent: usize,
    processed: bool,
}

fn maximize_sum_of_weights(edges: &[(usize, usize, i32)], n: usize, k: usize) -> i64 {
    // Build adjacency list
    let mut adj: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
    for &(u, v, w) in edges {
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    let mut dp0 = vec![0i64; n];
    let mut dp1 = vec![0i64; n];

    let mut stack = Vec::with_capacity(n);
    stack.push(StackNode {
        node: 0,
        parent: n, // Using n as a sentinel value for the root's parent
        processed: false,
    });

    while let Some(current) = stack.pop() {
        if !current.processed {
            // Push back as processed to handle children first
            stack.push(StackNode {
                node: current.node,
                parent: current.parent,
                processed: true,
            });

            // Push all children (excluding parent) onto the stack
            for &(child, weight) in &adj[current.node] {
                if child != current.parent {
                    stack.push(StackNode {
                        node: child,
                        parent: current.node,
                        processed: false,
                    });
                }
            }
        } else {
            // Process the node after its children are processed
            let mut gains = Vec::new();
            let mut sum_dp0 = 0;

            // Calculate gains and sum_dp0 for all children
            for &(child, weight) in &adj[current.node] {
                if child != current.parent {
                    sum_dp0 += dp0[child];
                    let gain = weight as i64 + dp1[child] - dp0[child];
                    gains.push(gain);
                }
            }

            // Sort gains in descending order
            gains.sort_unstable_by(|a, b| b.cmp(a));

            // Calculate dp0: sum_dp0 + top k positive gains
            let mut sum0 = sum_dp0;
            let mut i = 0;
            let limit = k.min(gains.len());
            while i < limit {
                if gains[i] > 0 {
                    sum0 += gains[i];
                    i += 1;
                } else {
                    break;
                }
            }
            dp0[current.node] = sum0;

            // Calculate dp1: sum_dp0 + top (k-1) positive gains
            let mut sum1 = sum_dp0;
            let limit = if k > 0 { (k - 1).min(gains.len()) } else { 0 };
            i = 0;
            while i < limit {
                if gains[i] > 0 {
                    sum1 += gains[i];
                    i += 1;
                } else {
                    break;
                }
            }
            dp1[current.node] = sum1;
        }
    }

    dp0[0]
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and k from the first line
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    // Read n-1 edges
    let mut edges = Vec::with_capacity(n - 1);
    for _ in 0..(n - 1) {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let u: usize = parts.next().unwrap().parse().unwrap();
        let v: usize = parts.next().unwrap().parse().unwrap();
        let w: i32 = parts.next().unwrap().parse().unwrap();
        edges.push((u, v, w));
    }

    // Call the function and print the result
    let result = maximize_sum_of_weights(&edges, n, k);
    println!("{}", result);
}