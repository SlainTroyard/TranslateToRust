use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and threshold
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let threshold: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    // Read edges
    let mut edges = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let edge: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        edges.push(edge);
    }

    // Create solution and compute result
    let solution = Solution;
    let result = solution.min_max_weight(n, &edges, threshold);

    // Print the result
    println!("{}", result);
}

struct Solution;

impl Solution {
    fn min_max_weight(&self, n: usize, edges: &Vec<Vec<i32>>, threshold: i32) -> i32 {
        // Define the check function
        let check = |lim: i32| {
            let mut e = vec![Vec::new(); n];
            for edge in edges {
                if edge[2] <= lim {
                    e[edge[1] as usize].push(edge[0]);
                }
            }

            let mut vis = vec![false; n];
            let mut q = VecDeque::new();
            q.push_back(0);
            vis[0] = true;

            while let Some(sn) = q.pop_front() {
                for &fn_ in &e[sn as usize] {
                    if !vis[fn_ as usize] {
                        vis[fn_ as usize] = true;
                        q.push_back(fn_);
                    }
                }
            }

            vis.iter().all(|&x| x)
        };

        // Find the maximum weight in edges
        let mx = edges.iter().map(|edge| edge[2]).max().unwrap_or(0);
        if !check(mx) {
            return -1;
        }

        // Binary search for the minimum maximum weight
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

        head
    }
}