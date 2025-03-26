use std::collections::VecDeque;
use std::io::{self, Read};

fn build_adjacency(n: usize, edges: &[[usize; 2]]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for &[u, v] in edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    adj
}

fn bfs(adj: &[Vec<usize>], n: usize) -> ([i32; 2], Vec<usize>) {
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    let mut node_color = vec![0; n];
    let mut color_count = [0, 0];

    queue.push_back(0);
    visited[0] = true;
    node_color[0] = 0;
    color_count[0] += 1;

    while let Some(curr) = queue.pop_front() {
        let color = node_color[curr];
        for &neighbor in &adj[curr] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                node_color[neighbor] = 1 - color;
                color_count[node_color[neighbor]] += 1;
                queue.push_back(neighbor);
            }
        }
    }

    (color_count, node_color)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let mut next = || tokens.next().unwrap();

    // Read tree 1 edges
    let n1_edges = next();
    let mut edges1 = Vec::new();
    for _ in 0..n1_edges {
        let u = next() as usize;
        let v = next() as usize;
        edges1.push([u, v]);
    }

    // Read tree 2 edges
    let n2_edges = next();
    let mut edges2 = Vec::new();
    for _ in 0..n2_edges {
        let u = next() as usize;
        let v = next() as usize;
        edges2.push([u, v]);
    }

    // Process trees
    let n1_nodes = edges1.len() + 1;
    let n2_nodes = edges2.len() + 1;

    let adj1 = build_adjacency(n1_nodes, &edges1);
    let adj2 = build_adjacency(n2_nodes, &edges2);

    let (color_count1, node_color1) = bfs(&adj1, n1_nodes);
    let (color_count2, _) = bfs(&adj2, n2_nodes);
    let max_color2 = color_count2[0].max(color_count2[1]);

    let result: Vec<i32> = node_color1
        .into_iter()
        .map(|c| color_count1[c] + max_color2)
        .collect();

    // Format output as space-separated integers
    let output = result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", output);
}