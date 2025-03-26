use std::io;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    fn min_max_weight(&self, n: usize, edges: Vec<Vec<i32>>, threshold: i32) -> i32 {
        // Compute the maximum edge weight
        let mx = edges.iter().map(|e| e[2]).max().unwrap_or(0);

        // Check if even the maximum weight allows all nodes to be reached
        if !can_reach_all(n, &edges, mx) {
            return -1;
        }

        // Binary search between 0 and mx
        let mut head = 0;
        let mut tail = mx;
        while head < tail {
            let mid = (head + tail) >> 1;
            if can_reach_all(n, &edges, mid) {
                tail = mid;
            } else {
                head = mid + 1;
            }
        }

        head as i32
    }
}

fn can_reach_all(n: usize, edges: &Vec<Vec<i32>>, limit: i32) -> bool {
    // Build adjacency list
    let mut adj = vec![Vec::new(); n];
    for edge in edges {
        if edge[2] <= limit {
            let u = edge[1] as usize;
            let v = edge[0] as usize;
            adj[u].push(v);
        }
    }

    // BFS
    let mut visited = vec![false; n];
    let mut q = VecDeque::new();
    q.push_back(0);
    visited[0] = true;

    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        for &v in &adj[u] {
            if !visited[v] {
                visited[v] = true;
                q.push_back(v);
            }
        }
    }

    visited.iter().all(|&x| x)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let mut ptr = 0;
    let n = tokens[ptr].parse::<usize>().unwrap();
    ptr += 1;
    let threshold = tokens[ptr].parse::<i32>().unwrap();
    ptr += 1;

    let mut edges = Vec::new();
    for _ in 0..n {
        let u = tokens[ptr].parse::<i32>().unwrap();
        ptr += 1;
        let v = tokens[ptr].parse::<i32>().unwrap();
        ptr += 1;
        let w = tokens[ptr].parse::<i32>().unwrap();
        ptr += 1;
        edges.push(vec![u, v, w]);
    }

    let solution = Solution;
    println!("{}", solution.min_max_weight(n, edges, threshold));
}