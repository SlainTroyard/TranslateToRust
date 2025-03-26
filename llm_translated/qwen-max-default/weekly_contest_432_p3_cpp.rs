use std::io::{self, BufRead, Write};
use std::collections::VecDeque;

struct Solution;

impl Solution {
    fn min_max_weight(n: usize, edges: &Vec<Vec<i32>>, threshold: i32) -> i32 {
        let check = |lim: i32| -> bool {
            let mut e: Vec<Vec<usize>> = vec![Vec::new(); n];
            for edge in edges.iter() {
                if edge[2] <= lim {
                    e[edge[1] as usize].push(edge[0] as usize);
                }
            }

            let mut vis: Vec<bool> = vec![false; n];
            let mut q: VecDeque<usize> = VecDeque::new();
            q.push_back(0);
            vis[0] = true;
            while let Some(sn) = q.pop_front() {
                for &fn in &e[sn] {
                    if !vis[fn] {
                        q.push_back(fn);
                        vis[fn] = true;
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
        for edge in edges.iter() {
            mx = mx.max(edge[2]);
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
        head
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut input = String::new();
    stdin_lock.read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Failed to parse n");

    input.clear();
    stdin_lock.read_line(&mut input).expect("Failed to read line");
    let threshold: i32 = input.trim().parse().expect("Failed to parse threshold");

    let mut edges: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        input.clear();
        stdin_lock.read_line(&mut input).expect("Failed to read line");
        let edge: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse edge"))
            .collect();
        edges.push(edge);
    }

    let solution = Solution;
    let result = solution.min_max_weight(n, &edges, threshold);
    writeln!(stdout_lock, "{}", result).expect("Failed to write result");
}