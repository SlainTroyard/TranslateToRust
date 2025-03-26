use std::collections::VecDeque;
use std::io;

struct Solution;

impl Solution {
    pub fn min_max_weight(n: i32, edges: Vec<Vec<i32>>, threshold: i32) -> i32 {
        // Closure to check connectivity with edges having weight <= lim
        let check = |lim: i32| -> bool {
            let n_usize = n as usize;
            let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n_usize]; // Adjacency list
            for edge in edges.iter() {
                if edge[2] <= lim {
                    let v = edge[1] as usize; // Target node
                    let u = edge[0] as usize; // Source node (edge[0] is added to edge[1]'s adjacency)
                    adj[v].push(u);
                }
            }

            let mut vis = vec![false; n_usize];
            let mut q = VecDeque::new();
            vis[0] = true;
            q.push_back(0);

            // BFS to check all nodes reachable from node 0
            while let Some(sn) = q.pop_front() {
                for &fn_node in &adj[sn] {
                    if !vis[fn_node] {
                        vis[fn_node] = true;
                        q.push_back(fn_node);
                    }
                }
            }

            // Check if all nodes are visited
            vis.iter().all(|&b| b)
        };

        // Find maximum edge weight
        let mut mx = 0;
        for edge in edges.iter() {
            mx = mx.max(edge[2]);
        }
        if !check(mx) {
            return -1; // If even the maximum weight can't connect all nodes
        }

        // Binary search for minimal lim
        let (mut head, mut tail) = (0i32, mx);
        while head < tail {
            let mid = (head + tail) / 2;
            if check(mid) {
                tail = mid;
            } else {
                head = mid + 1;
            }
        }
        head
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read first line: n and threshold
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<_> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = parts[0];
    let threshold = parts[1];

    // Read edges: n lines each with 3 integers
    let mut edges = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<_> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        edges.push(vec![nums[0], nums[1], nums[2]]);
    }

    let solution = Solution {};
    println!("{}", solution.min_max_weight(n, edges, threshold));
}