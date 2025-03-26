use std::io;

#[derive(Clone)]
struct Edge {
    to: usize,
    weight: i32,
}

#[derive(Clone)]
struct StackNode {
    node: usize,
    parent: isize,
    processed: bool,
}

fn maximize_sum_of_weights(edges: &[(usize, usize, i32)], k: usize) -> i64 {
    let n = edges.len() + 1;
    let mut adj = vec![Vec::new(); n];

    for &(u, v, w) in edges {
        adj[u].push(Edge { to: v, weight: w });
        adj[v].push(Edge { to: u, weight: w });
    }

    let mut dp0 = vec![0i64; n];
    let mut dp1 = vec![0i64; n];
    let mut stack = Vec::new();

    stack.push(StackNode {
        node: 0,
        parent: -1,
        processed: false,
    });

    while let Some(current) = stack.pop() {
        if !current.processed {
            stack.push(StackNode {
                node: current.node,
                parent: current.parent,
                processed: true,
            });

            for edge in &adj[current.node] {
                let child = edge.to;
                if (child as isize) != current.parent {
                    stack.push(StackNode {
                        node: child,
                        parent: current.node as isize,
                        processed: false,
                    });
                }
            }
        } else {
            let mut children_count = 0;
            let mut sum_dp0 = 0;
            let mut gains = Vec::new();

            for edge in &adj[current.node] {
                let child = edge.to;
                if (child as isize) != current.parent {
                    let w = edge.weight as i64;
                    let gain = w + dp1[child] - dp0[child];
                    gains.push(gain);
                    sum_dp0 += dp0[child];
                    children_count += 1;
                }
            }

            gains.sort_unstable_by(|a, b| b.cmp(a));

            let mut sum0 = sum_dp0;
            for &gain in gains.iter().take(k) {
                if gain > 0 {
                    sum0 += gain;
                } else {
                    break;
                }
            }
            dp0[current.node] = sum0;

            let allowed = k.saturating_sub(1);
            if allowed == 0 {
                dp1[current.node] = sum_dp0;
            } else {
                let mut sum1 = sum_dp0;
                for &gain in gains.iter().take(allowed) {
                    if gain > 0 {
                        sum1 += gain;
                    } else {
                        break;
                    }
                }
                dp1[current.node] = sum1;
            }
        }
    }

    dp0[0]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    let mut edges = Vec::new();
    for _ in 0..(n - 1) {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let nums: Vec<usize> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let u = nums[0];
        let v = nums[1];
        let w = nums[2] as i32;
        edges.push((u, v, w));
    }

    let result = maximize_sum_of_weights(&edges, k);
    println!("{}", result);
}