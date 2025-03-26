use std::io;
use std::io::BufRead;
use std::collections::VecDeque;
use std::cmp::max;

struct Solution {}

impl Solution {
    fn min_max_weight(&self, n: i32, edges: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let check = |lim: i32| -> bool {
            let n_usize = n as usize;
            let mut e: Vec<Vec<i32>> = vec![Vec::new(); n_usize];
            for edge in &edges {
                if edge[2] <= lim {
                    e[edge[1] as usize].push(edge[0]);
                }
            }

            let mut vis: Vec<bool> = vec![false; n_usize];
            let mut q: VecDeque<i32> = VecDeque::new();
            q.push_back(0);
            vis[0] = true;
            while !q.is_empty() {
                let sn = q.pop_front().unwrap();
                for &fn_node in &e[sn as usize] {
                    if !vis[fn_node as usize] {
                        q.push_back(fn_node);
                        vis[fn_node as usize] = true;
                    }
                }
            }

            for i in 0..n_usize {
                if !vis[i] {
                    return false;
                }
            }
            true
        };

        let mut mx = 0;
        for edge in &edges {
            mx = max(mx, edge[2]);
        }

        if !check(mx) {
            return -1;
        }

        let mut head = 0;
        let mut tail = mx;
        while head < tail {
            let mid = head + (tail - head) / 2;
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

    let n: i32 = lines.next().unwrap().expect("Failed to read n").trim().parse().expect("Failed to parse n");
    let threshold: i32 = lines.next().unwrap().expect("Failed to read threshold").trim().parse().expect("Failed to parse threshold");

    let mut edges: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().expect("Failed to read edge line");
        let parts: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse edge value"))
            .collect();
        edges.push(parts);
    }

    let solution = Solution {};
    println!("{}", solution.min_max_weight(n, edges, threshold));
}