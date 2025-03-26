use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead, Write};

// Graph node for adjacency list
struct Node {
    vertex: usize,
    next: Option<Box<Node>>,
}

// Add a node to the adjacency list
fn add_edge(adj_list: &mut HashMap<usize, Option<Box<Node>>>, src: usize, dest: usize) {
    let new_node = Box::new(Node {
        vertex: dest,
        next: adj_list.entry(src).or_insert(None),
    });
    adj_list.insert(src, Some(new_node));
}

// Check if all nodes can be reached from node 0 with edges of weight <= limit
fn check(n: usize, edges: &[(usize, usize, i32)], limit: i32) -> bool {
    // Create adjacency list - using only edges with weight <= limit
    let mut adj_list: HashMap<usize, Option<Box<Node>>> = HashMap::new();
    for &(src, dest, weight) in edges.iter() {
        if weight <= limit {
            add_edge(&mut adj_list, dest, src); // Reverse edge as in C++ code
        }
    }

    // BFS to check if all nodes can be reached from node 0
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();

    visited[0] = true;
    queue.push_back(0);

    while let Some(current_vertex) = queue.pop_front() {
        if let Some(mut current) = adj_list.get(&current_vertex).cloned() {
            while let Some(node) = current {
                let adj_vertex = node.vertex;
                if !visited[adj_vertex] {
                    visited[adj_vertex] = true;
                    queue.push_back(adj_vertex);
                }
                current = node.next;
            }
        }
    }

    // Check if all nodes were visited
    visited.iter().all(|&v| v)
}

// Find the maximum weight among all edges
fn find_max_weight(edges: &[(usize, usize, i32)]) -> i32 {
    edges.iter().map(|&(_, _, weight)| weight).max().unwrap_or(0)
}

// Main solution function
fn min_max_weight(n: usize, edges: &[(usize, usize, i32)], threshold: i32) -> i32 {
    let max_weight = find_max_weight(edges);

    // Check if it's possible to reach all nodes even with max weight
    if !check(n, edges, max_weight) {
        return -1;
    }

    // Binary search for the minimal maximum weight
    let mut left = 0;
    let mut right = max_weight;

    while left < right {
        let mid = left + (right - left) / 2;
        if check(n, edges, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read n and threshold
    stdin.read_line(&mut buffer).expect("Failed to read line");
    let mut iter = buffer.split_whitespace();
    let n: usize = iter.next().and_then(|s| s.parse().ok()).expect("Invalid input for n");
    let threshold: i32 = iter.next().and_then(|s| s.parse().ok()).expect("Invalid input for threshold");
    buffer.clear();

    // Initialize edges array
    let mut edges: Vec<(usize, usize, i32)> = Vec::new();

    // Read all the edges from the input until EOF
    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        let mut parts = line.split_whitespace();
        let src: usize = parts.next().and_then(|s| s.parse().ok()).expect("Invalid input for src");
        let dest: usize = parts.next().and_then(|s| s.parse().ok()).expect("Invalid input for dest");
        let weight: i32 = parts.next().and_then(|s| s.parse().ok()).expect("Invalid input for weight");
        edges.push((src, dest, weight));
    }

    // Call the solution function
    let result = min_max_weight(n, &edges, threshold);
    writeln!(stdout, "{}", result).expect("Failed to write result");
}