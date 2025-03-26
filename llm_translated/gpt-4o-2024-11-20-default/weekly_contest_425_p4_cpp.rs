use std::cmp;
use std::collections::VecDeque;
use std::io::{self, Read};

struct Solution;

impl Solution {
    fn dfs(
        adj: &Vec<Vec<(usize, i64)>>,
        u: usize,
        parent: isize,
        k: usize,
        dp: &mut Vec<Vec<i64>>,
    ) {
        let mut differences = Vec::new();
        let mut sum_weights = 0;

        // Explore all neighbors of the current node
        for &(v, w) in &adj[u] {
            if v as isize == parent {
                continue; // Skip the parent node to avoid backtracking
            }

            // Recursively process the child node
            Self::dfs(adj, v, u as isize, k, dp);

            // Difference in weight contribution between keeping and removing the edge
            differences.push(w + dp[v][1] - dp[v][0]);

            // Accumulate the base case sum (dp[v][0]) for the subtree rooted at v
            sum_weights += dp[v][0];
        }

        // Sort differences in descending order to prioritize edges with higher contributions
        differences.sort_by(|a, b| b.cmp(a));

        // Case 1: Select at most `k` edges for the current node
        dp[u][0] = sum_weights;
        for i in 0..cmp::min(k, differences.len()) {
            if differences[i] > 0 {
                dp[u][0] += differences[i];
            }
        }

        // Case 2: Select at most `k-1` edges for the current node (for parent inclusion)
        dp[u][1] = sum_weights;
        for i in 0..cmp::min(k.saturating_sub(1), differences.len()) {
            if differences[i] > 0 {
                dp[u][1] += differences[i];
            }
        }
    }

    pub fn maximize_sum_of_weights(edges: Vec<Vec<i64>>, k: usize) -> i64 {
        let n = edges.len() + 1; // Total nodes in the tree
        let mut adj = vec![Vec::new(); n];

        // Build adjacency list
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            adj[u].push((v, w));
            adj[v].push((u, w));
        }

        // dp[u][0]: Maximum sum when at most `k` edges are selected for node `u`
        // dp[u][1]: Maximum sum when at most `k-1` edges are selected for node `u` (to consider parent inclusion)
        let mut dp = vec![vec![0; 2]; n];

        // Start DFS from the root node (0)
        Self::dfs(&adj, 0, -1, k, &mut dp);

        // The result is the maximum sum when starting from the root node with at most `k` edges
        dp[0][0]
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // Read the number of edges and the allowed number of edges to select
    let first_line = lines.next().unwrap();
    let (n, k) = {
        let parts: Vec<_> = first_line.split_whitespace().collect();
        (parts[0].parse::<usize>().unwrap(), parts[1].parse::<usize>().unwrap())
    };

    let mut edges = Vec::new();

    // Read the edges, each as a triplet (u, v, w)
    for _ in 0..n {
        if let Some(line) = lines.next() {
            let parts: Vec<_> = line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
            edges.push(parts);
        }
    }

    // Call the maximize_sum_of_weights function and print the result
    let result = Solution::maximize_sum_of_weights(edges, k);
    println!("{}", result);
}