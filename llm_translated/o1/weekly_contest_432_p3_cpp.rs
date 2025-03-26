// Translated from LeetCode Weekly Contest 432 Problem 3 (C++ to Rust)
// Requirements:
//  1. Translated entire file as a complete program
//  2. Preserved the algorithm logic exactly
//  3. Used idiomatic Rust with proper error handling
//  4. Maintained the EXACT SAME stdin/stdout format
//  5. Added helpful comments

use std::cmp;
use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // Equivalent to the C++: int minMaxWeight(int n, vector<vector<int>>& edges, int threshold)
    // The threshold parameter is not used in the original C++ code, but it is preserved.
    fn min_max_weight(&self, n: usize, edges: &Vec<Vec<i32>>, _threshold: i32) -> i32 {
        // Helper function to check if all nodes can be visited from node 0
        // when we only allow edges with weight <= lim.
        fn check(n: usize, edges: &Vec<Vec<i32>>, lim: i32) -> bool {
            // Build adjacency list: e[node] = list of neighbors
            // As in the original code, direct edges from edge[1] -> edge[0] if edge[2] <= lim
            let mut e = vec![Vec::new(); n];
            for edge in edges.iter() {
                if edge[2] <= lim {
                    let from = edge[1] as usize;
                    let to = edge[0] as usize;
                    if from < n && to < n {
                        e[from].push(to);
                    }
                }
            }

            // BFS from node 0
            let mut visited = vec![false; n];
            let mut queue = VecDeque::new();
            queue.push_back(0);
            visited[0] = true;

            while let Some(sn) = queue.pop_front() {
                for &fn_ in &e[sn] {
                    if !visited[fn_] {
                        visited[fn_] = true;
                        queue.push_back(fn_);
                    }
                }
            }

            // Check if all nodes have been visited
            for i in 0..n {
                if !visited[i] {
                    return false;
                }
            }
            true
        }

        // Find the maximum edge weight
        let mut mx = 0;
        for edge in edges.iter() {
            mx = cmp::max(mx, edge[2]);
        }

        // If we cannot visit all nodes with the max weight allowed, return -1
        if !check(n, edges, mx) {
            return -1;
        }

        // Binary search for the minimum limit that still allows visiting all nodes
        let mut head = 0;
        let mut tail = mx;
        while head < tail {
            let mid = (head + tail) >> 1;
            if check(n, edges, mid) {
                tail = mid;
            } else {
                head = mid + 1;
            }
        }
        head
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Prepare to read from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n (number of times we also read edges, used as number of graph nodes in BFS)
    let n = lines
        .next()
        .ok_or("Missing input for n")??
        .trim()
        .parse::<usize>()?;

    // Read threshold (not used in the logic, but preserved as per original code)
    let threshold = lines
        .next()
        .ok_or("Missing input for threshold")??
        .trim()
        .parse::<i32>()?;

    // Read n lines, each containing 3 integers describing an edge
    let mut edges = Vec::new();
    for _ in 0..n {
        let line = lines
            .next()
            .ok_or("Missing edge input")??
            .trim()
            .to_string();
        let mut parts = line.split_whitespace();
        let mut edge = Vec::new();
        for _ in 0..3 {
            let x = parts
                .next()
                .ok_or("Incomplete edge data")?
                .parse::<i32>()?;
            edge.push(x);
        }
        edges.push(edge);
    }

    // Create a Solution instance and compute the answer
    let solution = Solution;
    let ans = solution.min_max_weight(n, &edges, threshold);

    // Print the result in the same format as the original C++ code
    println!("{}", ans);

    Ok(())
}