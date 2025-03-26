use std::collections::VecDeque;
use std::io;
use std::io::BufRead;

// Graph node for adjacency list
#[derive(Debug, Clone)]
struct Node {
    vertex: usize,
    next: Option<Box<Node>>,
}

// Queue implementation for BFS
#[derive(Debug)]
struct Queue {
    data: Vec<usize>,
    front: usize,
    rear: usize,
    capacity: usize,
    size: usize,
}

impl Queue {
    fn new(capacity: usize) -> Self {
        Queue {
            data: vec![0; capacity],
            front: 0,
            rear: capacity - 1,
            capacity,
            size: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn enqueue(&mut self, item: usize) {
        if self.size == self.capacity {
            return;
        }
        self.rear = (self.rear + 1) % self.capacity;
        self.data[self.rear] = item;
        self.size += 1;
    }

    fn dequeue(&mut self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        let item = self.data[self.front];
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        Some(item)
    }
}

// Add a node to the adjacency list
fn add_edge(adj_list: &mut Vec<Option<Box<Node>>>, src: usize, dest: usize) {
    let new_node = Node {
        vertex: dest,
        next: adj_list[src].take(),
    };
    adj_list[src] = Some(Box::new(new_node));
}

// Check if all nodes can be reached from node 0 with edges of weight <= limit
fn check(n: usize, edges: &Vec<Vec<i32>>, limit: i32) -> bool {
    // Create adjacency list - using only edges with weight <= limit
    let mut adj_list: Vec<Option<Box<Node>>> = vec![None; n];

    for edge in edges {
        if edge[2] <= limit {
            add_edge(&mut adj_list, edge[1] as usize, edge[0] as usize); // Reverse edge
        }
    }

    // BFS to check if all nodes can be reached from node 0
    let mut visited = vec![false; n];
    let mut queue = Queue::new(n);

    visited[0] = true;
    queue.enqueue(0);

    while let Some(current_vertex) = queue.dequeue() {
        let mut temp = adj_list[current_vertex].take();
        while let Some(mut node) = temp {
            let adj_vertex = node.vertex;
            if !visited[adj_vertex] {
                visited[adj_vertex] = true;
                queue.enqueue(adj_vertex);
            }
            temp = node.next.take();
        }
    }

    // Check if all nodes were visited
    let all_visited = visited.iter().all(|&v| v);

    all_visited
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
fn min_max_weight(n: usize, edges: &Vec<Vec<i32>>) -> i32 {
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and threshold
    let first_line = lines.next().unwrap()?;
    let mut first_split = first_line.split_whitespace();
    let n: usize = first_split.next().unwrap().parse().unwrap();
    let _threshold: i32 = first_split.next().unwrap().parse().unwrap();

    // Read edges
    let mut edges: Vec<Vec<i32>> = Vec::new();
    while let Some(line) = lines.next() {
        let line = line?;
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if values.len() == 3 {
            edges.push(values);
        } else {
            break;
        }
    }

    // Call the solution function
    let result = min_max_weight(n, &edges);
    println!("{}", result);

    Ok(())
}