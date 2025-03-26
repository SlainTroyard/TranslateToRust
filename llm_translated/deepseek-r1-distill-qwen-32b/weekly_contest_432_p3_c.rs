use std::collections::VecDeque;
use std::io;

// Check if all nodes can be reached from node 0 with edges of weight <= limit
fn check(n: i32, edges: &Vec<Vec<i32>>, limit: i32) -> bool {
    let mut adj_list = vec![Vec::new(); n as usize];
    for edge in edges {
        let u = edge[0];
        let v = edge[1];
        let w = edge[2];
        if w <= limit {
            adj_list[v as usize].push(u);
        }
    }

    let mut visited = vec![false; n as usize];
    let mut queue = VecDeque::new();
    visited[0] = true;
    queue.push_back(0);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        for &neighbor in &adj_list[current as usize] {
            if !visited[neighbor as usize] {
                visited[neighbor as usize] = true;
                queue.push_back(neighbor);
            }
        }
    }

    visited.iter().all(|&v| v)
}

// Find the maximum weight among all edges
fn find_max_weight(edges: &Vec<Vec<i32>>) -> i32 {
    edges.iter()
        .map(|edge| edge[2])
        .max()
        .unwrap_or(0)
}

// Main solution function
fn min_max_weight(n: i32, edges: &Vec<Vec<i32>>, threshold: i32) -> i32 {
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

    if check(n, edges, left) {
        left
    } else {
        -1
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());

    // Read n and threshold
    let first_line = lines.next().expect("No input");
    let mut parts = first_line.split_whitespace();
    let n: i32 = parts.next().unwrap().parse().unwrap();
    let threshold: i32 = parts.next().unwrap().parse().unwrap();

    // Read edges
    let mut edges = Vec::new();
    for line in lines {
        let mut parts = line.split_whitespace();
        let u: i32 = parts.next().unwrap().parse().unwrap();
        let v: i32 = parts.next().unwrap().parse().unwrap();
        let w: i32 = parts.next().unwrap().parse().unwrap();
        edges.push(vec![u, v, w]);
    }

    let result = min_max_weight(n, &edges, threshold);
    println!("{}", result);
}