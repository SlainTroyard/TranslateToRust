use std::collections::VecDeque;
use std::io::{self, BufRead};

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
            for &fn_ in &e[sn] {
                if !vis[fn_] {
                    q.push_back(fn_);
                    vis[fn_] = true;
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

    let mx = edges.iter().map(|edge| edge[2]).max().unwrap_or(0);
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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let threshold: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut edges: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let edge: Vec<i32> = line.split_whitespace()
                                 .map(|x| x.parse().unwrap())
                                 .collect();
        edges.push(edge);
    }

    let solution = min_max_weight(n, &edges, threshold);
    println!("{}", solution);
}