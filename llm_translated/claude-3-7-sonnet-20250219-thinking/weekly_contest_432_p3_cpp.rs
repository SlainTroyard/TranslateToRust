use std::collections::VecDeque;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn min_max_weight(n: i32, edges: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let n = n as usize;
        
        let check = |lim: i32| -> bool {
            // Create adjacency list for edges with weight <= lim
            let mut adj_list: Vec<Vec<usize>> = vec![vec![]; n];
            for edge in &edges {
                if edge[2] <= lim {
                    adj_list[edge[1] as usize].push(edge[0] as usize);
                }
            }
            
            // BFS to check if all nodes are reachable
            let mut visited = vec![false; n];
            let mut q = VecDeque::new();
            q.push_back(0);
            visited[0] = true;
            
            while !q.is_empty() {
                let sn = q.pop_front().unwrap();
                for &fn_val in &adj_list[sn] {
                    if !visited[fn_val] {
                        q.push_back(fn_val);
                        visited[fn_val] = true;
                    }
                }
            }
            
            // Check if all nodes were visited
            for i in 0..n {
                if !visited[i] {
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
        
        if !check(mx) {
            return -1;
        }
        
        // Binary search for minimum weight that satisfies condition
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
    
    // Read number of nodes
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read threshold
    let threshold: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read edges
    let mut edges: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let edge: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        edges.push(edge);
    }
    
    let solution = Solution;
    println!("{}", solution.min_max_weight(n, edges, threshold));
}