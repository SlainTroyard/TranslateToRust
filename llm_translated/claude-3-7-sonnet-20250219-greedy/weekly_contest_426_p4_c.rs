use std::collections::VecDeque;
use std::io::{self, BufRead};

/// Function to build adjacency list for a tree
fn build_adjacency_list(edges: &Vec<Vec<i32>>, n: usize) -> Vec<Vec<i32>> {
    let mut adj_list = vec![Vec::new(); n];
    
    // Fill adjacency list
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        adj_list[u].push(v as i32);
        adj_list[v].push(u as i32);
    }
    
    adj_list
}

/// BFS to count nodes of each color
fn bfs_count(adj_list: &Vec<Vec<i32>>, n: usize) -> (Vec<i32>, [i32; 2]) {
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    let mut node_color = vec![0; n];
    let mut color_count = [0, 0];
    
    // Start BFS from node 0
    queue.push_back(0);
    node_color[0] = 0;  // Color of root is 0
    visited[0] = true;
    
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        let color = node_color[curr];
        color_count[color as usize] += 1;
        
        for &neighbor in &adj_list[curr] {
            let neighbor = neighbor as usize;
            if !visited[neighbor] {
                visited[neighbor] = true;
                node_color[neighbor] = 1 - color;  // Alternate color
                queue.push_back(neighbor);
            }
        }
    }
    
    (node_color, color_count)
}

/// Main solution function
fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
    let n1 = edges1.len() + 1;  // Number of nodes in tree 1
    let n2 = edges2.len() + 1;  // Number of nodes in tree 2
    
    // Build adjacency lists
    let adj_list1 = build_adjacency_list(&edges1, n1);
    let adj_list2 = build_adjacency_list(&edges2, n2);
    
    // Perform BFS to calculate color distribution
    let (node_color1, color_count1) = bfs_count(&adj_list1, n1);
    let (_, color_count2) = bfs_count(&adj_list2, n2);
    
    // Calculate results for tree 1
    let max_color_in_tree2 = color_count2[0].max(color_count2[1]);
    
    let mut result = vec![0; n1];
    for i in 0..n1 {
        result[i] = color_count1[node_color1[i] as usize] + max_color_in_tree2;
    }
    
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Input number of edges for tree 1
    let n1: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut edges1 = Vec::with_capacity(n1);
    
    for _ in 0..n1 {
        let line = lines.next().unwrap().unwrap();
        let edge: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges1.push(edge);
    }
    
    // Input number of edges for tree 2
    let n2: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut edges2 = Vec::with_capacity(n2);
    
    for _ in 0..n2 {
        let line = lines.next().unwrap().unwrap();
        let edge: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges2.push(edge);
    }
    
    // Call the solution function
    let result = max_target_nodes(edges1, edges2);
    
    // Output the result
    let output: String = result.iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", output);
}