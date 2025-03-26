use std::collections::VecDeque;
use std::io;

/// Checks if all nodes are reachable from node 0 using edges with weight <= limit.
/// The adjacency list is built by adding edges from destination to source as per original C logic.
fn can_reach_all(n: usize, edges: &[[i32; 3]], limit: i32) -> bool {
    // Build adjacency list: edges from destination to source
    let mut adj = vec![Vec::new(); n];
    for &edge in edges.iter() {
        if edge[2] <= limit {
            let src = edge[0] as usize;
            let dest = edge[1] as usize;
            adj[dest].push(src);
        }
    }

    // BFS to check reachability
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    visited[0] = true;
    queue.push_back(0);

    while let Some(current) = queue.pop_front() {
        for &neighbor in &adj[current] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    visited.iter().all(|&v| v)
}

/// Finds the maximum edge weight in the input edges.
fn find_max_weight(edges: &[[i32; 3]]) -> i32 {
    edges.iter().map(|e| e[2]).max().unwrap_or(0)
}

/// Finds the minimal maximum edge weight needed to reach all nodes from 0 via binary search.
fn min_max_weight(n: usize, edges: &[[i32; 3]], threshold: i32) -> i32 {
    let max_weight = find_max_weight(edges);
    if !can_reach_all(n, edges, max_weight) {
        return -1;
    }

    let mut left = 0;
    let mut right = max_weight;

    while left < right {
        let mid = left + (right - left) / 2;
        if can_reach_all(n, edges, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut lines = input.lines();

    // Read first line: n and threshold
    let first_line = lines.next().expect("No first line");
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().expect("Invalid n");
    let threshold: i32 = parts.next().unwrap().parse().expect("Invalid threshold");

    // Read all edges until EOF
    let mut edges = Vec::new();
    for line in lines {
        let mut parts = line.split_whitespace();
        let src: i32 = parts.next().unwrap().parse().expect("Invalid src");
        let dest: i32 = parts.next().unwrap().parse().expect("Invalid dest");
        let weight: i32 = parts.next().unwrap().parse().expect("Invalid weight");
        edges.push([src, dest, weight]);
    }

    // Compute and print result
    let result = min_max_weight(n, &edges, threshold);
    println!("{}", result);
}