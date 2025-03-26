use std::error::Error;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    /// Performs a DFS to compute dp[u][0] and dp[u][1] for node u.
    /// dp[u][0]: Maximum sum when at most k edges are selected for node u.
    /// dp[u][1]: Maximum sum when at most k-1 edges are selected for node u (to allow parent edge selection).
    fn dfs(
        &self,
        adj: &Vec<Vec<(usize, i64)>>,
        u: usize,
        parent: isize,
        k: i32,
        dp: &mut Vec<Vec<i64>>,
    ) {
        let mut differences = Vec::new();
        let mut sum_weights = 0_i64;

        // Explore all neighbors of the current node
        for &(v, w) in &adj[u] {
            if v as isize == parent {
                continue; // Skip the parent node to avoid backtracking
            }

            // Recursively process the child node
            self.dfs(adj, v, u as isize, k, dp);

            // Difference in weight contribution between keeping and removing the edge
            let diff = w + dp[v][1] - dp[v][0];
            differences.push(diff);

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

    /// Builds the adjacency list from the given edges and computes the maximum sum of weights.
    pub fn maximize_sum_of_weights(&self, edges: &Vec<Vec<i32>>, k: i32) -> i64 {
        let n = edges.len() + 1; // Total nodes in the tree
        let mut adj = vec![Vec::new(); n];

        // Build adjacency list
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let w = e[2] as i64;
            adj[u].push((v, w));
            adj[v].push((u, w));
        }

        // dp[u][0]: max sum with at most k edges for node u
        // dp[u][1]: max sum with at most k-1 edges for node u (parent may use an edge)
        let mut dp = vec![vec![-1_i64; 2]; n];

        // Start DFS from the root node (0)
        self.dfs(&adj, 0, -1, k, &mut dp);

        // The result is dp[0][0] since we're looking for maximum sum from the root with up to k edges
        dp[0][0]
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of edges (n) and allowed number of edges to select (k)
    let first_line = lines.next().ok_or("Missing input line for n, k")??;
    let mut first_parts = first_line.split_whitespace();
    let n: usize = first_parts
        .next()
        .ok_or("Missing value for n")?
        .parse()
        .map_err(|_| "Invalid integer for n")?;
    let k: i32 = first_parts
        .next()
        .ok_or("Missing value for k")?
        .parse()
        .map_err(|_| "Invalid integer for k")?;

    // Read the edges, each containing three integers (u, v, w)
    let mut edges = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().ok_or("Missing input line for edge")??;
        let mut parts = line.split_whitespace();
        let u: i32 = parts
            .next()
            .ok_or("Missing value for u")?
            .parse()
            .map_err(|_| "Invalid integer for u")?;
        let v: i32 = parts
            .next()
            .ok_or("Missing value for v")?
            .parse()
            .map_err(|_| "Invalid integer for v")?;
        let w: i32 = parts
            .next()
            .ok_or("Missing value for w")?
            .parse()
            .map_err(|_| "Invalid integer for w")?;

        edges.push(vec![u, v, w]);
    }

    // Create an instance of the Solution struct
    let solution = Solution;

    // Compute and print the result
    let result = solution.maximize_sum_of_weights(&edges, k);
    println!("{}", result);

    Ok(())
}