use std::io::{self, BufRead};

// Function to build adjacency list for a tree
fn build_adjacency_list(edges: &Vec<Vec<i32>>, n: usize) -> (Vec<Vec<i32>>, Vec<i32>) {
    let mut adj_list = vec![Vec::new(); n];
    let mut adj_len = vec![0; n];

    // Count the degree of each node
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        adj_len[u] += 1;
        adj_len[v] += 1;
    }

    // Allocate memory for adjacency list
    for i in 0..n {
        adj_list[i] = vec![0; adj_len[i] as usize];
        adj_len[i] = 0; // Reset to use as an index
    }

    // Fill adjacency list
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        adj_list[u][adj_len[u] as usize] = v as i32;
        adj_len[u] += 1;
        adj_list[v][adj_len[v] as usize] = u as i32;
        adj_len[v] += 1;
    }

    (adj_list, adj_len)
}

// BFS to count nodes of each color
fn bfs_count(adj_list: &Vec<Vec<i32>>, adj_len: &Vec<i32>, n: usize, color_count: &mut [i32; 2], node_color: &mut Vec<i32>) {
    let mut visited = vec![false; n];
    let mut queue = Vec::with_capacity(n);

    queue.push(0); // Start BFS from node 0
    node_color[0] = 0;  // Color of root is 0
    visited[0] = true;

    let mut front = 0;
    while front < queue.len() {
        let curr = queue[front];
        front += 1;
        let color = node_color[curr as usize];
        color_count[color as usize] += 1;

        for i in 0..adj_len[curr as usize] {
            let neighbor = adj_list[curr as usize][i as usize] as usize;
            if !visited[neighbor] {
                visited[neighbor] = true;
                node_color[neighbor] = 1 - color; // Alternate color
                queue.push(neighbor);
            }
        }
    }
}

// Main solution function
fn max_target_nodes(edges1: &Vec<Vec<i32>>, edges2: &Vec<Vec<i32>>) -> Vec<i32> {
    let n1 = edges1.len() + 1; // Number of nodes in tree 1
    let n2 = edges2.len() + 1; // Number of nodes in tree 2

    // Build adjacency lists
    let (adj_list1, adj_len1) = build_adjacency_list(edges1, n1);
    let (adj_list2, adj_len2) = build_adjacency_list(edges2, n2);

    // Color count and node color arrays
    let mut color_count1 = [0, 0];
    let mut color_count2 = [0, 0];
    let mut node_color1 = vec![0; n1];
    let mut node_color2 = vec![0; n2];

    // Perform BFS to calculate color distribution
    bfs_count(&adj_list1, &adj_len1, n1, &mut color_count1, &mut node_color1);
    bfs_count(&adj_list2, &adj_len2, n2, &mut color_count2, &mut node_color2);

    // Calculate results for tree 1
    let mut result = vec![0; n1];
    let max_color_in_tree2 = color_count2[0].max(color_count2[1]);

    for i in 0..n1 {
        result[i] = color_count1[node_color1[i] as usize] + max_color_in_tree2;
    }

    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input number of edges for tree 1
    let n1: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut edges1 = Vec::with_capacity(n1);

    for _ in 0..n1 {
        let line = lines.next().unwrap()?;
        let values: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        edges1.push(values);
    }

    // Input number of edges for tree 2
    let n2: usize = lines.next().unwrap()?.trim().parse().unwrap();
    let mut edges2 = Vec::with_capacity(n2);

    for _ in 0..n2 {
        let line = lines.next().unwrap()?;
        let values: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        edges2.push(values);
    }

    // Call the solution function
    let result = max_target_nodes(&edges1, &edges2);

    // Output the result
    for &value in &result {
        print!("{} ", value);
    }
    println!();

    Ok(())
}