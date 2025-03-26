use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};
use std::str::FromStr;

// Graph representation using adjacency list
#[derive(Debug)]
struct Graph {
    adj_list: Vec<Vec<usize>>, // Adjacency list storage
}

impl Graph {
    fn new(size: usize) -> Self {
        Self {
            adj_list: vec![vec![]; size],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj_list[src].push(dest);
    }
}

// BFS function to check if all nodes can be reached from node 0
fn bfs_check(graph: &Graph, n: usize) -> bool {
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();

    visited[0] = true;
    queue.push_back(0);

    while let Some(current) = queue.pop_front() {
        for &neighbor in &graph.adj_list[current] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    visited.iter().all(|&v| v)
}

// Helper function to construct a graph with edges ≤ limit weight
fn construct_graph(n: usize, edges: &[(usize, usize, i32)], limit: i32) -> Graph {
    let mut graph = Graph::new(n);
    for &(src, dest, weight) in edges {
        if weight <= limit {
            graph.add_edge(dest, src); // Reverse edge as in C++ code
        }
    }
    graph
}

// Function to find the maximum weight among all edges
fn find_max_weight(edges: &[(usize, usize, i32)]) -> i32 {
    edges.iter().map(|&(_, _, weight)| weight).max().unwrap_or(0)
}

// Check function as described in the C code
fn check(n: usize, edges: &[(usize, usize, i32)], limit: i32) -> bool {
    let graph = construct_graph(n, edges, limit);
    bfs_check(&graph, n)
}

// Binary search to find the minimal maximum weight
fn min_max_weight(n: usize, edges: &[(usize, usize, i32)]) -> i32 {
    let max_weight = find_max_weight(edges);

    // Check if all nodes are reachable even with the maximum weight
    if !check(n, edges, max_weight) {
        return -1;
    }

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

// Utility function to parse input and build the edge list
fn parse_input() -> (usize, i32, Vec<(usize, usize, i32)>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read first line: n and threshold
    let first_line = lines.next().unwrap().unwrap();
    let mut first_split = first_line.split_whitespace();
    let n = usize::from_str(first_split.next().unwrap()).unwrap();
    let threshold = i32::from_str(first_split.next().unwrap()).unwrap();

    // Read edges until EOF
    let mut edges = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let src = usize::from_str(parts.next().unwrap()).unwrap();
        let dest = usize::from_str(parts.next().unwrap()).unwrap();
        let weight = i32::from_str(parts.next().unwrap()).unwrap();
        edges.push((src, dest, weight));
    }

    (n, threshold, edges)
}

// Main function to run the solution
fn main() {
    let (n, _threshold, edges) = parse_input();
    let result = min_max_weight(n, &edges);
    println!("{}", result);
}