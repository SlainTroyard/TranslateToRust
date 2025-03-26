use std::cmp::Ordering;
use std::io::{self, BufRead};

// Define the maximum number of nodes and edges based on constraints
const MAX_NODES: usize = 100005;
const MAX_EDGES: usize = 200010;
const MAX_STACK: usize = 100005;

// Structure to represent an edge in the adjacency list
struct Edge {
    to: usize,     // Destination node
    weight: i32,   // Weight of the edge
    next: i32,     // Index of the next edge in the adjacency list
}

// Structure to represent a node in the DFS stack
struct StackNode {
    node: usize,      // Current node
    parent: i32,      // Parent of the current node
    processed: bool,  // Flag to indicate if the node has been processed
}

// Main function to maximize the sum of weights after edge removals
fn maximize_sum_of_weights(edges: &[Vec<i32>], k: i32) -> i64 {
    // Calculate the number of nodes
    let n = edges.len() + 1;
    
    // Initialize the adjacency list
    let mut head_list = vec![-1; MAX_NODES];
    let mut edge_list = vec![Edge { to: 0, weight: 0, next: -1 }; MAX_EDGES];
    let mut edge_count = 0;
    
    // Initialize DP arrays
    let mut dp0_arr = vec![0i64; MAX_NODES];
    let mut dp1_arr = vec![0i64; MAX_NODES];
    
    // Function to add an undirected edge to the graph
    let mut add_edge = |u: usize, v: usize, w: i32| {
        // Add edge u -> v
        edge_list[edge_count] = Edge {
            to: v,
            weight: w,
            next: head_list[u],
        };
        head_list[u] = edge_count as i32;
        edge_count += 1;
        
        // Add edge v -> u
        edge_list[edge_count] = Edge {
            to: u,
            weight: w,
            next: head_list[v],
        };
        head_list[v] = edge_count as i32;
        edge_count += 1;
    };
    
    // Build the adjacency list
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        let w = edge[2];
        add_edge(u, v, w);
    }
    
    // Initialize the DFS stack
    let mut stack = vec![StackNode {
        node: 0,
        parent: -1, // No parent for root
        processed: false,
    }];
    
    // Iterative DFS
    while let Some(current) = stack.pop() {
        let node = current.node;
        let parent = current.parent;
        
        if !current.processed {
            // Push the node back onto the stack as processed
            stack.push(StackNode {
                node,
                parent,
                processed: true,
            });
            
            // Push all children onto the stack
            let mut edge_idx = head_list[node];
            while edge_idx != -1 {
                let child = edge_list[edge_idx as usize].to;
                if child as i32 != parent {
                    stack.push(StackNode {
                        node: child,
                        parent: node as i32,
                        processed: false,
                    });
                }
                edge_idx = edge_list[edge_idx as usize].next;
            }
        } else {
            // Processing the node after its children have been processed
            let mut children_count = 0;
            let mut edge_idx = head_list[node];
            
            // First, count the number of children
            while edge_idx != -1 {
                let child = edge_list[edge_idx as usize].to;
                if child as i32 != parent {
                    children_count += 1;
                }
                edge_idx = edge_list[edge_idx as usize].next;
            }
            
            // Prepare gains vector
            let mut gains = Vec::with_capacity(children_count);
            edge_idx = head_list[node];
            let mut sum_dp0 = 0i64;
            
            // Calculate gains and sum of dp0 for all children
            while edge_idx != -1 {
                let child = edge_list[edge_idx as usize].to;
                let weight = edge_list[edge_idx as usize].weight;
                if child as i32 != parent {
                    gains.push(i64::from(weight) + dp1_arr[child] - dp0_arr[child]);
                    sum_dp0 += dp0_arr[child];
                }
                edge_idx = edge_list[edge_idx as usize].next;
            }
            
            // Sort the gains in descending order
            gains.sort_by(|a, b| b.cmp(a));
            
            // Calculate dp0[node]: can pick up to k edges
            let mut sum0 = sum_dp0;
            for i in 0..gains.len().min(k as usize) {
                if gains[i] > 0 {
                    sum0 += gains[i];
                } else {
                    break;
                }
            }
            dp0_arr[node] = sum0;
            
            // Calculate dp1[node]: can pick up to (k-1) edges
            if k - 1 < 0 {
                dp1_arr[node] = 0;
            } else {
                let mut sum1 = sum_dp0;
                for i in 0..gains.len().min((k - 1) as usize) {
                    if gains[i] > 0 {
                        sum1 += gains[i];
                    } else {
                        break;
                    }
                }
                dp1_arr[node] = sum1;
            }
        }
    }
    
    // The answer is dp0 for the root node
    dp0_arr[0]
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line to get n and k
    let line = lines.next().unwrap()?;
    let parts: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    
    let n = parts[0];
    let k = parts[1];
    
    // Number of edges is n - 1 for a tree
    let edges_size = n - 1;
    
    // Read the edges
    let mut edges = Vec::with_capacity(edges_size as usize);
    for _ in 0..edges_size {
        let line = lines.next().unwrap()?;
        let edge: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        edges.push(edge);
    }
    
    // Call maximize_sum_of_weights function and get the result
    let result = maximize_sum_of_weights(&edges, k);
    
    // Print the result
    println!("{}", result);
    
    Ok(())
}