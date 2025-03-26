// Problem: Weekly Contest 418 Problem 2
use std::collections::VecDeque;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn remaining_methods(
        n: usize,
        k: usize,
        invocations: Vec<[usize; 2]>,
    ) -> Vec<usize> {
        // Adjacency list representation for the graph
        let mut graph = vec![Vec::new(); n];
        for e in &invocations {
            graph[e[0]].push(e[1]);
        }

        // Vector to track suspicious methods
        let mut is_suspicious = vec![false; n];

        // Depth-first search (DFS) for traversing the graph
        fn dfs(graph: &[Vec<usize>], is_suspicious: &mut Vec<bool>, x: usize) {
            is_suspicious[x] = true;
            for &y in &graph[x] {
                if !is_suspicious[y] {
                    dfs(graph, is_suspicious, y);
                }
            }
        }
        dfs(&graph, &mut is_suspicious, k);

        // Check if there exists a "non-suspicious method" -> "suspicious method" edge
        for e in &invocations {
            if !is_suspicious[e[0]] && is_suspicious[e[1]] {
                // If such an edge exists, return all methods (indices 0 to n-1)
                return (0..n).collect();
            }
        }

        // Filter out all suspicious methods
        (0..n).filter(|&i| !is_suspicious[i]).collect()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n, k, and invocations_size
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_split = first_line.split_whitespace();
    let n: usize = first_line_split.next().unwrap().parse().unwrap();
    let k: usize = first_line_split.next().unwrap().parse().unwrap();
    let invocations_size: usize = first_line_split.next().unwrap().parse().unwrap();

    // Read the invocations
    let mut invocations = Vec::with_capacity(invocations_size);
    for _ in 0..invocations_size {
        let line = lines.next().unwrap().unwrap();
        let mut split = line.split_whitespace();
        let from: usize = split.next().unwrap().parse().unwrap();
        let to: usize = split.next().unwrap().parse().unwrap();
        invocations.push([from, to]);
    }

    // Solve the problem using Solution
    let solution = Solution;
    let result = solution.remaining_methods(n, k, invocations);

    // Print the result
    for x in result {
        print!("{} ", x);
    }
    println!();
}