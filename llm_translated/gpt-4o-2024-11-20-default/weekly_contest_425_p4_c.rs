use std::collections::VecDeque;
use std::cmp::Reverse;

// Define constants for maximum nodes and edges based on constraints
const MAX_NODES: usize = 100_005;
const MAX_EDGES: usize = 200_010;

// Structure to represent an edge in the adjacency list
#[derive(Clone, Copy)]
struct Edge {
    to: usize,     // Destination node
    weight: i64,   // Weight of the edge
    next: Option<usize>, // Index of the next edge in the adjacency list
}

fn add_edge(edges: &mut Vec<Edge>, head_list: &mut [Option<usize>], u: usize, v: usize, w: i64) {
    // Add edge u -> v
    edges.push(Edge {
        to: v,
        weight: w,
        next: head_list[u],
    });
    head_list[u] = Some(edges.len() - 1);

    // Add edge v -> u
    edges.push(Edge {
        to: u,
        weight: w,
        next: head_list[v],
    });
    head_list[v] = Some(edges.len() - 1);
}

fn maximize_sum_of_weights(edges_input: &[(usize, usize, i64)], k: usize) -> i64 {
    let n = edges_input.len() + 1;

    // Initialize adjacency list
    let mut head_list = vec![None; n];
    let mut edges = Vec::with_capacity(MAX_EDGES);

    // Build the adjacency list
    for &(u, v, w) in edges_input {
        add_edge(&mut edges, &mut head_list, u, v, w);
    }

    // Initialize DP arrays
    let mut dp0_arr = vec![0i64; n];
    let mut dp1_arr = vec![0i64; n];

    // Perform iterative DFS using a stack
    let mut stack: VecDeque<(usize, Option<usize>, bool)> = VecDeque::new();

    // Push the root node (assume node 0) onto the stack
    stack.push_back((0, None, false));

    while let Some((node, parent, processed)) = stack.pop_back() {
        if !processed {
            // Push the node back onto the stack as processed
            stack.push_back((node, parent, true));

            // Push all children onto the stack
            let mut edge_idx = head_list[node];
            while let Some(e_idx) = edge_idx {
                let edge = edges[e_idx];
                let child = edge.to;
                if Some(child) != parent {
                    stack.push_back((child, Some(node), false));
                }
                edge_idx = edge.next;
            }
        } else {
            // Processing the node after its children have been processed
            let mut children = Vec::new();
            let mut edge_idx = head_list[node];

            // Find all children
            while let Some(e_idx) = edge_idx {
                let edge = edges[e_idx];
                let child = edge.to;
                let weight = edge.weight;
                if Some(child) != parent {
                    let gain = weight + dp1_arr[child] - dp0_arr[child];
                    children.push(gain);
                }
                edge_idx = edge.next;
            }

            children.sort_by_key(|&gain| Reverse(gain));

            // Calculate dp0[node]: can pick up to k edges
            let sum_dp0: i64 = children.iter().map(|_| dp0_arr[node]);
            let sum0 ;
            for i =
            dp0_arr[node] = sum0;

            // Calculate dp1[node]: can pick up to (k-1) edges
            let sum1 ;
-------