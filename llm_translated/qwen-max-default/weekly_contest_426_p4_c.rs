use std::io::{self, BufRead};

// Function to build adjacency list for a tree
fn build_adjacency_list(edges: &Vec<(usize, usize)>, n: usize) -> (Vec<Vec<usize>>, Vec<usize>) {
    let mut adj_list = vec![Vec::new(); n];
    let mut adj_len = vec![0; n];

    // Count the degree of each node
    for &(u, v) in edges.iter() {
        adj_len[u] += 1;
        adj_len[v] += 1;
    }

    // Allocate memory for adjacency list
    for i in 0..n {
        adj_list[i].reserve(adj_len[i]);
        adj_len[i] = 0; // Reset to use as an index
    }

    // Fill adjacency list
    for &(u, v) in edges.iter() {
        adj_list[u].push(v);
        adj_list[v].push(u);
        adj_len[u] += 1;
        adj_len[v] += 1;
    }

    (adj_list, adj_len)
}

// BFS to count nodes of each color
fn bfs_count(adj_list: &Vec<Vec<usize>>, n: usize) -> (Vec<usize>, Vec<usize>) {
    let mut visited = vec![false; n];
    let mut queue = Vec::new();
    let mut color_count = vec![0, 0];
    let mut node_color = vec![0; n];

    queue.push(0); // Start BFS from node 0
    node_color[0] = 0; // Color of root is 0
    visited[0] = true;

    while let Some(curr) = queue.pop() {
        let color = node_color[curr];
        color_count[color] += 1;

        for &neighbor in &adj_list[curr] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                node_color[neighbor] = 1 - color; // Alternate color
                queue.push(neighbor);
            }
        }
    }

    (color_count, node_color)
}

// Main solution function
fn max_target_nodes(edges1: &Vec<(usize, usize)>, edges2: &Vec<(usize, usize)>, n1: usize, n2: usize) -> Vec<usize> {
    // Build adjacency lists
    let (adj_list1, _) = build_adjacency_list(edges1, n1);
    let (adj_list2, _) = build_adjacency_list(edges2, n2);

    // Color count and node color arrays
    let (color_count1, node_color1) = bfs_count(&adj_list1, n1);
    let (color_count2, node_color2) = bfs_count(&adj_list2, n2);

    // Calculate results for tree 1
    let max_color_in_tree2 = *color_count2.iter().max().unwrap();
    let mut result = Vec::with_capacity(n1);

    for i in 0..n1 {
        result.push(color_count1[node_color1[i]] + max_color_in_tree2);
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input number of edges for tree 1
    let n1: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut edges1 = Vec::with_capacity(n1);
    for _ in 0..n1 {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        edges1.push((parts[0], parts[1]));
    }

    // Input number of edges for tree 2
    let n2: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut edges2 = Vec::with_capacity(n2);
    for _ in 0..n2 {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        edges2.push((parts[0], parts[1]));
    }

    // Call the solution function
    let result = max_target_nodes(&edges1, &edges2, n1 + 1, n2 + 1);

    // Output the result
    for value in result {
        print!("{} ", value);
    }
    println!();
}