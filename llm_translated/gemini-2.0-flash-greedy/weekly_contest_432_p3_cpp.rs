use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn min_max_weight(&self, n: usize, edges: &Vec<Vec<i32>>, _threshold: i32) -> i32 {
        // Define a closure to check if a given limit allows reaching all nodes
        let check = |lim: i32| -> bool {
            let mut e: Vec<Vec<usize>> = vec![vec![]; n];
            for edge in edges {
                if edge[2] <= lim {
                    e[edge[1] as usize].push(edge[0] as usize);
                }
            }

            let mut vis: Vec<bool> = vec![false; n];
            let mut q: VecDeque<usize> = VecDeque::new();
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

            for i in 0..n {
                if !vis[i] {
                    return false;
                }
            }
            true
        };

        // Find the maximum edge weight
        let mut mx: i32 = 0;
        for edge in edges {
            mx = std::cmp::max(mx, edge[2]);
        }

        // If it's not possible to reach all nodes with the maximum weight, return -1
        if !check(mx) {
            return -1;
        }

        // Binary search to find the minimum weight that allows reaching all nodes
        let mut head: i32 = 0;
        let mut tail: i32 = mx;
        while head < tail {
            let mid: i32 = (head + tail) / 2;
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
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    let n: usize = n_str.trim().parse().unwrap();

    let mut threshold_str = String::new();
    io::stdin().read_line(&mut threshold_str).unwrap();
    let threshold: i32 = threshold_str.trim().parse().unwrap();

    let mut edges: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let mut edge_str = String::new();
        io::stdin().read_line(&mut edge_str).unwrap();
        let edge: Vec<i32> = edge_str
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        edges.push(edge);
    }

    let solution = Solution {};
    println!("{}", solution.min_max_weight(n, &edges, threshold));
}