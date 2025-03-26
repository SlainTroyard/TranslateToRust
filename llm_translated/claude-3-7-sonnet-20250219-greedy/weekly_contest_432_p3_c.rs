use std::collections::VecDeque;
use std::io::{self, BufRead};

// Main solution function
fn min_max_weight(n: i32, edges: &Vec<Vec<i32>>, threshold: i32) -> i32 {
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

// Find the maximum weight among all edges
fn find_max_weight(edges: &Vec<Vec<i32>>) -> i32 {
    edges.iter().map(|edge| edge[2]).max().unwrap_or(0)
}

// Check if all nodes can be reached from node 0 with edges of weight <= limit
fn check(n: i32, edges: &Vec<Vec<i32>>, limit: i32) -> bool {
    let n = n as usize;
    
    // Create adjacency list - using only edges with weight <= limit
    let mut adj_list: Vec<Vec<i32>> = vec![Vec::new(); n];
    
    for edge in edges {
        if edge[2] <= limit {
            // Reverse edge as in original code
            adj_list[edge[1] as usize].push(edge[0]);
        }
    }
    
    // BFS to check if all nodes can be reached from node 0
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    
    visited[0] = true;
    queue.push_back(0);
    
    while !queue.is_empty() {
        let current_vertex = queue.pop_front().unwrap();
        
        for &adj_vertex in &adj_list[current_vertex as usize] {
            let adj_vertex = adj_vertex as usize;
            if !visited[adj_vertex] {
                visited[adj_vertex] = true;
                queue.push_back(adj_vertex as i32);
            }
        }
    }
    
    // Check if all nodes were visited
    visited.iter().all(|&v| v)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and threshold
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().expect("Failed to parse n");
    let threshold: i32 = iter.next().unwrap().parse().expect("Failed to parse threshold");
    
    // Read edges
    let mut edges = Vec::new();
    
    for line in lines {
        let line = line?;
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse edge value"))
            .collect();
        
        if values.len() == 3 {
            edges.push(values);
        }
    }
    
    // Call the solution function
    let result = min_max_weight(n, &edges, threshold);
    println!("{}", result);
    
    Ok(())
}