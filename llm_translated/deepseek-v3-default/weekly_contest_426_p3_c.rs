use std::io;
use std::collections::HashMap;

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
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n1: i32 = input.trim().parse().unwrap();

    let mut edges1 = Vec::new();
    for _ in 0..n1 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let edge: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges1.push(edge);
    }

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let n2: i32 = input.trim().parse().unwrap();

    let mut edges2 = Vec::new();
    for _ in 0..n2 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let edge: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges2.push(edge);
    }

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let k: i32 = input.trim().parse().unwrap();

    let result = max_target_nodes(&edges1, &edges2, k);

    for val in result {
        print!("{} ", val);
    }
    println!();
}