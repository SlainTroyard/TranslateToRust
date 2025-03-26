use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn min_max_weight(&self, n: usize, edges: Vec<Vec<i32>>, _threshold: i32) -> i32 {
        let check = |lim: i32| -> bool {
            let mut e: Vec<Vec<usize>> = vec![Vec::new(); n];
            for edge in &edges {
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

        let mut mx = 0;
        for edge in &edges {
            mx = std::cmp::max(mx, edge[2]);
        }

        if !check(mx) {
            return -1;
        }

        let mut head = 0;
        let mut tail = mx;

        while head < tail {
            let mid = (head + tail) / 2;
            if check(mid) {
                tail = mid;
            } else {
                head = mid + 1;
            }
        }
        return head;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
    let threshold = lines.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut edges: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let edge: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        edges.push(edge);
    }

    let solution = Solution {};
    println!("{}", solution.min_max_weight(n, edges, threshold));
}