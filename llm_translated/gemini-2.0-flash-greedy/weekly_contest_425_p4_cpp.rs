use std::io;
use std::io::prelude::*;

fn dfs(
    adj: &Vec<Vec<(usize, i32)>>,
    u: usize,
    parent: i32,
    k: i32,
    dp: &mut Vec<Vec<i64>>,
) {
    let mut differences: Vec<i64> = Vec::new();
    let mut sum_weights: i64 = 0;

    // Explore all neighbors of the current node
    for &(v, w) in &adj[u] {
        if v as i32 == parent {
            continue; // Skip the parent node to avoid backtracking
        }

        dfs(adj, v, u as i32, k, dp); // Recursively process the child node

        // Difference in weight contribution between keeping and removing the edge
        differences.push((w as i64) + dp[v][1] - dp[v][0]);

        // Accumulate the base case sum (dp[v][0]) for the subtree rooted at v
        sum_weights += dp[v][0];
    }

    // Sort differences in descending order to prioritize edges with higher contributions
    differences.sort_by(|a, b| b.cmp(a));

    // Case 1: Select at most `k` edges for the current node
    dp[u][0] = sum_weights;
    for i in 0..std::cmp::min(k as usize, differences.len()) {
        if differences[i] > 0 {
            dp[u][0] += differences[i];
        }
    }

    // Case 2: Select at most `k-1` edges for the current node (for parent inclusion)
    dp[u][1] = sum_weights;
    for i in 0..std::cmp::min((k - 1) as usize, differences.len()) {
        if differences[i] > 0 {
            dp[u][1] += differences[i];
        }
    }
}

fn maximize_sum_of_weights(edges: &Vec<Vec<i32>>, k: i32) -> i64 {
    let n = edges.len() + 1; // Total nodes in the tree
    let mut adj: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n];

    // Build adjacency list
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        let w = e[2];
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    // dp[u][0]: Maximum sum when at most `k` edges are selected for node `u`
    // dp[u][1]: Maximum sum when at most `k-1` edges are selected for node `u` (to consider parent inclusion)
    let mut dp: Vec<Vec<i64>> = vec![vec![-1; 2]; n];

    // Start DFS from the root node (0)
    dfs(&adj, 0, -1, k, &mut dp);

    // The result is the maximum sum when starting from the root node with at most `k` edges
    dp[0][0]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut edges: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let nums: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        edges.push(nums);
    }

    let result = maximize_sum_of_weights(&edges, k);
    println!("{}", result);
}