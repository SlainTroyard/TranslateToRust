use std::error::Error;
use std::io::{self, BufRead, BufWriter, Write};

/// Structure representing an edge in the adjacency list.
struct Edge {
    to: usize,           // Destination node.
    weight: i32,         // Weight of the edge.
    next: Option<usize>, // Index of the next edge in the adjacency list.
}

/// Structure representing a node entry in the DFS stack.
struct StackNode {
    node: usize,         // Current node.
    parent: Option<usize>, // Parent of the current node.
    processed: bool,     // Flag to indicate if the node has been processed.
}

/// Function to maximize the sum of weights after optionally picking up to k edges per node.
///
/// It builds an undirected graph using an adjacency list, then performs an iterative DFS
/// using a stack and dynamic programming. The dp0[node] represents the best sum if you can pick
/// up to k edges from this node, while dp1[node] represents the best sum if the parent has already used one edge slot.
fn maximize_sum_of_weights(edges_input: &[[i32; 3]], k: i32) -> i64 {
    // Calculate the number of nodes.
    let n = edges_input.len() + 1;

    // Initialize the adjacency list head pointer for each node.
    let mut head_list = vec![None; n];
    // Pre-allocate vector for edges. There are 2 edges per undirected edge.
    let mut edge_list = Vec::with_capacity(2 * edges_input.len());

    // Helper function to add an undirected edge to the graph.
    let mut add_edge = |u: usize, v: usize, w: i32| {
        // Add edge u -> v.
        edge_list.push(Edge {
            to: v,
            weight: w,
            next: head_list[u],
        });
        head_list[u] = Some(edge_list.len() - 1);

        // Add edge v -> u.
        edge_list.push(Edge {
            to: u,
            weight: w,
            next: head_list[v],
        });
        head_list[v] = Some(edge_list.len() - 1);
    };

    // Build the graph by adding each edge.
    for e in edges_input.iter() {
        // Convert endpoints to usize.
        let u = e[0] as usize;
        let v = e[1] as usize;
        let w = e[2];
        add_edge(u, v, w);
    }

    // DP arrays:
    // dp0_arr[node] = best sum obtainable from subtree rooted at node when picking up to k edges.
    // dp1_arr[node] = best sum if the parent already selected an edge towards this node (k-1 remaining).
    let mut dp0_arr = vec![0i64; n];
    let mut dp1_arr = vec![0i64; n];

    // Use iterative DFS using a stack.
    let mut stack = Vec::with_capacity(n);
    // Start from root node 0; it has no parent.
    stack.push(StackNode {
        node: 0,
        parent: None,
        processed: false,
    });

    while let Some(current) = stack.pop() {
        let node = current.node;
        let parent = current.parent;

        if !current.processed {
            // First visit: mark the node to be processed after its children.
            stack.push(StackNode {
                node,
                parent,
                processed: true,
            });

            // Push all children to the stack.
            let mut edge_idx_opt = head_list[node];
            while let Some(edge_idx) = edge_idx_opt {
                let child = edge_list[edge_idx].to;
                if Some(child) != parent {
                    stack.push(StackNode {
                        node: child,
                        parent: Some(node),
                        processed: false,
                    });
                }
                edge_idx_opt = edge_list[edge_idx].next;
            }
        } else {
            // Process the node after all its children have been processed.
            // First, count the number of children and compute the sum of dp0 for each child.
            let mut children_count = 0;
            let mut edge_idx_opt = head_list[node];
            while let Some(edge_idx) = edge_idx_opt {
                let child = edge_list[edge_idx].to;
                if Some(child) != parent {
                    children_count += 1;
                }
                edge_idx_opt = edge_list[edge_idx].next;
            }

            // Allocate a vector to store gains from each child.
            let mut gains = Vec::with_capacity(children_count);
            let mut sum_dp0 = 0i64;

            // Re-iterate over children to compute gains and accumulate sum_dp0.
            let mut edge_idx_opt = head_list[node];
            while let Some(edge_idx) = edge_idx_opt {
                let edge = &edge_list[edge_idx];
                let child = edge.to;
                if Some(child) != parent {
                    // Gain is computed as weight + (dp1[child] - dp0[child]).
                    gains.push((edge.weight as i64) + dp1_arr[child] - dp0_arr[child]);
                    sum_dp0 += dp0_arr[child];
                }
                edge_idx_opt = edge.next;
            }

            // Sort the gains in descending order.
            gains.sort_by(|a, b| b.cmp(a));

            // Calculate dp0[node]: can pick up to k edges.
            let mut sum0 = sum_dp0;
            for (i, &gain) in gains.iter().enumerate() {
                if i < (k as usize) {
                    if gain > 0 {
                        sum0 += gain;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            dp0_arr[node] = sum0;

            // Calculate dp1[node]: can pick up to (k-1) edges.
            if k - 1 < 0 {
                dp1_arr[node] = 0;
            } else {
                let mut sum1 = sum_dp0;
                for (i, &gain) in gains.iter().enumerate() {
                    if i < ((k - 1) as usize) {
                        if gain > 0 {
                            sum1 += gain;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                dp1_arr[node] = sum1;
            }
        }
    }

    // The answer is dp0 for the root node.
    dp0_arr[0]
}

fn main() -> Result<(), Box<dyn Error>> {
    // Use buffered IO for efficient reading and writing.
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    // Read the first line containing two integers: n (number of nodes) and k (allowed number of selected edges).
    let mut input_line = String::new();
    reader.read_line(&mut input_line)?;
    let mut parts = input_line.split_whitespace();
    let n: usize = parts.next().ok_or("Missing n")?.parse()?;
    let k: i32 = parts.next().ok_or("Missing k")?.parse()?;

    // For a tree, the number of edges is n - 1.
    let edges_size = n - 1;

    // Read the edges, each as a triplet (u, v, w).
    let mut edges = Vec::with_capacity(edges_size);
    for _ in 0..edges_size {
        input_line.clear();
        reader.read_line(&mut input_line)?;
        let nums: Vec<i32> = input_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if nums.len() != 3 {
            return Err("Each edge must have exactly 3 numbers".into());
        }
        edges.push([nums[0], nums[1], nums[2]]);
    }

    // Call the function to get the maximum sum of weights.
    let result = maximize_sum_of_weights(&edges, k);

    // Write the result exactly like the C code output format.
    writeln!(writer, "{}", result)?;
    writer.flush()?;
    Ok(())
}