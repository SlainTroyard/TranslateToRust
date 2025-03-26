use std::collections::VecDeque;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn min_max_weight(n: usize, edges: Vec<Vec<i32>>, threshold: i32) -> i32 {
        // Helper function to check if a given weight limit allows all nodes to be reachable
        let check = |lim: i32| -> bool {
            let mut adjacency_list = vec![Vec::new(); n];
            for edge in &edges {
                if edge[2] <= lim {
                    adjacency_list[edge[1] as usize].push(edge[0] as usize);
                }
            }

            let mut visited = vec![false; n];
            let mut queue = VecDeque::new();
            queue.push_back(0);
            visited[0] = true;

            while let Some(current) = queue.pop_front() {
                for &neighbor in &adjacency_list[current] {
                    if !visited[neighbor] {
                        queue.push_back(neighbor);
                        visited[neighbor] = true;
                    }
                }
            }

            visited.iter().all(|&v| v)
        };

        // Find the maximum weight in the edges
        let max_weight = edges.iter().map(|edge| edge[2]).max().unwrap_or(0);

        // If the maximum weight doesn't allow all nodes to be reachable, return -1
        if !check(max_weight) {
            return -1;
        }

        // Binary search to find the minimum weight limit
        let mut left = 0;
        let mut right = max_weight;
        while left < right {
            let mid = (left + right) / 2;
            if check(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of nodes
    let n: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse number of nodes");

    // Read the threshold (not used in the solution, but included for completeness)
    let threshold: i32 = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse threshold");

    // Read the edges
    let mut edges = Vec::new();
    for _ in 0..n {
        let edge: Vec<i32> = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse edge value"))
            .collect();
        edges.push(edge);
    }

    // Solve the problem
    let solution = Solution;
    let result = solution.min_max_weight(n, edges, threshold);

    // Print the result
    println!("{}", result);
}