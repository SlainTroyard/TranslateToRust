use std::collections::VecDeque;
use std::io;

fn build_adjacency_list(edges: Vec<Vec<i32>>, n: usize) -> (Vec<Vec<i32>>, Vec<usize>) {
    let mut adj_list = vec![Vec::new(); n];
    let mut adj_len = vec![0; n];

    // Count the degree of each node
    for edge in &edges {
        adj_len[edge[0] as usize] += 1;
        adj_len[edge[1] as usize] += 1;
    }

    // Fill adjacency list
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        adj_list[u].push(v as i32);
        adj_list[v].push(u as i32);
    }

    (adj_list, adj_len)
}

fn bfs_count(
    adj_list: &[Vec<i32>],
    n: usize,
    color_count: &mut [usize; 2],
    node_color: &mut Vec<u8>,
) {
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();

    queue.push_back(0); // Start BFS from node 0
    node_color[0] = 0;  // Color of root is 0
    visited[0] = true;

    while let Some(curr) = queue.pop_front() {
        let color = node_color[curr];
        color_count[color as usize] += 1;

        for &neighbor in &adj_list[curr] {
            let neighbor = neighbor as usize;
            if !visited[neighbor] {
                visited[neighbor] = true;
                node_color[neighbor] = 1 - color; // Alternate color
                queue.push_back(neighbor);
            }
        }
    }
}

fn max_target_nodes(
    edges1: Vec<Vec<i32>>,
    edges2: Vec<Vec<i32>>,
) -> Vec<usize> {
    let n1 = edges1.len() + 1; // Number of nodes in tree 1
    let n2 = edges2.len() + 1; // Number of nodes in tree 2

    // Build adjacency lists
    let (adj_list1, _) = build_adjacency_list(edges1, n1);
    let (adj_list2, _) = build_adjacency_list(edges2, n2);

    // Color count and node color arrays
    let mut color_count1 = [0, 0];
    let mut color_count2 = [0, 0];
    let mut node_color1 = vec![0; n1];
    let mut node_color2 = vec![0; n2];

    // Perform BFS to calculate color distribution
    bfs_count(&adj_list1, n1, &mut color_count1, &mut node_color1);
    bfs_count(&adj_list2, n2, &mut color_count2, &mut node_color2);

    // Calculate results for tree 1
    let max_color_in_tree2 = std::cmp::max(color_count2[0], color_count2[1]);

    let mut result = vec![0; n1];
    for i in 0..n1 {
        result[i] = color_count1[node_color1[i] as usize] + max_color_in_tree2;
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    // Parse number of edges for tree 1
    let n1 = lines.next().unwrap().parse::<usize>().unwrap();
    let mut edges1 = Vec::with_capacity(n1);
    for _ in 0..n1 {
        let line = lines.next().unwrap();
        let edge: Vec<i32> = line.split_whitespace()
                                 .map(|x| x.parse::<i32>().unwrap())
                                 .collect();
        edges1.push(edge);
    }

    // Parse number of edges for tree 2
    let n2 = lines.next().unwrap().parse::<usize>().unwrap();
    let mut edges2 = Vec::with_capacity(n2);
    for _ in 0..n2 {
        let line = lines.next().unwrap();
        let edge: Vec<i32> = line.split_whitespace()
                                 .map(|x| x.parse::<i32>().unwrap())
                                 .collect();
        edges2.push(edge);
    }

    // Call the solution function
    let result = max_target_nodes(edges1, edges2);

    // Output the result
    for value in result {
        print!("{} ", value);
    }
    println!();
}