use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap()).filter(|l| !l.is_empty());

    // Read n and threshold from the first line
    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: i32 = parts.next().unwrap().parse().unwrap();
    let threshold: i32 = parts.next().unwrap().parse().unwrap();

    // Read n edges, each with three integers
    let mut edges = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap();
        let edge: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges.push(edge);
    }

    // Compute and print the result
    println!("{}", min_max_weight(n, edges, threshold));
}

fn min_max_weight(n: i32, edges: Vec<Vec<i32>>, _threshold: i32) -> i32 {
    // Closure to check if all nodes are reachable using edges with weight <= lim
    let check = |lim: i32| -> bool {
        let num_nodes = n as usize;
        let mut adj = vec![vec![]; num_nodes];
        for edge in &edges {
            if edge[2] <= lim {
                let from_node = edge[1] as usize;
                let to_node = edge[0] as usize;
                adj[from_node].push(to_node);
            }
        }

        let mut visited = vec![false; num_nodes];
        let mut queue = VecDeque::new();
        queue.push_back(0);
        visited[0] = true;

        while let Some(node) = queue.pop_front() {
            for &neighbor in &adj[node] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }

        visited.iter().all(|&v| v)
    };

    // Determine maximum edge weight
    let mx = edges.iter().map(|edge| edge[2]).max().unwrap_or(0);

    // Early return if even maximum weight is insufficient
    if !check(mx) {
        return -1;
    }

    // Binary search to find the minimal maximum weight
    let mut head = 0;
    let mut tail = mx;
    while head < tail {
        let mid = (head + tail) / 2;
        if check(mid) {
            tail = mid;
        } else {
            head = mid + 1;
        }
    }

    head
}