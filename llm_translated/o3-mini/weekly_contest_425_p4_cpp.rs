use std::io::{self, BufRead, Write};
use std::cmp::min;

// Define the DFS function that computes dp values for each node in the tree
fn dfs(
    adj: &Vec<Vec<(usize, i64)>>,
    u: usize,
    parent: Option<usize>,
    k: usize,
    dp: &mut Vec<[i64; 2]>,
) {
    let mut differences: Vec<i64> = Vec::new();
    let mut sum_weights: i64 = 0;

    // Iterate over each neighbor (child) of current node
    for &(v, w) in adj[u].iter() {
        // Avoid processing the parent to prevent going backwards in tree
        if Some(v) == parent {
            continue;
        }
        // Recursively process child node v
        dfs(adj, v, Some(u), k, dp);
        
        // Calculate difference if we include the edge from u to v 
        // (w + contribution difference between allowing parent's edge removal)
        differences.push(w + dp[v][1] - dp[v][0]);
        // Add the base contribution (without selecting an extra edge) from subtree v
        sum_weights += dp[v][0];
    }

    // Sort differences in descending order so that the highest gains are used first
    differences.sort_by(|a, b| b.cmp(a));

    // Case 1: When parent's connection doesn't matter, we can select at most k edges.
    dp[u][0] = sum_weights;
    for i in 0..min(k, differences.len()) {
        if differences[i] > 0 {
            dp[u][0] += differences[i];
        }
    }

    // Case 2: When parent's connection requires using one less opportunity, at most k-1 edges.
    dp[u][1] = sum_weights;
    let available = if k > 0 { k - 1 } else { 0 };
    for i in 0..min(available, differences.len()) {
        if differences[i] > 0 {
            dp[u][1] += differences[i];
        }
    }
}

// Function that takes the edge list and parameter k and returns the maximum sum of weights.
fn maximize_sum_of_weights(edges: Vec<Vec<i32>>, k: usize) -> i64 {
    // The number of nodes is the number of edges + 1 since the input is a tree.
    let n = edges.len() + 1;
    
    // Build the adjacency list for the tree.
    let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
    for e in edges.into_iter() {
        let u = e[0] as usize;
        let v = e[1] as usize;
        let w = e[2] as i64;
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    // Initialize dp table for each node.
    // dp[u][0]: Maximum sum when up to k edges are selected in subtree of u.
    // dp[u][1]: Maximum sum when up to k-1 edges are selected, used when including parent's contribution.
    let mut dp: Vec<[i64; 2]> = vec![[0; 2]; n];

    // Perform DFS starting from the root node 0. Use None for parent of root.
    dfs(&adj, 0, None, k, &mut dp);

    // Return the answer which is the maximum sum for root node with at most k selected edges.
    dp[0][0]
}

fn main() -> io::Result<()> {
    // Use BufReader for efficient input handling.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    
    // Split all tokens by whitespace.
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut idx = 0;
    
    // First two tokens: n (number of edges) and k (allowed number of edges to select).
    if idx >= tokens.len() {
        return Ok(());
    }
    let n: usize = tokens[idx].parse().expect("Failed to parse n");
    idx += 1;
    
    if idx >= tokens.len() {
        return Ok(());
    }
    let k: usize = tokens[idx].parse().expect("Failed to parse k");
    idx += 1;
    
    // Read exactly n edges. Each edge consists of three integers: u, v, w.
    let mut edges = Vec::with_capacity(n);
    for _ in 0..n {
        if idx + 2 >= tokens.len() {
            break;
        }
        let u: i32 = tokens[idx].parse().expect("Failed to parse u");
        idx += 1;
        let v: i32 = tokens[idx].parse().expect("Failed to parse v");
        idx += 1;
        let w: i32 = tokens[idx].parse().expect("Failed to parse w");
        idx += 1;
        edges.push(vec![u, v, w]);
    }
    
    // Compute the result and write it to stdout.
    let result = maximize_sum_of_weights(edges, k);
    println!("{}", result);
    
    Ok(())
}