use std::collections::VecDeque;
use std::io::{self, BufRead};

fn check(n: i32, edges: &[[i32; 3]], limit: i32) -> bool {
    let n = n as usize;
    let mut adj = vec![vec![]; n];
    
    // Build adjacency list with reversed edges where weight <= limit
    for &[src, dest, weight] in edges {
        if weight <= limit {
            adj[dest as usize].push(src as usize);
        }
    }
    
    // BFS to check reachability from node 0
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

fn min_max_weight(n: i32, edges: &[[i32; 3]], _threshold: i32) -> i32 {
    let max_weight = edges.iter().map(|edge| edge[2]).max().unwrap_or(0);
    
    // Check if even the maximum weight fails to connect all nodes
    if !check(n, edges, max_weight) {
        return -1;
    }
    
    // Binary search for minimal maximum weight
    let (mut left, mut right) = (0, max_weight);
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
    let mut lines = stdin.lock().lines().filter_map(Result::ok);
    
    // Parse first line: n and threshold
    let first_line = match lines.next() {
        Some(line) => line,
        None => {
            eprintln!("Error reading input");
            return;
        }
    };
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() != 2 {
        eprintln!("Invalid input format");
        return;
    }
    let n: i32 = parts[0].parse().unwrap_or_else(|_| {
        eprintln!("Invalid n");
        std::process::exit(1);
    });
    let threshold: i32 = parts[1].parse().unwrap_or_else(|_| {
        eprintln!("Invalid threshold");
        std::process::exit(1);
    });
    
    // Parse remaining lines as edges
    let mut edges = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 {
            continue;
        }
        let src = parts[0].parse().unwrap_or(0);
        let dest = parts[1].parse().unwrap_or(0);
        let weight = parts[2].parse().unwrap_or(0);
        edges.push([src, dest, weight]);
    }
    
    let result = min_max_weight(n, &edges, threshold);
    println!("{}", result);
}