use std::cmp::Ordering;
use std::io;
use std::io::Read;

const MAX_NODES: usize = 100005;
const MAX_EDGES: usize = 200010;

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    weight: i32,
    next: i32,
}

fn maximize_sum_of_weights(edges_input: &Vec<Vec<i32>>, k: i32) -> i64 {
    let edges_size = edges_input.len();
    let n = edges_size + 1;

    let mut edge_list: [Edge; MAX_EDGES] = [Edge { to: 0, weight: 0, next: 0 }; MAX_EDGES];
    let mut head_list: [i32; MAX_NODES] = [-1; MAX_NODES];
    let mut edge_count: usize = 0;

    let mut dp0_arr: [i64; MAX_NODES] = [0; MAX_NODES];
    let mut dp1_arr: [i64; MAX_NODES] = [0; MAX_NODES];

    fn add_edge(u: usize, v: usize, w: i32, edge_list: &mut [Edge; MAX_EDGES], head_list: &mut [i32; MAX_NODES], edge_count: &mut usize) {
        // Add edge u -> v
        edge_list[*edge_count].to = v;
        edge_list[*edge_count].weight = w;
        edge_list[*edge_count].next = head_list[u];
        head_list[u] = *edge_count as i32;
        *edge_count += 1;

        // Add edge v -> u
        edge_list[*edge_count].to = u;
        edge_list[*edge_count].weight = w;
        edge_list[*edge_count].next = head_list[v];
        head_list[v] = *edge_count as i32;
        *edge_count += 1;
    }

    // Build the adjacency list
    for i in 0..edges_size {
        let u = edges_input[i][0] as usize;
        let v = edges_input[i][1] as usize;
        let w = edges_input[i][2];
        add_edge(u, v, w, &mut edge_list, &mut head_list, &mut edge_count);
    }

    // Iterative DFS
    let mut stack: Vec<(usize, i32, bool)> = Vec::new(); // (node, parent, processed)
    stack.push((0, -1, false));

    while let Some((node, parent, processed)) = stack.pop() {
        if !processed {
            // Push the node back onto the stack as processed
            stack.push((node, parent, true));

            // Push all children onto the stack
            let mut edge_idx = head_list[node];
            while edge_idx != -1 {
                let child = edge_list[edge_idx as usize].to;
                if child as i32 != parent {
                    stack.push((child, node as i32, false));
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

            // Calculate gains and sum of dp0 for all children
            let mut gains: Vec<i64> = Vec::with_capacity(children_count);
            let mut idx = 0;
            edge_idx = head_list[node];
            let mut sum_dp0: i64 = 0;

            while edge_idx != -1 {
                let child = edge_list[edge_idx as usize].to;
                let weight = edge_list[edge_idx as usize].weight;
                if child as i32 != parent {
                    gains.push(weight as i64 + dp1_arr[child] - dp0_arr[child]);
                    sum_dp0 += dp0_arr[child];
                    idx += 1;
                }
                edge_idx = edge_list[edge_idx as usize].next;
            }

            // Sort the gains in descending order
            gains.sort_by(|a, b| b.cmp(a));

            // Calculate dp0[node]: can pick up to k edges
            let mut sum0 = sum_dp0;
            for i in 0..children_count.min(k as usize) {
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
                for i in 0..children_count.min((k - 1) as usize) {
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut split = first_line.split_whitespace();
    let n: usize = split.next().unwrap().parse().unwrap();
    let k: i32 = split.next().unwrap().parse().unwrap();

    let edges_size = n - 1;
    let mut edges: Vec<Vec<i32>> = Vec::with_capacity(edges_size);

    for _ in 0..edges_size {
        let line = lines.next().unwrap();
        let mut split = line.split_whitespace();
        let u: i32 = split.next().unwrap().parse().unwrap();
        let v: i32 = split.next().unwrap().parse().unwrap();
        let w: i32 = split.next().unwrap().parse().unwrap();
        edges.push(vec![u as i32, v as i32, w as i32]);
    }

    let result = maximize_sum_of_weights(&edges, k);

    println!("{}", result);

    Ok(())
}