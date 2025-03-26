use std::collections::VecDeque;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn construct_grid_layout(n: usize, edges: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        // Build adjacency list
        let mut g = vec![Vec::new(); n];
        for edge in edges {
            let x = edge[0];
            let y = edge[1];
            g[x].push(y);
            g[y].push(x);
        }

        // Find the node with the smallest degree
        let mut x = 0;
        for i in 0..g.len() {
            if g[i].len() < g[x].len() {
                x = i;
            }
        }

        // Construct the first row
        let mut row = vec![x];
        let mut vis = vec![false; n];
        vis[x] = true;
        let deg_st = g[x].len();
        loop {
            let mut nxt = None;
            for &y in &g[x] {
                if !vis[y] && (nxt.is_none() || g[y].len() < g[nxt.unwrap()].len()) {
                    nxt = Some(y);
                }
            }
            if let Some(next_node) = nxt {
                x = next_node;
                row.push(x);
                vis[x] = true;
            } else {
                break;
            }
            if g[x].len() <= deg_st {
                break;
            }
        }

        // Construct the grid layout
        let mut ans = vec![Vec::new(); n / row.len()];
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

        ans
    }
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let n: usize = first_iter.next().unwrap().parse().unwrap();
    let edges_size: usize = first_iter.next().unwrap().parse().unwrap();

    let mut edges = Vec::new();
    for _ in 0..edges_size {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let u: usize = iter.next().unwrap().parse().unwrap();
        let v: usize = iter.next().unwrap().parse().unwrap();
        edges.push(vec![u, v]);
    }

    // Solve the problem
    let solution = Solution;
    let result = solution.construct_grid_layout(n, edges);

    // Print the result
    for row in result {
        for (i, &x) in row.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", x);
        }
        println!();
    }
}