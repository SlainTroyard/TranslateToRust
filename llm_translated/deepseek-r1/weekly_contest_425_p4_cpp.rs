use std::cmp::min;

struct Solution;

impl Solution {
    pub fn maximize_sum_of_weights(edges: Vec<Vec<i32>>, k: usize) -> i64 {
        let n = edges.len() + 1;
        let mut adj = vec![vec![]; n];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let w = e[2] as i64;
            adj[u].push((v, w));
            adj[v].push((u, w));
        }
        let mut dp = vec![vec![0; 2]; n];
        Self::dfs(&adj, 0, usize::MAX, k, &mut dp);
        dp[0][0]
    }

    fn dfs(adj: &[Vec<(usize, i64)>], u: usize, parent: usize, k: usize, dp: &mut [Vec<i64>]) {
        let mut differences = Vec::new();
        let mut sum_weights = 0;

        // Process all children and collect differences
        for &(v, w) in &adj[u] {
            if v == parent {
                continue;
            }
            Self::dfs(adj, v, u, k, dp);
            let diff = w + dp[v][1] - dp[v][0];
            differences.push(diff);
            sum_weights += dp[v][0];
        }

        // Sort differences in descending order
        differences.sort_by(|a, b| b.cmp(a));

        // Calculate dp[u][0] with up to k positive differences
        let max_k = min(k, differences.len());
        let mut sum0 = sum_weights;
        for i in 0..max_k {
            if differences[i] > 0 {
                sum0 += differences[i];
            } else {
                break;
            }
        }
        dp[u][0] = sum0;

        // Calculate dp[u][1] with up to k-1 positive differences
        let max_k_minus_1 = min(k.saturating_sub(1), differences.len());
        let mut sum1 = sum_weights;
        for i in 0..max_k_minus_1 {
            if differences[i] > 0 {
                sum1 += differences[i];
            } else {
                break;
            }
        }
        dp[u][1] = sum1;
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    // Read n (number of edges) and k
    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    // Read edges data
    let mut edges = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let u: i32 = parts.next().unwrap().parse().unwrap();
        let v: i32 = parts.next().unwrap().parse().unwrap();
        let w: i32 = parts.next().unwrap().parse().unwrap();
        edges.push(vec![u, v, w]);
    }

    // Compute and print the result
    let result = Solution::maximize_sum_of_weights(edges, k);
    println!("{}", result);
}