use std::io;
use std::io::BufRead;
use std::vec;

/* Function to build adjacency list for a tree */
fn build_adjacency_list(edges: &Vec<Vec<i32>>, n: usize) -> Vec<Vec<i32>> {
    let mut adj_list: Vec<Vec<i32>> = vec![vec![]; n];

    // Fill adjacency list
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        adj_list[u].push(v as i32);
        adj_list[v].push(u as i32);
    }
    adj_list
}

/* BFS to count nodes of each color */
fn bfs_count(adj_list: &Vec<Vec<i32>>, n: usize, color_count: &mut [i32; 2], node_color: &mut Vec<i32>) {
    let mut visited: Vec<bool> = vec![false; n];
    let mut queue: Vec<usize> = vec![];
    let mut head = 0;

    queue.push(0); // Start BFS from node 0
    node_color[0] = 0;  // Color of root is 0
    visited[0] = true;

    while head < queue.len() {
        let curr = queue[head];
        head += 1;
        let color = node_color[curr];
        color_count[color as usize] += 1;

        for &neighbor_node in &adj_list[curr] {
            let neighbor = neighbor_node as usize;
            if !visited[neighbor] {
                visited[neighbor] = true;
                node_color[neighbor] = 1 - color; // Alternate color
                queue.push(neighbor);
            }
        }
    }
}

/* Main solution function */
fn max_target_nodes(edges1: &Vec<Vec<i32>>, edges2: &Vec<Vec<i32>>) -> Vec<i32> {
    let n1 = edges1.len() + 1; // Number of nodes in tree 1
    let n2 = edges2.len() + 1; // Number of nodes in tree 2

    // Build adjacency lists
    let adj_list1 = build_adjacency_list(edges1, n1);
    let adj_list2 = build_adjacency_list(edges2, n2);

    // Color count and node color arrays
    let mut color_count1 = [0, 0];
    let mut color_count2 = [0, 0];
    let mut node_color1 = vec![0; n1];
    let mut node_color2 = vec![0; n2];

    // Perform BFS to calculate color distribution
    bfs_count(&adj_list1, n1, &mut color_count1, &mut node_color1);
    bfs_count(&adj_list2, n2, &mut color_count2, &mut node_color2);

    // Calculate results for tree 1
    let mut result = vec![0; n1];
    let max_color_in_tree2 = if color_count2[0] > color_count2[1] { color_count2[0] } else { color_count2[1] };

    for i in 0..n1 {
        result[i] = color_count1[node_color1[i] as usize] + max_color_in_tree2;
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input number of edges for tree 1
    let n1_edges = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
    let mut edges1: Vec<Vec<i32>> = Vec::new();

    for _ in 0..n1_edges {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        edges1.push(nums);
    }

    // Input number of edges for tree 2
    let n2_edges = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
    let mut edges2: Vec<Vec<i32>> = Vec::new();

    for _ in 0..n2_edges {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        edges2.push(nums);
    }

    // Call the solution function
    let result = max_target_nodes(&edges1, &edges2);

    // Output the result
    for i in 0..result.len() {
        print!("{} ", result[i]);
    }
    println!();
}