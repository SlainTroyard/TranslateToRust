/// Translated from the C code of LeetCode Weekly Contest 425 Problem 4
/// Maintains the exact same logic and I/O format as the original C program.

use std::io::{self, BufRead};

/// A struct representing an edge in the adjacency list.
#[derive(Clone)]
struct Edge {
    to: i32,     // Destination node
    weight: i32, // Weight of the edge
    next: i32,   // Index of the next edge in the adjacency list
}

/// A struct to represent a node in the DFS stack.
#[derive(Clone)]
struct StackNode {
    node: i32,
    parent: i32,
    processed: bool,
}

/// This function sorts in descending order (equivalent to cmp_desc in C).
fn cmp_desc(a: &i64, b: &i64) -> std::cmp::Ordering {
    if *a < *b {
        std::cmp::Ordering::Greater
    } else if *a > *b {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Equal
    }
}

/// Main function to maximize the sum of weights after partial edge selection.
fn maximize_sum_of_weights(edges: &Vec<[i32; 3]>, edges_size: usize, k: i32) -> i64 {
    // Number of nodes for a tree with edges_size edges
    let n = edges_size + 1;

    // Adjacency list storage, similar to the C code (headList + edgeList).
    // head_list[node] will store the head index of the adjacency list for 'node'.
    let mut head_list = vec![-1; n];

    // We store twice as many edges because the graph is undirected (u->v, v->u).
    let mut edge_list = Vec::with_capacity(2 * edges_size);
    let mut edge_count = 0;

    // dp0_arr[node] and dp1_arr[node] store the DP values from the C program.
    let mut dp0_arr = vec![0i64; n];
    let mut dp1_arr = vec![0i64; n];

    // Adds an undirected edge (u <-> v) to the adjacency structure.
    // This mimics add_edge(u, v, w) in the C code.
    let mut add_edge = |u: i32, v: i32, w: i32| {
        // u->v edge
        edge_list.push(Edge {
            to: v,
            weight: w,
            next: head_list[u as usize],
        });
        head_list[u as usize] = edge_count as i32;
        edge_count += 1;

        // v->u edge
        edge_list.push(Edge {
            to: u,
            weight: w,
            next: head_list[v as usize],
        });
        head_list[v as usize] = edge_count as i32;
        edge_count += 1;
    };

    // Build the adjacency list (same as the C code loop calling add_edge).
    for e in edges {
        let (u, v, w) = (e[0], e[1], e[2]);
        add_edge(u, v, w);
    }

    // Stack for iterative DFS (similar to the C code's stack).
    let mut stack = Vec::new();
    // Push the root (0) with parent = -1 and processed = false.
    stack.push(StackNode {
        node: 0,
        parent: -1,
        processed: false,
    });

    // Iterative DFS approach exactly matching the C code.
    while let Some(mut current) = stack.pop() {
        if !current.processed {
            // Mark the current node as processed, and push it back on stack
            // so we process it after its children.
            current.processed = true;
            stack.push(current.clone());

            // Push children (all adjacent nodes != parent).
            let mut edge_idx = head_list[current.node as usize];
            while edge_idx != -1 {
                let e = &edge_list[edge_idx as usize];
                let child = e.to;
                if child != current.parent {
                    stack.push(StackNode {
                        node: child,
                        parent: current.node,
                        processed: false,
                    });
                }
                edge_idx = e.next;
            }
        } else {
            // Now we process the node after its children have been processed.
            let node = current.node;
            let parent = current.parent;

            // Count children
            let mut children_count = 0;
            let mut edge_idx = head_list[node as usize];
            while edge_idx != -1 {
                let e = &edge_list[edge_idx as usize];
                if e.to != parent {
                    children_count += 1;
                }
                edge_idx = e.next;
            }

            // Prepare vector for gains
            let mut gains = Vec::with_capacity(children_count);
            let mut sum_dp0: i64 = 0;

            // Collect gains = weight + dp1(child) - dp0(child); sum_dp0 accumulates dp0(child).
            edge_idx = head_list[node as usize];
            while edge_idx != -1 {
                let e = &edge_list[edge_idx as usize];
                let child = e.to;
                let weight = e.weight;
                if child != parent {
                    gains.push(weight as i64 + dp1_arr[child as usize] - dp0_arr[child as usize]);
                    sum_dp0 += dp0_arr[child as usize];
                }
                edge_idx = e.next;
            }

            // Sort gains in descending order
            gains.sort_by(cmp_desc);

            // dp0[node]: can pick up to k edges among children if they're profitable
            let mut sum0 = sum_dp0;
            if k > 0 {
                for (i, g) in gains.iter().enumerate() {
                    if i as i32 >= k {
                        break;
                    }
                    if *g > 0 {
                        sum0 += *g;
                    } else {
                        break;
                    }
                }
            }
            dp0_arr[node as usize] = sum0;

            // dp1[node]: can pick up to (k - 1) edges among children
            // If k - 1 < 0, it means no edges can be picked, so dp1[node] = 0.
            if k - 1 < 0 {
                dp1_arr[node as usize] = 0;
            } else {
                let mut sum1 = sum_dp0;
                for (i, g) in gains.iter().enumerate() {
                    if i as i32 >= (k - 1) {
                        break;
                    }
                    if *g > 0 {
                        sum1 += *g;
                    } else {
                        break;
                    }
                }
                dp1_arr[node as usize] = sum1;
            }
        }
    }

    // The final answer is dp0 on the root node (node 0).
    dp0_arr[0]
}

fn main() {
    // We will read exactly the same way the C code does:
    // First line: n k
    // Then n-1 lines each with (u, v, w)
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and k
    let line = lines.next().unwrap().unwrap();
    let mut tokens = line.split_whitespace();
    let n: i32 = tokens.next().unwrap().parse().unwrap();
    let k: i32 = tokens.next().unwrap().parse().unwrap();

    // edgesSize = n - 1
    let edges_size = (n - 1) as usize;

    // Read edges
    let mut edges = Vec::with_capacity(edges_size);
    for _ in 0..edges_size {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let u: i32 = parts.next().unwrap().parse().unwrap();
        let v: i32 = parts.next().unwrap().parse().unwrap();
        let w: i32 = parts.next().unwrap().parse().unwrap();
        edges.push([u, v, w]);
    }

    // Call the function to get the result
    let result = maximize_sum_of_weights(&edges, edges_size, k);

    // Print the result
    println!("{}", result);
}