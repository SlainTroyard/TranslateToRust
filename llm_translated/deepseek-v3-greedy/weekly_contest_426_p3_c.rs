use std::collections::HashMap;
use std::io::{self, BufRead};

// Function to calculate the number of ways to walk in k steps
fn linepots(k: i32, pots: &Vec<Vec<i32>>, node: i32, visited: i32) -> i32 {
    if k == -1 {
        return 0; // Base case: no steps left
    }
    if k == 0 {
        return 1; // Base case: exactly 0 steps left, count 1 path
    }
    let mut answer = 1; // Count the current node itself
    for &adj_node in &pots[node as usize] {
        if adj_node != visited {
            answer += linepots(k - 1, pots, adj_node, node); // Recursively count paths from adjacent nodes
        }
    }
    answer // Return total paths from this node
}

// Function to calculate the maximum target nodes in two trees
fn max_target_nodes(
    edges1: &Vec<Vec<i32>>,
    edges2: &Vec<Vec<i32>>,
    k: i32,
) -> Vec<i32> {
    let len1 = edges1.iter().map(|edge| edge[1]).max().unwrap_or(0);
    let len2 = edges2.iter().map(|edge| edge[1]).max().unwrap_or(0);

    // Build adjacency list for tree 1
    let mut pots1 = vec![vec![]; (len1 + 1) as usize];
    for edge in edges1 {
        pots1[edge[0] as usize].push(edge[1]);
        pots1[edge[1] as usize].push(edge[0]);
    }

    // Build adjacency list for tree 2
    let mut pots2 = vec![vec![]; (len2 + 1) as usize];
    for edge in edges2 {
        pots2[edge[0] as usize].push(edge[1]);
        pots2[edge[1] as usize].push(edge[0]);
    }

    // Find the max number of ways to walk in k-1 steps from any node in tree 2
    let max = (0..=len2)
        .map(|i| linepots(k - 1, &pots2, i, -1))
        .max()
        .unwrap_or(0);

    // For each node in tree 1, calculate the total number of paths by adding paths from tree 2
    let mut answer = vec![0; (len1 + 1) as usize];
    for i in 0..=len1 {
        answer[i as usize] = linepots(k, &pots1, i, -1) + max;
    }

    answer
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input for edges1
    let n1: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut edges1 = Vec::with_capacity(n1);
    for _ in 0..n1 {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let u = iter.next().unwrap().parse().unwrap();
        let v = iter.next().unwrap().parse().unwrap();
        edges1.push(vec![u, v]);
    }

    // Input for edges2
    let n2: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut edges2 = Vec::with_capacity(n2);
    for _ in 0..n2 {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let u = iter.next().unwrap().parse().unwrap();
        let v = iter.next().unwrap().parse().unwrap();
        edges2.push(vec![u, v]);
    }

    // Input for k
    let k: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    // Call the max_target_nodes function
    let result = max_target_nodes(&edges1, &edges2, k);

    // Output the result
    for val in result {
        print!("{} ", val);
    }
    println!();
}