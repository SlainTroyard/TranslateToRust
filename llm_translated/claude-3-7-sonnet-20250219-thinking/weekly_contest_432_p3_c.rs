use std::collections::VecDeque;
use std::io::{self, BufRead};

/// Check if all nodes can be reached from node 0 with edges of weight <= limit
fn check(n: usize, edges: &[Vec<i32>], limit: i32) -> bool {
    // Create adjacency list - using only edges with weight <= limit
    let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); n];
    
    for edge in edges {
        if edge[2] <= limit {
            adj_list[edge[1] as usize].push(edge[0] as usize); // Reverse edge as in original code
        }
    }
    
    // BFS to check if all nodes can be reached from node 0
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    
    visited[0] = true;
    queue.push_back(0);
    
    while let Some(current_vertex) = queue.pop_front() {
        for &adj_vertex in &adj_list[current_vertex] {
            if !visited[adj_vertex] {
                visited[adj_vertex] = true;
                queue.push_back(adj_vertex);
            }
        }
    }
    
    // Check if all nodes were visited
    visited.iter().all(|&v| v)
}

/// Find the maximum weight among all edges
fn find_max_weight(edges: &[Vec<i32>]) -> i32 {
    edges.iter().map(|edge| edge[2]).max().unwrap_or(0)
}

/// Main solution function
fn min_max_weight(n: usize, edges: &[Vec<i32>], _threshold: i32) -> i32 {
    let max_weight = find_max_weight(edges);
    
    // Check if it's possible to reach all nodes even with max weight
    if !check(n, edges, max_weight) {
        return -1;
    }
    
    // Binary search for the minimal maximum weight
    let mut left = 0;
    let mut right = max_weight;
    
    while left < right {
        let mid = left + (right - left) / 2;
        if check(n, edges, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    
    left
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and threshold
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let threshold: i32 = iter.next().unwrap().parse().expect("Failed to parse threshold");
    
    // Initialize edges vector
    let mut edges = Vec::new();
    
    // Read all edges until EOF
    for line in lines {
        let line = line.unwrap();
        let values: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().expect("Failed to parse edge value"))
            .collect();
        
        // Make sure we have a complete edge
        if values.len() >= 3 {
            edges.push(vec![values[0], values[1], values[2]]);
        }
    }
    
    // Call the solution function
    let result = min_max_weight(n, &edges, threshold);
    println!("{}", result);
}