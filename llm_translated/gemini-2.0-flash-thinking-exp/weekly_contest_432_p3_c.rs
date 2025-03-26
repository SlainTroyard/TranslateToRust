use std::io::{self, BufRead};
use std::process;
use std::collections::VecDeque;

// Graph node for adjacency list
struct Node {
    vertex: i32,
    next: Option<Box<Node>>,
}

// Queue implementation for BFS
struct Queue {
    data: VecDeque<i32>,
}

impl Queue {
    fn new() -> Self {
        Queue { data: VecDeque::new() }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn enqueue(&mut self, item: i32) {
        self.data.push_back(item);
    }

    fn dequeue(&mut self) -> Option<i32> {
        self.data.pop_front()
    }
}

// Add a node to the adjacency list
fn add_edge(adj_list: &mut Vec<Option<Box<Node>>>, src: usize, dest: i32) {
    let new_node = Box::new(Node {
        vertex: dest,
        next: adj_list[src].take(),
    });
    adj_list[src] = Some(new_node);
}

// Check if all nodes can be reached from node 0 with edges of weight <= limit
fn check(n: usize, edges: &Vec<Vec<i32>>, limit: i32) -> bool {
    // Create adjacency list - using only edges with weight <= limit
    let mut adj_list: Vec<Option<Box<Node>>> = vec![None; n];

    for edge in edges {
        if edge[2] <= limit {
            add_edge(&mut adj_list, edge[1] as usize, edge[0]); // Reverse edge as in C++ code
        }
    }

    // BFS to check if all nodes can be reached from node 0
    let mut visited: Vec<bool> = vec![false; n];
    let mut queue = Queue::new();

    visited[0] = true;
    queue.enqueue(0);

    while !queue.is_empty() {
        if let Some(current_vertex) = queue.dequeue() {
            let mut temp = &adj_list[current_vertex as usize];
            while let Some(node) = temp {
                let adj_vertex = node.vertex;
                if !visited[adj_vertex as usize] {
                    visited[adj_vertex as usize] = true;
                    queue.enqueue(adj_vertex);
                }
                temp = &node.next;
            }
        }
    }

    // Check if all nodes were visited
    for i in 0..n {
        if !visited[i] {
            return false;
        }
    }

    true
}

// Find the maximum weight among all edges
fn find_max_weight(edges: &Vec<Vec<i32>>) -> i32 {
    let mut max_weight = 0;
    for edge in edges {
        if edge[2] > max_weight {
            max_weight = edge[2];
        }
    }
    max_weight
}

// Main solution function
fn min_max_weight(n: usize, edges: &Vec<Vec<i32>>, threshold: i32) -> i32 {
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
    let mut iterator = stdin.lock().lines();

    let first_line = iterator.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let n: usize = first_line_iter.next().unwrap().parse().unwrap();
    let threshold: i32 = first_line_iter.next().unwrap().parse().unwrap();

    let mut edges: Vec<Vec<i32>> = Vec::new();
    for line in iterator {
        let line_str = line.unwrap();
        let parts: Vec<i32> = line_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if parts.len() == 3 {
            edges.push(parts);
        } else {
            break; // Stop reading if line format is incorrect or EOF is reached
        }
    }

    let result = min_max_weight(n, &edges, threshold);
    println!("{}", result);
}