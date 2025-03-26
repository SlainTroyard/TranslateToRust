use std::io::{self, BufRead, Write};

// Graph node for adjacency list
#[derive(Debug)]
struct Node {
    vertex: usize,
    next: Option<Box<Node>>,
}

// Queue implementation for BFS
struct Queue<T> {
    data: Vec<T>,
    front: usize,
    rear: usize,
}

impl<T> Queue<T> {
    fn new(capacity: usize) -> Self {
        Queue {
            data: vec![None; capacity],
            front: 0,
            rear: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.front == self.rear && self.data[self.front].is_none()
    }

    fn enqueue(&mut self, item: T) {
        if self.front == (self.rear + 1) % self.data.len() {
            return;
        }
        self.data[self.rear] = Some(item);
        self.rear = (self.rear + 1) % self.data.len();
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let item = self.data[self.front].take();
        self.front = (self.front + 1) % self.data.len();
        item
    }
}

// Add a node to the adjacency list
fn add_edge(adj_list: &mut Vec<Option<Box<Node>>>, src: usize, dest: usize) {
    let new_node = Box::new(Node { vertex: dest, next: adj_list[src].take() });
    adj_list[src] = Some(new_node);
}

// Check if all nodes can be reached from node 0 with edges of weight <= limit
fn check(n: usize, edges: &[(usize, usize, i32)], limit: i32) -> bool {
    let mut adj_list: Vec<Option<Box<Node>>> = vec![None; n];

    for &(src, dest, weight) in edges.iter().filter(|&&(_, _, w)| w <= limit) {
        add_edge(&mut adj_list, dest, src); // Reverse edge as in C++ code
    }

    let mut visited = vec![false; n];
    let mut queue = Queue::new(n);

    visited[0] = true;
    queue.enqueue(0);

    while let Some(current_vertex) = queue.dequeue() {
        if let Some(mut temp) = adj_list[current_vertex].take() {
            while let Some(node) = temp {
                let adj_vertex = node.vertex;
                if !visited[adj_vertex] {
                    visited[adj_vertex] = true;
                    queue.enqueue(adj_vertex);
                }
                temp = node.next;
            }
        }
    }

    visited.iter().all(|&v| v)
}

// Find the maximum weight among all edges
fn find_max_weight(edges: &[(usize, usize, i32)]) -> i32 {
    edges.iter().map(|&(_, _, w)| w).max().unwrap_or(0)
}

// Main solution function
fn min_max_weight(n: usize, edges: &[(usize, usize, i32)], threshold: usize) -> i32 {
    let max_weight = find_max_weight(edges);

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

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut stdout_lock = stdout.lock();

    let mut input = String::new();
    stdin_lock.read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid input for n");
    let threshold: usize = iter.next().unwrap().parse().expect("Invalid input for threshold");

    let mut edges: Vec<(usize, usize, i32)> = Vec::new();
    let mut buffer = String::new();
    while stdin_lock.read_line(&mut buffer).unwrap() > 0 {
        let mut parts = buffer.trim().split_whitespace();
        let src: usize = parts.next().unwrap().parse().expect("Invalid input for src");
        let dest: usize = parts.next().unwrap().parse().expect("Invalid input for dest");
        let weight: i32 = parts.next().unwrap().parse().expect("Invalid input for weight");
        edges.push((src, dest, weight));
        buffer.clear();
    }

    let result = min_max_weight(n, &edges, threshold);
    writeln!(stdout_lock, "{}", result).expect("Failed to write result");
}