use std::io::{self, BufRead, Write};

fn dfs(adj: &Vec<Vec<(usize, i64)>>, u: usize, parent: isize, k: usize, dp: &mut Vec<[i64; 2]>) {
    let mut differences = Vec::new();
    let mut sum_weights = 0;

    // Explore all neighbors of the current node
    for (v, w) in &adj[u] {
        if *v as isize == parent {
            continue;  // Skip the parent node to avoid backtracking
        }

        dfs(adj, *v, u as isize, k, dp);  // Recursively process the child node

        // Difference in weight contribution between keeping and removing the edge
        differences.push(*w + dp[*v][1] - dp[*v][0]);

        // Accumulate the base case sum (dp[v][0]) for the subtree rooted at v
        sum_weights += dp[*v][0];
    }

    // Sort differences in descending order to prioritize edges with higher contributions
    differences.sort_by(|a, b| b.cmp(a));

    // Case 1: Select at most `k` edges for the current node
    dp[u][0] = sum_weights;
    for i in 0..std::cmp::min(k, differences.len()) {
        if differences[i] > 0 {
            dp[u][0] += differences[i];
        }
    }

    // Case 2: Select at most `k-1` edges for the current node (for parent inclusion)
    dp[u][1] = sum_weights;
    for i in 0..std::cmp::min(k - 1, differences.len()) {
        if differences[i] > 0 {
            dp[u][1] += differences[i];
        }
    }
}

struct Solution;

impl Solution {
    fn maximize_sum_of_weights(edges: Vec<Vec<i64>>, k: usize) -> i64 {
        let n = edges.len() + 1;  // Total nodes in the tree
        let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];

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
        let mut dp: Vec<[i64; 2]> = vec![[0, 0]; n];

        // Start DFS from the root node (0)
        dfs(&adj, 0, -1, k, &mut dp);

        // The result is the maximum sum when starting from the root node with at most `k` edges
        dp[0][0]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut stdout_lock = stdout.lock();

    // Read the number of edges and the allowed number of edges to select
    let mut input = String::new();
    stdin_lock.read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    let mut edges: Vec<Vec<i64>> = Vec::with_capacity(n);
    
    // Read the edges, each as a triplet (u, v, w)
    for _ in 0..n {
        input.clear();
        stdin_lock.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let u: i64 = iter.next().unwrap().parse().unwrap();
        let v: i64 = iter.next().unwrap().parse().unwrap();
        let w: i64 = iter.next().unwrap().parse().unwrap();
        edges.push(vec![u, v, w]);
    }

    // Create an instance of the Solution class
    let solution = Solution;

    // Call the maximize_sum_of_weights function and print the result
    let result = solution.maximize_sum_of_weights(edges, k);
    writeln!(stdout_lock, "{}", result).unwrap();
}