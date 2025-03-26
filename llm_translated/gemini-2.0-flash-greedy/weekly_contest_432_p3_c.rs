use std::collections::VecDeque;
use std::io;
use std::io::Read;
use std::process;

// Graph node for adjacency list
#[derive(Debug)]
struct Node {
    vertex: usize,
    next: Option<Box<Node>>,
}

// Queue implementation for BFS
struct Queue {
    data: VecDeque<usize>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            data: VecDeque::new(),
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
            add_edge(&mut adj_list, edge[1] as usize, edge[0] as usize); // Reverse edge as in C++ code
        }
    }

    // BFS to check if all nodes can be reached from node 0
    let mut visited = vec![false; n];

    let mut queue = Queue::new();

    visited[0] = true;
    queue.enqueue(0);

    while !queue.is_empty() {
        let current_vertex = queue.dequeue().unwrap();
        let mut temp = adj_list[current_vertex].as_deref();

        while let Some(node) = temp {
            let adj_vertex = node.vertex;
            if !visited[adj_vertex] {
                visited[adj_vertex] = true;
                queue.enqueue(adj_vertex);
            }
            temp = node.next.as_deref();
        }
    }

    // Check if all nodes were visited
    let all_visited = visited.iter().all(|&v| v);

    return all_visited;
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

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut first_line_iter = first_line.split_whitespace();

    let n: usize = first_line_iter.next().unwrap().parse().unwrap();
    let _threshold: i32 = first_line_iter.next().unwrap().parse().unwrap();

    let mut edges: Vec<Vec<i32>> = Vec::new();

    while let Some(line) = lines.next() {
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if values.len() == 3 {
            edges.push(values);
        }
    }

    let result = min_max_weight(n, &edges, 0);
    println!("{}", result);

    Ok(())
}