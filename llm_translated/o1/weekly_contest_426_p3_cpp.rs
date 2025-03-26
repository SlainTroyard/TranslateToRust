// Weekly Contest 426 Problem 3
// Translated from the provided C++ code to Rust
// Requirements met:
// 1) Entire file is translated as a complete Rust program
// 2) Algorithm logic is preserved exactly
// 3) Uses idiomatic Rust with error handling
// 4) Maintains the exact same stdin/stdout format
// 5) Input parsing and output are faithfully reproduced

use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // Build adjacency list from edges
    fn build_graph(&self, edges: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        // The original code used edges.size()+1 as the graph size
        let n = edges.len() + 1;
        let mut g = vec![Vec::new(); n];
        for e in edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(b);
            g[b].push(a);
        }
        g
    }

    // Depth-first search to count reachable nodes within distance k
    fn dfs(&self, g: &[Vec<usize>], root: usize, par: i32, k: i32) -> i32 {
        // If no more distance is allowed, return 0
        if k < 0 {
            return 0;
        }
        // Count this node, then recurse on children
        let mut count = 1;
        for &node in &g[root] {
            if node as i32 != par {
                count += self.dfs(g, node, root as i32, k - 1);
            }
        }
        count
    }

    // Main solution logic
    fn max_target_nodes(&self, edges1: &Vec<Vec<i32>>, edges2: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let g1 = self.build_graph(edges1);
        let g2 = self.build_graph(edges2);

        let n = edges1.len() + 1;
        let m = edges2.len() + 1;

        // Compute the maximum subtree size in g2 with distance k-1
        let mut count = 0;
        for i in 0..m {
            let c = self.dfs(&g2, i, -1, k - 1);
            if c > count {
                count = c;
            }
        }

        // For each node in g1, add the best count from g2
        let mut ans = Vec::new();
        for i in 0..n {
            ans.push(count + self.dfs(&g1, i, -1, k));
        }
        ans
    }
}

fn main() {
    // Read all tokens from stdin (matching the behavior of C++ cin)
    let stdin = io::stdin();
    let mut tokens = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        for token in line.split_whitespace() {
            tokens.push(token.to_string());
        }
    }

    // Parse n1
    let mut idx = 0;
    let n1: usize = tokens[idx].parse().unwrap();
    idx += 1;

    // Read edges1
    let mut edges1 = vec![vec![0; 2]; n1];
    for i in 0..n1 {
        edges1[i][0] = tokens[idx].parse().unwrap();
        idx += 1;
        edges1[i][1] = tokens[idx].parse().unwrap();
        idx += 1;
    }

    // Parse n2
    let n2: usize = tokens[idx].parse().unwrap();
    idx += 1;

    // Read edges2
    let mut edges2 = vec![vec![0; 2]; n2];
    for i in 0..n2 {
        edges2[i][0] = tokens[idx].parse().unwrap();
        idx += 1;
        edges2[i][1] = tokens[idx].parse().unwrap();
        idx += 1;
    }

    // Parse k
    let k: i32 = tokens[idx].parse().unwrap();
    idx += 1;

    // Call the solution
    let solution = Solution;
    let result = solution.max_target_nodes(&edges1, &edges2, k);

    // Print the result (same format as the C++ code)
    for val in result {
        print!("{} ", val);
    }
    println!();
}