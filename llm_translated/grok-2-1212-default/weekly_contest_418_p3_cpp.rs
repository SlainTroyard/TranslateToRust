use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn construct_grid_layout(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in &edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            g[x].push(y);
            g[y].push(x);
        }

        let mut x = 0;
        for i in 0..g.len() {
            if g[i].len() < g[x].len() {
                x = i;
            }
        }

        let mut row = vec![x as i32];
        let mut vis = vec![false; n];
        vis[x] = true;
        let deg_st = g[x].len();
        loop {
            let mut nxt = -1;
            for &y in &g[x] {
                if !vis[y] && (nxt < 0 || g[y].len() < g[nxt as usize].len()) {
                    nxt = y as i32;
                }
            }
            if nxt < 0 {
                break;
            }
            x = nxt as usize;
            row.push(nxt);
            vis[x] = true;
        }

        let mut ans = vec![vec![]; n / row.len()];
        ans[0] = row;
        for i in 1..ans.len() {
            for &x in &ans[i - 1] {
                for &y in &g[x as usize] {
                    if !vis[y] {
                        vis[y] = true;
                        ans[i].push(y as i32);
                        break;
                    }
                }
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = first_line[0];
    let edges_size = first_line[1];

    let mut edges = Vec::with_capacity(edges_size as usize);
    for _ in 0..edges_size {
        let edge: Vec<i32> = lines.next().unwrap()?.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges.push(edge);
    }

    let res = Solution::construct_grid_layout(n, edges);

    for row in res {
        for x in row {
            print!("{} ", x);
        }
        println!();
    }

    Ok(())
}