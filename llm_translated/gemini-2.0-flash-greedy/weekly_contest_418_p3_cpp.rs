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
            g[x].push(y);
            g[y].push(x);
        }

        let mut x = 0;
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

            match nxt {
                Some(next_node) => {
                    x = next_node;
                    row.push(x);
                    vis[x] = true;
                    if g[x].len() <= deg_st {
                        break;
                    }
                }
                None => break,
            }
        }

        let mut ans: Vec<Vec<usize>> = vec![vec![]; n / row.len()];
        ans[0] = row;

        for i in 1..ans.len() {
            for &x in &ans[i - 1] {
                for &y in &g[x] {
                    if !vis[y] {
                        vis[y] = true;
                        ans[i].push(y);
                        break;
                    }
                }
            }
        }

        ans.iter().map(|r| r.iter().map(|&x| x as i32).collect()).collect()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut first_split = first_line.split_whitespace();
    let n: i32 = first_split.next().unwrap().parse().unwrap();
    let edges_size: i32 = first_split.next().unwrap().parse().unwrap();

    let mut edges: Vec<Vec<i32>> = Vec::new();
    for _ in 0..edges_size {
        let line = lines.next().unwrap();
        let mut split = line.split_whitespace();
        let u: i32 = split.next().unwrap().parse().unwrap();
        let v: i32 = split.next().unwrap().parse().unwrap();
        edges.push(vec![u, v]);
    }

    let sol = Solution {};
    let res = sol.construct_grid_layout(n, edges);

    for row in &res {
        for &x in row {
            print!("{} ", x);
        }
        println!();
    }
}