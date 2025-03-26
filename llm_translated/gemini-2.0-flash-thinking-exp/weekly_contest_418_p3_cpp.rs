// Problem: Weekly Contest 418 Problem 3
use std::io;
use std::io::BufRead;

fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap()?;
    let mut first_line_iter = first_line.split_whitespace();
    let n: usize = first_line_iter.next().unwrap().parse()?;
    let edges_size: usize = first_line_iter.next().unwrap().parse()?;

    let mut edges: Vec<Vec<i32>> = Vec::with_capacity(edges_size);
    for _ in 0..edges_size {
        let line = lines.next().unwrap()?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges.push(nums);
    }

    let sol = Solution {};
    let res = sol.construct_grid_layout(n, &mut edges);

    for row in res {
        for (i, x) in row.iter().enumerate() {
            print!("{}", x);
            if i < row.len() - 1 {
                print!(" ");
            }
        }
        println!();
    }

    Ok(())
}

struct Solution {}

impl Solution {
    pub fn construct_grid_layout(
        &self,
        n: usize,
        edges: &mut Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut g: Vec<Vec<i32>> = vec![Vec::new(); n];
        for e in edges.iter() {
            let x = e[0] as usize;
            let y = e[1] as usize;
            g[x].push(y as i32);
            g[y].push(x as i32);
        }

        let mut x: usize = 0;
        for i in 0..g.len() {
            if g[i].len() < g[x].len() {
                x = i;
            }
        }

        let mut row: Vec<i32> = vec![x as i32];
        let mut vis: Vec<bool> = vec![false; n];
        vis[x] = true;
        let deg_st = g[x].len();

        loop {
            let mut nxt: i32 = -1;
            for &y_usize in g[x].iter() {
                let y = y_usize as usize;
                if !vis[y] {
                    if nxt == -1 || g[y].len() < g[nxt as usize].len() {
                        nxt = y as i32;
                    }
                }
            }
            if nxt == -1 {
                break; // Should not happen based on original logic, but added for safety
            }
            x = nxt as usize;
            row.push(x as i32);
            vis[x] = true;
            if g[x].len() <= deg_st {
                break;
            }
        }

        let mut ans: Vec<Vec<i32>> = vec![Vec::new(); n / row.len()];
        ans[0] = row;
        for i in 1..ans.len() {
            for &x_val in ans[i - 1].iter() {
                let x_usize = x_val as usize;
                for &y_usize in g[x_usize].iter() {
                    let y = y_usize as usize;
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

fn main() {
    solve().unwrap();
}