use std::collections::VecDeque;
use std::io::{self, BufRead};

// Graph node for adjacency list
struct Node {
    vertex: usize,
    next: Option<Box<Node>>,
}

// Queue implementation for BFS
struct Queue {
    data: VecDeque<usize>,
}

impl Queue {
    fn new(capacity: usize) -> Self {
        Queue {
            data: VecDeque::with_capacity(capacity),
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn enqueue(&mut self, item: usize) {
        self.data.push_back(item);
    }

    fn dequeue(&mut self) -> Option<usize> {
        self.data.pop_front()
    }
}

// Add a node to the adjacency list
fn add_edge(adj_list: &mut Vec<Option<Box<Node>>>, src: usize, dest: usize) {
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
            add_edge(&mut adj_list, edge[1] as usize, edge[0] as usize);
        }
    }

    // BFS to check if all nodes can be reached from node 0
    let mut visited = vec![false; n];
    let mut queue = Queue::new(n);

    visited[0] = true;
    queue.enqueue(0);

    while !queue.is_empty() {
        let current_vertex = queue.dequeue().unwrap();
        let mut temp = &adj_list[current_vertex];

        while let Some(node) = temp {
            let adj_vertex = node.vertex;
            if !visited[adj_vertex] {
                visited[adj_vertex] = true;
                queue.enqueue(adj_vertex);
            }
            temp = &node.next;
        }
    }

    // Check if all nodes were visited
    visited.iter().all(|&v| v)
}

// Find the maximum weight among all edges
fn find_max_weight(edges: &Vec<Vec<i32>>) -> i32 {
    edges.iter().map(|edge| edge[2]).max().unwrap_or(0)
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
    let mut lines = stdin.lock().lines();

    // Read n and threshold
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let threshold: i32 = parts.next().unwrap().parse().unwrap();

    // Read all the edges from the input until EOF
    let mut edges: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let src: i32 = parts.next().unwrap().parse().unwrap();
        let dest: i32 = parts.next().unwrap().parse().unwrap();
        let weight: i32 = parts.next().unwrap().parse().unwrap();
        edges.push(vec![src, dest, weight]);
    }

    // Call the solution function
    let result = min_max_weight(n, &edges, threshold);
    println!("{}", result);
}