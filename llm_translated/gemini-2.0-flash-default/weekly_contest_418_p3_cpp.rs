use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn construct_grid_layout(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut g: Vec<Vec<usize>> = vec![vec![]; n];
        for e in &edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            g[x].push_back(y);
            g[y].push_back(x);
        }

        let mut x: usize = 0;
        for i in 0..g.len() {
            if g[i].len() < g[x].len() {
                x = i;
            }
        }

        let mut row: Vec<usize> = vec![x];
        let mut vis: Vec<bool> = vec![false; n];
        vis[x] = true;
        let deg_st = g[x].len();
        loop {
            let mut nxt: Option<usize> = None;
            for &y in &g[x] {
                if !vis[y] {
                    if nxt.is_none() || g[y].len() < g[nxt.unwrap()].len() {
                        nxt = Some(y);
                    }
                }
            }
            if nxt.is_none() {
                break;
            }
            x = nxt.unwrap();
            row.push_back(x);
            vis[x] = true;
            if g[x].len() <= deg_st {
                break;
            }
        }

        let mut ans: Vec<Vec<usize>> = vec![vec![]; n / row.len()];
        ans[0] = row;
        for i in 1..ans.len() {
            for &x in &ans[i - 1] {
                for &y in &g[x] {
                    if !vis[y] {
                        vis[y] = true;
                        ans[i].push_back(y);
                        break;
                    }
                }
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let mut iter = line.trim().split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let edges_size: i32 = iter.next().unwrap().parse().unwrap();

    let mut edges: Vec<Vec<i32>> = Vec::new();
    for _ in 0..edges_size {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let nums: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges.push(nums);
    }

    let sol = Solution {};
    let res = sol.construct_grid_layout(n, edges);

    for row in &res {
        for &x in row {
            print!("{} ", x);
        }
        println!();
    }

    Ok(())
}