use std::io::{self, BufRead};
use std::cmp::Ordering;

// Define constants for maximum nodes and edges
const MAX_NODES: usize = 100_005;
const MAX_EDGES: usize = 200_010;

// Structure to represent an edge in the adjacency list
#[derive(Clone, Copy)]
struct Edge {
    to: usize,     // Destination node
    weight: i64,   // Weight of the edge
    next: Option<usize>, // Index of the next edge in the adjacency list
}

// Global variables for the graph and dynamic programming arrays
static mut EDGE_LIST: [Edge; MAX_EDGES] = [Edge { to: 0, weight: 0, next: None }; MAX_EDGES];
static mut HEAD_LIST: [Option<usize>; MAX_NODES] = [None; MAX_NODES];
static mut EDGE_COUNT: usize = 0;

static mut DP0_ARR: [i64; MAX_NODES] = [0; MAX_NODES];
static mut DP1_ARR: [i64; MAX_NODES] = [0; MAX_NODES];

/// Function to add an undirected edge to the graph
unsafe fn add_edge(u: usize, v: usize, w: i64) {
    // Add edge u -> v
    EDGE_LIST[EDGE_COUNT] = Edge { to: v, weight: w, next: HEAD_LIST[u] };
    HEAD_LIST[u] = Some(EDGE_COUNT);
    EDGE_COUNT += 1;

    // Add edge v -> u
    EDGE_LIST[EDGE_COUNT] = Edge { to: u, weight: w, next: HEAD_LIST[v] };
    HEAD_LIST[v] = Some(EDGE_COUNT);
    EDGE_COUNT += 1;
}

/// Main function to maximize the sum of weights after edge removals
unsafe fn maximize_sum_of_weights(edges: &Vec<(usize, usize, i64)>, n: usize, k: usize) -> i64 {
    // Initialize the adjacency list
    for i in 0..n {
        HEAD_LIST[i] = None;
    }
    EDGE_COUNT = 0;

    // Build the adjacency list
    for &(u, v, w) in edges {
        add_edge(u, v, w);
    }

    // Stack for iterative DFS
    let mut stack = Vec::new();
    stack.push((0, None, false)); // (node, parent, processed)

    while let Some((node, parent, processed)) = stack.pop() {
        if !processed {
            // Push the node back onto the stack as processed
            stack.push((node, parent, true));

            // Push all children onto the stack
            let mut edge_idx = HEAD_LIST[node];
            while let Some(idx) = edge_idx {
                let child = EDGE_LIST[idx].to;
                if Some(child) != parent {
                    stack.push((child, Some(node), false));
                }
                edge_idx = EDGE_LIST[idx].next;
            }
        } else {
            // Processing the node after its children have been processed
            let mut children = Vec::new();
            let mut edge_idx = HEAD_LIST[node];
            let mut sum_dp0 = 0;

            // Collect children and calculate sum of dp0 for all children
            while let Some(idx) = edge_idx {
                let child = EDGE_LIST[idx].to;
                let weight = EDGE_LIST[idx].weight;
                if Some(child) != parent {
                    children.push((child, weight));
                    sum_dp0 += DP0_ARR[child];
                }
                edge_idx = EDGE_LIST[idx].next;
            }

            // Calculate gains
            let mut gains: Vec<i64> = children.iter()
                .map(|&(child, weight)| weight + DP1_ARR[child] - DP0_ARR[child])
                .collect();

            // Sort gains in descending order
            gains.sort_by(|a, b| b.cmp(a));

            // Calculate dp0[node]: can pick up to k edges
            let mut sum0 = sum_dp0;
            for i in 0..gains.len().min(k) {
                if gains[i] > 0 {
                    sum0 += gains[i];
                } else {
                    break;
                }
            }
            DP0_ARR[node] = sum0;

            // Calculate dp1[node]: can pick up to (k-1) edges
            if k > 0 {
                let mut sum1 = sum_dp0;
                for i in 0..gains.len().min(k - 1) {
                    if gains[i] > 0 {
                        sum1 += gains[i];
                    } else {
                        break;
                    }
                }
                DP1_ARR[node] = sum1;
            } else {
                DP1_ARR[node] = 0;
            }
        }
    }

    // The answer is dp0 for the root node
    DP0_ARR[0]
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of nodes and the allowed number of edges to select
    let first_line = lines.next().unwrap().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let n: usize = first_iter.next().unwrap().parse().unwrap();
    let k: usize = first_iter.next().unwrap().parse().unwrap();

    // Read the edges
    let mut edges = Vec::new();
    for _ in 0..(n - 1) {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let u: usize = iter.next().unwrap().parse().unwrap();
        let v: usize = iter.next().unwrap().parse().unwrap();
        let w: i64 = iter.next().unwrap().parse().unwrap();
        edges.push((u, v, w));
    }

    // Call maximize_sum_of_weights function and get the result
    let result = unsafe { maximize_sum_of_weights(&edges, n, k) };

    // Print the result
    println!("{}", result);
}