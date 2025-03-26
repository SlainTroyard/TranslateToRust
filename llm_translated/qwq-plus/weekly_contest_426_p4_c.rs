use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read edges for tree1
    let n1: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut edges1 = Vec::with_capacity(n1);
    for _ in 0..n1 {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let u: i32 = parts.next().unwrap().parse().unwrap();
        let v: i32 = parts.next().unwrap().parse().unwrap();
        edges1.push(vec![u, v]);
    }

    // Read edges for tree2
    let n2: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut edges2 = Vec::with_capacity(n2);
    for _ in 0..n2 {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let u: i32 = parts.next().unwrap().parse().unwrap();
        let v: i32 = parts.next().unwrap().parse().unwrap();
        edges2.push(vec![u, v]);
    }

    // Build adjacency lists
    let adj1 = build_adjacency_list(&edges1);
    let adj2 = build_adjacency_list(&edges2);

    // BFS to get color counts and node colors
    let (color_count1, node_color1) = bfs_color_count(&adj1);
    let (color_count2, _node_color2) = bfs_color_count(&adj2);

    // Calculate maximum color count in tree2
    let max_color2 = *color_count2.iter().max().unwrap();

    // Compute result for each node in tree1
    let mut result = Vec::with_capacity(adj1.len());
    for i in 0..adj1.len() {
        let color = node_color1[i];
        result.push(color_count1[color] + max_color2);
    }

    // Output the result
    for num in &result {
        print!("{} ", num);
    }
    println!();
}

// Build adjacency list from edges
fn build_adjacency_list(edges: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let n_nodes = edges.len() + 1; // Number of nodes is edges.len() + 1
    let mut adj = vec![Vec::new(); n_nodes];
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        adj[u].push(v);
        adj[v].push(u);
    }
    adj
}

// BFS to compute color counts and node colors
fn bfs_color_count(adj: &Vec<Vec<usize>>) -> (Vec<usize>, Vec<usize>) {
    let n = adj.len();
    let mut color_count = vec![0; 2];
    let mut node_color = vec![0; n];
    let mut visited = vec![false; n];
    use std::collections::VecDeque;

    let mut queue = VecDeque::new();
    queue.push_back(0);
    visited[0] = true;
    node_color[0] = 0;

    while let Some(curr) = queue.pop_front() {
        let color = node_color[curr];
        color_count[color] += 1;
        for &neighbor in &adj[curr] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                node_color[neighbor] = 1 - color;
                queue.push_back(neighbor);
            }
        }
    }
    (color_count, node_color)
}