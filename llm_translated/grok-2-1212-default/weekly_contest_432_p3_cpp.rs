use std::io::{self, BufRead};
use std::collections::VecDeque;

struct Solution;

impl Solution {
    fn min_max_weight(n: i32, edges: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let n = n as usize;
        let check = |lim: i32| -> bool {
            let mut e = vec![vec![]; n];
            for edge in &edges {
                if edge[2] <= lim {
                    e[edge[1] as usize].push(edge[0] as usize);
                }
            }

            let mut vis = vec![false; n];
            let mut q = VecDeque::new();
            q.push_back(0);
            vis[0] = true;
            while let Some(sn) = q.pop_front() {
                for &fn_ in &e[sn] {
                    if !vis[fn_] {
                        q.push_back(fn_);
                        vis[fn_] = true;
                    }
                }
            }

            vis.into_iter().all(|v| v)
        };

        let mx = edges.iter().map(|edge| edge[2]).max().unwrap();
        if !check(mx) {
            return -1;
        }

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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: i32 = lines.next().unwrap()?.trim().parse().unwrap();
    let threshold: i32 = lines.next().unwrap()?.trim().parse().unwrap();

    let mut edges = Vec::new();
    for _ in 0..n {
        let edge: Vec<i32> = lines.next().unwrap()?
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges.push(edge);
    }

    let result = Solution::min_max_weight(n, edges, threshold);
    println!("{}", result);

    Ok(())
}