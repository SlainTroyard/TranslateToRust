use std::io::{self, BufRead, Write};

// Function to build adjacency list for a tree
fn build_adjacency_list(edges: &Vec<Vec<usize>>, n: usize) -> Vec<Vec<usize>> {
    let mut adj_list = vec![Vec::new(); n];
    for edge in edges.iter() {
        let (u, v) = (edge[0], edge[1]);
        adj_list[u].push(v);
        adj_list[v].push(u);
    }
    adj_list
}

// BFS to count nodes of each color
fn bfs_count(adj_list: &Vec<Vec<usize>>, n: usize) -> (usize, usize) {
    let mut visited = vec![false; n];
    let mut queue = Vec::new();
    let mut color_count = [0, 0];

    queue.push(0);
    visited[0] = true;
    color_count[0] += 1;

    while let Some(curr) = queue.pop() {
        let color = if visited[curr] { 0 } else { 1 };
        color_count[color] += 1;

        for &neighbor in &adj_list[curr] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push(neighbor);
            }
        }
    }

    color_count
}

// Main solution function
fn max_target_nodes(edges1: &Vec<Vec<usize>>, edges2: &Vec<Vec<usize>>) -> Vec<usize> {
    let n1 = edges1.len() + 1;
    let n2 = edges2.len() + 1;

    // Build adjacency lists
    let adj_list1 = build_adjacency_list(edges1, n1);
    let adj_list2 = build_adjacency_list(edges2, n2);

    // Perform BFS to calculate color distribution
    let (color_count1, color_count2) = (bfs_count(&adj_list1, n1), bfs_count(&adj_list2, n2));

    // Calculate results for tree 1
    let max_color_in_tree2 = color_count2[0].max(color_count2[1]);
    let mut result = Vec::with_capacity(n1);

    for i in 0..n1 {
        result.push(color_count1[i % 2] + max_color_in_tree2);
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    // Input number of edges for tree 1
    stdin.lock().read_line(&mut input).unwrap();
    let n1: usize = input.trim().parse().unwrap();
    input.clear();

    let mut edges1 = Vec::with_capacity(n1);
    for _ in 0..n1 {
        stdin.lock().read_line(&mut input).unwrap();
        let edge: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges1.push(edge);
        input.clear();
    }

    // Input number of edges for tree 2
    stdin.lock().read_line(&mut input).unwrap();
    let n2: usize = input.trim().parse().unwrap();
    input.clear();

    let mut edges2 = Vec::with_capacity(n2);
    for _ in 0..n2 {
        stdin.lock().read_line(&mut input).unwrap();
        let edge: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges2.push(edge);
        input.clear();
    }

    // Call the solution function
    let result = max_target_nodes(&edges1, &edges2);

    // Output the result
    for (i, &val) in result.iter().enumerate() {
        write!(stdout, "{}{}", val, if i < result.len() - 1 { " " } else { "" }).unwrap();
    }
    writeln!(stdout).unwrap();
}