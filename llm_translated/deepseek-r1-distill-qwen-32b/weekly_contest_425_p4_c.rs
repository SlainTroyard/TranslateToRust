use std::io;

struct Edge {
    to: usize,
    weight: i32,
}

struct StackNode {
    node: usize,
    parent: Option<usize>,
    processed: bool,
}

fn maximize_sum_of_weights(
    adjacency: &Vec<Vec<Edge>>,
    k: usize,
    dp0: &mut Vec<i64>,
    dp1: &mut Vec<i64>,
) -> i64 {
    let n = adjacency.len();
    let mut stack = Vec::new();
    stack.push(StackNode {
        node: 0,
        parent: None,
        processed: false,
    });

    while !stack.is_empty() {
        let mut current = stack.pop().unwrap();
        let node = current.node;
        let parent = current.parent;

        if !current.processed {
            current.processed = true;
            stack.push(current);

            for edge in &adjacency[node] {
                if edge.to != parent.unwrap_or(usize::MAX) {
                    stack.push(StackNode {
                        node: edge.to,
                        parent: Some(node),
                        processed: false,
                    });
                }
            }
        } else {
            let mut gains = Vec::new();
            let mut sum_dp0 = 0;
            for edge in &adjacency[node] {
                if let Some(p) = parent {
                    if edge.to != p {
                        let gain = (edge.weight as i64) + dp1[edge.to] - dp0[edge.to];
                        gains.push(gain);
                        sum_dp0 += dp0[edge.to];
                    }
                } else {
                    let gain = (edge.weight as i64) + dp1[edge.to] - dp0[edge.to];
                    gains.push(gain);
                    sum_dp0 += dp0[edge.to];
                }
            }

            gains.sort_by(|a, b| b.cmp(a));

            let mut sum0 = sum_dp0;
            for i in 0..gains.len() {
                if i >= k {
                    break;
                }
                if gains[i] > 0 {
                    sum0 += gains[i];
                } else {
                    break;
                }
            }
            dp0[node] = sum0;

            let limit = if k == 0 { 0 } else { k - 1 };
            let mut sum1 = sum_dp0;
            for i in 0..gains.len() {
                if i >= limit {
                    break;
                }
                if gains[i] > 0 {
                    sum1 += gains[i];
                } else {
                    break;
                }
            }
            dp1[node] = sum1;
        }
    }

    dp0[0]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    let edges_size = n - 1;
    let mut adjacency = vec![Vec::new(); n];

    for _ in 0..edges_size {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let u: usize = parts.next().unwrap().parse().unwrap();
        let v: usize = parts.next().unwrap().parse().unwrap();
        let w: i32 = parts.next().unwrap().parse().unwrap();

        adjacency[u].push(Edge { to: v, weight: w });
        adjacency[v].push(Edge { to: u, weight: w });
    }

    let mut dp0 = vec![0; n];
    let mut dp1 = vec![0; n];

    let result = maximize_sum_of_weights(&adjacency, k, &mut dp0, &mut dp1);

    println!("{}", result);
}