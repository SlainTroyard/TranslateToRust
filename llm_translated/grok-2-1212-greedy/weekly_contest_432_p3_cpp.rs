use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn min_max_weight(n: i32, edges: &Vec<Vec<i32>>, threshold: i32) -> i32 {
        let check = |lim: i32| -> bool {
            let mut e: Vec<Vec<i32>> = vec![Vec::new(); n as usize];
            for edge in edges {
                if edge[2] <= lim {
                    e[edge[1] as usize].push(edge[0]);
                }
            }

            let mut vis = vec![false; n as usize];
            let mut q: Vec<i32> = Vec::new();
            q.push(0);
            vis[0] = true;
            while !q.is_empty() {
                let sn = q.remove(0);
                for &fn_ in &e[sn as usize] {
                    if !vis[fn_ as usize] {
                        q.push(fn_);
                        vis[fn_ as usize] = true;
                    }
                }
            }

            for i in 0..n as usize {
                if !vis[i] {
                    return false;
                }
            }
            true
        };

        let mut mx = 0;
        for edge in edges {
            mx = mx.max(edge[2]);
        }
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

    // Read n
    let n: i32 = lines.next().unwrap()?.trim().parse().unwrap();

    // Read threshold
    let threshold: i32 = lines.next().unwrap()?.trim().parse().unwrap();

    // Read edges
    let mut edges = Vec::new();
    for _ in 0..n {
        let edge: Vec<i32> = lines
            .next()
            .unwrap()?
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges.push(edge);
    }

    // Calculate and print result
    let result = Solution::min_max_weight(n, &edges, threshold);
    println!("{}", result);

    Ok(())
}