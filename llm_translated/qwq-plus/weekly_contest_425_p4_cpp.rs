use std::io;

struct Solution;

impl Solution {
    fn dfs(
        adj: &Vec<Vec<(i32, i32)>>,
        u: i32,
        parent: i32,
        k: i32,
        dp: &mut Vec<[i64; 2]>,
    ) {
        let mut differences = Vec::new();
        let mut sum_weights = 0i64;

        for &(v, w) in &adj[u as usize] {
            if v == parent {
                continue;
            }

            Self::dfs(adj, v, u, k, dp);

            differences.push(
                w as i64 + dp[v as usize][1] - dp[v as usize][0],
            );
            sum_weights += dp[v as usize][0];
        }

        // Sort differences in descending order
        differences.sort_by(|a, b| b.cmp(a));

        let mut dp0 = sum_weights;
        let mut dp1 = sum_weights;

        // Calculate dp0 (up to k edges)
        let take_k = std::cmp::min(k as usize, differences.len());
        for i in 0..take_k {
            if differences[i] > 0 {
                dp0 += differences[i];
            }
        }

        // Calculate dp1 (up to k-1 edges)
        let take_k_minus_1 = std::cmp::min((k - 1) as usize, differences.len());
        for i in 0..take_k_minus_1 {
            if differences[i] > 0 {
                dp1 += differences[i];
            }
        }

        dp[u as usize][0] = dp0;
        dp[u as usize][1] = dp1;
    }

    fn maximize_sum_of_weights(&self, edges: Vec<(i32, i32, i32)>, k: i32) -> i64 {
        let n_nodes = edges.len() + 1; // Number of nodes is edges.len() + 1
        let mut adj = vec![Vec::new(); n_nodes];

        // Build adjacency list
        for &(u, v, w) in &edges {
            adj[u as usize].push((v, w));
            adj[v as usize].push((u, w));
        }

        let mut dp = vec![[-1i64; 2]; n_nodes];

        // Start DFS from root node 0
        Self::dfs(&adj, 0, -1, k, &mut dp);

        // Return the result for root node with at most k edges
        dp[0][0]
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // Read first line: n and k
    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: i32 = parts.next().unwrap().parse().unwrap();

    let mut edges = Vec::with_capacity(n);
    for line in lines {
        let mut parts = line.split_whitespace();
        let u: i32 = parts.next().unwrap().parse().unwrap();
        let v: i32 = parts.next().unwrap().parse().unwrap();
        let w: i32 = parts.next().unwrap().parse().unwrap();
        edges.push((u, v, w));
    }

    let solution = Solution;
    let result = solution.maximize_sum_of_weights(edges, k);
    println!("{}", result);
}