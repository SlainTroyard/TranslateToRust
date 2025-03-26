use std::io;
use std::io::Read;
use std::vec;

// Function to build adjacency list for a tree
fn build_adjacency_list(
    edges: &Vec<Vec<i32>>,
    n: usize,
) -> (Vec<Vec<i32>>, Vec<usize>) {
    let mut adj_list: Vec<Vec<i32>> = vec![Vec::new(); n];
    let mut adj_len: Vec<usize> = vec![0; n];

    // Count the degree of each node
    for edge in edges {
        adj_len[edge[0] as usize] += 1;
        adj_len[edge[1] as usize] += 1;
    }

    // Allocate memory for adjacency list
    for i in 0..n {
        adj_list[i] = Vec::with_capacity(adj_len[i]);
        adj_len[i] = 0; // Reset to use as an index
    }

    // Fill adjacency list
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        adj_list[u].push(v);
        adj_list[v].push(u);
        adj_len[u] += 1;
        adj_len[v] += 1;
    }

    (adj_list, adj_len)
}

// BFS to count nodes of each color
fn bfs_count(adj_list: &Vec<Vec<i32>>, n: usize) -> ([usize; 2], Vec<usize>) {
    let mut color_count = [0, 0];
    let mut node_color: Vec<usize> = vec![0; n];
    let mut visited: Vec<bool> = vec![false; n];
    let mut queue: Vec<usize> = Vec::new();
    let mut front = 0;
    let mut rear = 0;

    queue.push(0); // Start BFS from node 0
    node_color[0] = 0; // Color of root is 0
    visited[0] = true;
    rear += 1;

    while front < rear {
        let curr = queue[front];
        front += 1;
        let color = node_color[curr];
        color_count[color] += 1;

        for &neighbor in &adj_list[curr] {
            let neighbor = neighbor as usize;
            if !visited[neighbor] {
                visited[neighbor] = true;
                node_color[neighbor] = 1 - color; // Alternate color
                queue.push(neighbor);
                rear += 1;
            }
        }
    }

    (color_count, node_color)
}

// Main solution function
fn max_target_nodes(
    edges1: &Vec<Vec<i32>>,
    edges2: &Vec<Vec<i32>>,
) -> Vec<i32> {
    let n1 = edges1.len() + 1; // Number of nodes in tree 1
    let n2 = edges2.len() + 1; // Number of nodes in tree 2

    // Build adjacency lists
    let (adj_list1, _adj_len1) = build_adjacency_list(edges1, n1);
    let (adj_list2, _adj_len2) = build_adjacency_list(edges2, n2);

    // Color count and node color arrays
    let (color_count1, node_color1) = bfs_count(&adj_list1, n1);
    let (color_count2, node_color2) = bfs_count(&adj_list2, n2);

    // Calculate results for tree 1
    let mut result: Vec<i32> = Vec::with_capacity(n1);
    let max_color_in_tree2 = color_count2[0].max(color_count2[1]);

    for i in 0..n1 {
        result.push(color_count1[node_color1[i]] as i32 + max_color_in_tree2 as i32);
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    // Input number of edges for tree 1
    let n1: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut edges1: Vec<Vec<i32>> = Vec::with_capacity(n1);

    for _ in 0..n1 {
        let edge_line = lines.next().unwrap();
        let edge_values: Vec<i32> = edge_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges1.push(edge_values);
    }

    // Input number of edges for tree 2
    let n2: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut edges2: Vec<Vec<i32>> = Vec::with_capacity(n2);

    for _ in 0..n2 {
        let edge_line = lines.next().unwrap();
        let edge_values: Vec<i32> = edge_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges2.push(edge_values);
    }

    // Call the solution function
    let result = max_target_nodes(&edges1, &edges2);

    // Output the result
    for &val in &result {
        print!("{} ", val);
    }
    println!();
}