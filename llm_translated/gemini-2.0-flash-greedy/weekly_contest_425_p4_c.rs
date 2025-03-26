use std::cmp::Ordering;
use std::io;
use std::io::BufRead;
use std::process::exit;

// Define the maximum number of nodes and edges based on constraints
const MAX_NODES: usize = 100005;
const MAX_EDGES: usize = 200010;
const MAX_STACK: usize = 100005;

// Structure to represent an edge in the adjacency list
#[derive(Clone, Copy)]
struct Edge {
    to: usize,     // Destination node
    weight: i32, // Weight of the edge
    next: i32,   // Index of the next edge in the adjacency list
}

// Structure to represent a node in the DFS stack
#[derive(Clone, Copy)]
struct StackNode {
    node: usize,      // Current node
    parent: i32,    // Parent of the current node
    processed: bool, // Flag to indicate if the node has been processed
}

// Global variables for the graph and dynamic programming arrays
static mut EDGE_LIST: [Edge; MAX_EDGES] = [Edge { to: 0, weight: 0, next: 0 }; MAX_EDGES];
static mut HEAD_LIST: [i32; MAX_NODES] = [-1; MAX_NODES];
static mut EDGE_COUNT: usize = 0;

static mut DP0_ARR: [i64; MAX_NODES] = [0; MAX_NODES];
static mut DP1_ARR: [i64; MAX_NODES] = [0; MAX_NODES];

// Comparator function for qsort to sort in descending order
fn cmp_desc(a: &i64, b: &i64) -> Ordering {
    if a < b {
        Ordering::Greater
    } else if a > b {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

/**
 * Function to add an undirected edge to the graph
 */
fn add_edge(u: usize, v: usize, w: i32) {
    unsafe {
        // Add edge u -> v
        EDGE_LIST[EDGE_COUNT].to = v;
        EDGE_LIST[EDGE_COUNT].weight = w;
        EDGE_LIST[EDGE_COUNT].next = HEAD_LIST[u];
        HEAD_LIST[u] = EDGE_COUNT as i32;
        EDGE_COUNT += 1;

        // Add edge v -> u
        EDGE_LIST[EDGE_COUNT].to = u;
        EDGE_LIST[EDGE_COUNT].weight = w;
        EDGE_LIST[EDGE_COUNT].next = HEAD_LIST[v];
        HEAD_LIST[v] = EDGE_COUNT as i32;
        EDGE_COUNT += 1;
    }
}

/**
 * Main function to maximize the sum of weights after edge removals
 */
fn maximize_sum_of_weights(edges: &Vec<Vec<i32>>, k: i32) -> i64 {
    // Calculate the number of nodes
    let n = edges.len() + 1;

    unsafe {
        // Initialize the adjacency list
        for i in 0..n {
            HEAD_LIST[i] = -1;
        }
        EDGE_COUNT = 0;

        // Build the adjacency list
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            add_edge(u, v, w);
        }

        // Initialize the DFS stack
        let mut stack: [StackNode; MAX_STACK] = [StackNode { node: 0, parent: 0, processed: false }; MAX_STACK];
        let mut top: usize = 0;

        // Push the root node (assume node 0) onto the stack
        stack[top].node = 0;
        stack[top].parent = -1; // No parent for root
        stack[top].processed = false;
        top += 1;

        // Iterative DFS
        while top > 0 {
            top -= 1;
            let current = stack[top];
            let node = current.node;
            let parent = current.parent;

            if !current.processed {
                // Push the node back onto the stack as processed
                stack[top].node = node;
                stack[top].parent = parent;
                stack[top].processed = true;
                top += 1;

                // Push all children onto the stack
                let mut edge_idx = HEAD_LIST[node];
                while edge_idx != -1 {
                    let child = EDGE_LIST[edge_idx as usize].to;
                    if child as i32 != parent {
                        stack[top].node = child;
                        stack[top].parent = node as i32;
                        stack[top].processed = false;
                        top += 1;
                    }
                    edge_idx = EDGE_LIST[edge_idx as usize].next;
                }
            } else {
                // Processing the node after its children have been processed
                let mut children_count = 0;
                let mut edge_idx = HEAD_LIST[node];

                // First, count the number of children
                while edge_idx != -1 {
                    let child = EDGE_LIST[edge_idx as usize].to;
                    if child as i32 != parent {
                        children_count += 1;
                    }
                    edge_idx = EDGE_LIST[edge_idx as usize].next;
                }

                // Allocate memory for gains
                let mut gains: Vec<i64> = vec![0; children_count];
                let mut idx = 0;
                edge_idx = HEAD_LIST[node];
                let mut sum_dp0: i64 = 0;

                // Calculate gains and sum of dp0 for all children
                while edge_idx != -1 {
                    let child = EDGE_LIST[edge_idx as usize].to;
                    let weight = EDGE_LIST[edge_idx as usize].weight;
                    if child as i32 != parent {
                        gains[idx] = weight as i64 + DP1_ARR[child] - DP0_ARR[child];
                        sum_dp0 += DP0_ARR[child];
                        idx += 1;
                    }
                    edge_idx = EDGE_LIST[edge_idx as usize].next;
                }

                // Sort the gains in descending order
                gains.sort_by(cmp_desc);

                // Calculate dp0[node]: can pick up to k edges
                let mut sum0 = sum_dp0;
                for i in 0..children_count.min(k as usize) {
                    if gains[i] > 0 {
                        sum0 += gains[i];
                    } else {
                        break;
                    }
                }
                DP0_ARR[node] = sum0;

                // Calculate dp1[node]: can pick up to (k-1) edges
                if k - 1 < 0 {
                    DP1_ARR[node] = 0;
                } else {
                    let mut sum1 = sum_dp0;
                    for i in 0..children_count.min((k - 1) as usize) {
                        if gains[i] > 0 {
                            sum1 += gains[i];
                        } else {
                            break;
                        }
                    }
                    DP1_ARR[node] = sum1;
                }
            }
        }

        // The answer is dp0 for the root node
        DP0_ARR[0]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of nodes and the allowed number of edges to select
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let n: usize = first_line_iter.next().unwrap().parse().unwrap();
    let k: i32 = first_line_iter.next().unwrap().parse().unwrap();

    // Number of edges is n - 1 for a tree
    let edges_size = n - 1;

    // Allocate memory for edges
    let mut edges: Vec<Vec<i32>> = Vec::with_capacity(edges_size);

    // Read the edges, each as a triplet (u, v, w)
    for _ in 0..edges_size {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let u: i32 = iter.next().unwrap().parse().unwrap();
        let v: i32 = iter.next().unwrap().parse().unwrap();
        let w: i32 = iter.next().unwrap().parse().unwrap();
        edges.push(vec![u, v, w]);
    }

    // Call maximizeSumOfWeights function and get the result
    let result = maximize_sum_of_weights(&edges, k);

    // Print the result
    println!("{}", result);
}