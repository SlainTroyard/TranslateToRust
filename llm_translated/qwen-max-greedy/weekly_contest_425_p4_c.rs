use std::cmp::Ordering;
use std::io::{self, BufRead};

// Define the maximum number of nodes and edges based on constraints
const MAX_NODES: usize = 100_005;
const MAX_EDGES: usize = 200_005;

// Structure to represent an edge in the adjacency list
struct Edge {
    to: usize,     // Destination node
    weight: i32,   // Weight of the edge
    next: usize,   // Index of the next edge in the adjacency list
}

// Structure to represent a node in the DFS stack
struct StackNode {
    node: usize,       // Current node
    parent: isize,    // Parent of the current node
    processed: bool,  // Flag to indicate if the node has been processed
}

// Global variables for the graph and dynamic programming arrays
static mut EDGE_LIST: [Edge; MAX_EDGES] = [Edge { to: 0, weight: 0, next: 0 }; MAX_EDGES];
static mut HEAD_LIST: [usize; MAX_NODES] = [0; MAX_NODES];
static mut EDGE_COUNT: usize = 0;
static mut DP0_ARR: [i64; MAX_NODES] = [0; MAX_NODES];
static mut DP1_ARR: [i64; MAX_NODES] = [0; MAX_NODES];

// Comparator function for sorting in descending order
fn cmp_desc(a: &i64, b: &i64) -> Ordering {
    b.cmp(a)
}

// Function to add an undirected edge to the graph
unsafe fn add_edge(u: usize, v: usize, w: i32) {
    // Add edge u -> v
    EDGE_LIST[EDGE_COUNT].to = v;
    EDGE_LIST[EDGE_COUNT].weight = w;
    EDGE_LIST[EDGE_COUNT].next = HEAD_LIST[u];
    HEAD_LIST[u] = EDGE_COUNT;
    EDGE_COUNT += 1;

    // Add edge v -> u
    EDGE_LIST[EDGE_COUNT].to = u;
    EDGE_LIST[EDGE_COUNT].weight = w;
    EDGE_LIST[EDGE_COUNT].next = HEAD_LIST[v];
    HEAD_LIST[v] = EDGE_COUNT;
    EDGE_COUNT += 1;
}

// Main function to maximize the sum of weights after edge removals
unsafe fn maximize_sum_of_weights(edges: &Vec<(usize, usize, i32)>, k: usize) -> i64 {
    let n = edges.len() + 1;

    // Initialize the adjacency list
    for i in 0..n {
        HEAD_LIST[i] = MAX_EDGES - 1;
    }
    EDGE_COUNT = 0;

    // Build the adjacency list
    for (u, v, w) in edges.iter() {
        add_edge(*u, *v, *w);
    }

    // Initialize the DFS stack
    let mut stack: Vec<StackNode> = Vec::new();
    stack.push(StackNode { node: 0, parent: -1, processed: false });

    // Iterative DFS
    while let Some(current) = stack.pop() {
        let node = current.node;
        let parent = current.parent;

        if !current.processed {
            // Push the node back onto the stack as processed
            stack.push(StackNode { node, parent, processed: true });

            // Push all children onto the stack
            let mut edge_idx = HEAD_LIST[node];
            while edge_idx != MAX_EDGES - 1 {
                let child = EDGE_LIST[edge_idx].to;
                if child as isize != parent {
                    stack.push(StackNode { node: child, parent: node as isize, processed: false });
                }
                edge_idx = EDGE_LIST[edge_idx].next;
            }
        } else {
            // Processing the node after its children have been processed
            let mut children_count = 0;
            let mut edge_idx = HEAD_LIST[node];

            // First, count the number of children
            while edge_idx != MAX_EDGES - 1 {
                let child = EDGE_LIST[edge_idx].to;
                if child as isize != parent {
                    children_count += 1;
                }
                edge_idx = EDGE_LIST[edge_idx].next;
            }

            // Allocate memory for gains
            let mut gains: Vec<i64> = Vec::with_capacity(children_count);
            let mut sum_dp0 = 0;

            // Calculate gains and sum of dp0 for all children
            edge_idx = HEAD_LIST[node];
            while edge_idx != MAX_EDGES - 1 {
                let child = EDGE_LIST[edge_idx].to;
                let weight = EDGE_LIST[edge_idx].weight as i64;
                if child as isize != parent {
                    gains.push(weight + DP1_ARR[child] - DP0_ARR[child]);
                    sum_dp0 += DP0_ARR[child];
                }
                edge_idx = EDGE_LIST[edge_idx].next;
            }

            // Sort the gains in descending order
            gains.sort_by(cmp_desc);

            // Calculate dp0[node]: can pick up to k edges
            let mut sum0 = sum_dp0;
            for gain in gains.iter().take(k).filter(|&g| *g > 0) {
                sum0 += gain;
            }
            DP0_ARR[node] = sum0;

            // Calculate dp1[node]: can pick up to (k-1) edges
            if k == 0 {
                DP1_ARR[node] = 0;
            } else {
                let mut sum1 = sum_dp0;
                for gain in gains.iter().take(k - 1).filter(|&g| *g > 0) {
                    sum1 += gain;
                }
                DP1_ARR[node] = sum1;
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
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    // Number of edges is n - 1 for a tree
    let edges_size = n - 1;

    // Read the edges, each as a triplet (u, v, w)
    let mut edges: Vec<(usize, usize, i32)> = Vec::with_capacity(edges_size);
    for _ in 0..edges_size {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let u: usize = iter.next().unwrap().parse().unwrap();
        let v: usize = iter.next().unwrap().parse().unwrap();
        let w: i32 = iter.next().unwrap().parse().unwrap();
        edges.push((u, v, w));
    }

    // Call maximizeSumOfWeights function and get the result
    let result = unsafe { maximize_sum_of_weights(&edges, k) };

    // Print the result
    println!("{}", result);
}