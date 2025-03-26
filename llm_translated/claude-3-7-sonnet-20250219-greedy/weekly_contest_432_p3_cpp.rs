use std::collections::VecDeque;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn min_max_weight(n: i32, edges: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let n = n as usize;
        
        // Closure to check if all nodes are reachable with edges of weight <= lim
        let check = |lim: i32| -> bool {
            // Create adjacency list for edges with weight <= lim
            let mut e: Vec<Vec<usize>> = vec![Vec::new(); n];
            for edge in &edges {
                if edge[2] <= lim {
                    e[edge[1] as usize].push(edge[0] as usize);
                }
            }

            // BFS to check if all nodes are reachable
            let mut vis = vec![false; n];
            let mut q = VecDeque::new();
            q.push_back(0);
            vis[0] = true;
            
            while !q.is_empty() {
                let sn = q.pop_front().unwrap();
                for &fn_node in &e[sn] {
                    if !vis[fn_node] {
                        q.push_back(fn_node);
                        vis[fn_node] = true;
                    }
                }
            }

            // Check if all nodes are visited
            for i in 0..n {
                if !vis[i] {
                    return false;
                }
            }
            true
        };

        // Find maximum edge weight
        let mut mx = 0;
        for edge in &edges {
            mx = mx.max(edge[2]);
        }
        
        // If not all nodes are reachable even with maximum weight, return -1
        if !check(mx) {
            return -1;
        }

        // Binary search for the minimum maximum weight
        let mut head = 0;
        let mut tail = mx;
        while head < tail {
            let mid = (head + tail) >> 1;
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
    
    // Read n
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read threshold
    let threshold: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read edges
    let mut edges = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let edge: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        edges.push(edge);
    }
    
    // Solve and print result
    let solution = Solution::min_max_weight(n, edges, threshold);
    println!("{}", solution);
}