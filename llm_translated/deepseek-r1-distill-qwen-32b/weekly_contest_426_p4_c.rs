use std::collections::VecDeque;
use std::io;

fn build_adjacency_list(n: usize, edges: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut adj = vec![Vec::new(); n];
    for edge in edges {
        let u = edge[0];
        let v = edge[1];
        adj[u].push(v);
        adj[v].push(u);
    }
    adj
}

fn bfs_count(adj: &Vec<Vec<usize>>) -> (Vec<i32>, [i32; 2]) {
    let n = adj.len();
    let mut visited = vec![false; n];
    let mut node_color = vec![0; n];
    let mut color_count = [0, 0];
    
    let mut queue = VecDeque::new();
    queue.push_back(0);
    visited[0] = true;
    node_color[0] = 0;
    
    while let Some(curr) = queue.pop_front() {
        let color = node_color[curr];
        color_count[color as usize] += 1;
        
        for neighbor in &adj[curr] {
            if !visited[*neighbor] {
                visited[*neighbor] = true;
                node_color[*neighbor] = 1 - color;
                queue.push_back(*neighbor);
            }
        }
    }
    
    (node_color, color_count)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<&str> = input.split_whitespace().collect();
    
    let mut ptr = 0;
    let n1 = tokens[ptr].parse::<usize>().unwrap();
    ptr += 1;
    
    let mut edges1 = Vec::new();
    for _ in 0..n1 {
        let u = tokens[ptr].parse::<usize>().unwrap();
        let v = tokens[ptr + 1].parse::<usize>().unwrap();
        edges1.push(vec![u, v]);
        ptr += 2;
    }
    
    let n2 = tokens[ptr].parse::<usize>().unwrap();
    ptr += 1;
    
    let mut edges2 = Vec::new();
    for _ in 0..n2 {
        let u = tokens[ptr].parse::<usize>().unwrap();
        let v = tokens[ptr + 1].parse::<usize>().unwrap();
        edges2.push(vec![u, v]);
        ptr += 2;
    }
    
    let adj1 = build_adjacency_list(n1 + 1, &edges1);
    let adj2 = build_adjacency_list(n2 + 1, &edges2);
    
    let (_, color_count1) = bfs_count(&adj1);
    let (_, color_count2) = bfs_count(&adj2);
    
    let max_color2 = color_count2[0].max(color_count2[1]);
    
    let result: Vec<i32> = adj1
        .iter()
        .enumerate()
        .map(|(i, _)| {
            if i == 0 {
                0
            } else {
                let color = bfs_count(&adj1).0[i];
                color_count1[color as usize] + max_color2
            }
        })
        .collect();
    
    println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}