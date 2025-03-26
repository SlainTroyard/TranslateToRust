// Problem: Weekly Contest 425 Problem 4
use std::io;
use std::vec;
use std::cmp::min;

struct Solution {}

impl Solution {
    fn dfs(adj: &Vec<Vec<(i32, i32)>>, u: i32, parent: i32, k: i32, dp: &mut Vec<Vec<i64>>) {
        let mut differences: Vec<i64> = vec![];
        let mut sum_weights: i64 = 0;

        // Explore all neighbors of the current node
        for &(v, w) in &adj[u as usize] {
            if v == parent {
                continue; // Skip the parent node to avoid backtracking
            }

            Solution::dfs(adj, v, u, k, dp); // Recursively process the child node

            // Difference in weight contribution between keeping and removing the edge
            differences.push(w as i64 + dp[v as usize][1] - dp[v as usize][0]);

            // Accumulate the base case sum (dp[v][0]) for the subtree rooted at v
            sum_weights += dp[v as usize][0];
        }

        // Sort differences in descending order to prioritize edges with higher contributions
        differences.sort_by(|a, b| b.cmp(a));

        // Case 1: Select at most `k` edges for the current node
        dp[u as usize][0] = sum_weights;
        for i in 0..min(k, differences.len() as i32) {
            if differences[i as usize] > 0 {
                dp[u as usize][0] += differences[i as usize];
            }
        }

        // Case 2: Select at most `k-1` edges for the current node (for parent inclusion)
        dp[u as usize][1] = sum_weights;
        for i in 0..min(k - 1, differences.len() as i32) {
            if differences[i as usize] > 0 {
                dp[u as usize][1] += differences[i as usize];
            }
        }
    }

    pub fn maximize_sum_of_weights(edges: Vec<Vec<i32>>, k: i32) -> i64 {
        let n = edges.len() + 1; // Total nodes in the tree
        let mut adj: Vec<Vec<(i32, i32)>> = vec![vec![]; n];

        // Build adjacency list
        for e in &edges {
            let u = e[0];
            let v = e[1];
            let w = e[2];
            adj[u as usize].push((v, w));
            adj[v as usize].push((u, w));
        }

        // dp[u][0]: Maximum sum when at most `k` edges are selected for node `u`
        // dp[u][1]: Maximum sum when at most `k-1` edges are selected for node `u` (to consider parent inclusion)
        let mut dp: Vec<Vec<i64>> = vec![vec![-1; 2]; n];

        // Start DFS from the root node (0)
        Solution::dfs(&adj, 0, -1, k, &mut dp);

        // The result is the maximum sum when starting from the root node with at most `k` edges
        dp[0][0]
    }
}

fn main() {
    let mut n_k = String::new();
    io::stdin().read_line(&mut n_k).unwrap();
    let mut iter = n_k.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut edges: Vec<Vec<i32>> = vec![vec![]; n];
    for i in 0..n {
        let mut edge_str = String::new();
        io::stdin().read_line(&mut edge_str).unwrap();
        let mut edge_iter = edge_str.trim().split_whitespace();
        let u: i32 = edge_iter.next().unwrap().parse().unwrap();
        let v: i32 = edge_iter.next().unwrap().parse().unwrap();
        let w: i32 = edge_iter.next().unwrap().parse().unwrap();
        edges[i] = vec![u, v, w];
    }

    // Create an instance of the Solution struct
    let solution = Solution {};

    // Call the maximize_sum_of_weights function and print the result
    let result = solution.maximize_sum_of_weights(edges, k);
    println!("{}", result);
}