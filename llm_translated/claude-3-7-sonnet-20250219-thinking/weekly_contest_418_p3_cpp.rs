use std::io::{self, BufRead};

struct Solution {}

impl Solution {
    fn construct_grid_layout(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        // Create adjacency list representation of graph
        let mut g: Vec<Vec<usize>> = vec![vec![]; n];
        for e in &edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            g[x].push(y);
            g[y].push(x);
        }

        // Find vertex with minimum degree
        let mut x = 0;
        for i in 0..n {
            if g[i].len() < g[x].len() {
                x = i;
            }
        }

        // Construct first row
        let mut row = vec![x as i32];
        let mut vis = vec![false; n];
        vis[x] = true;
        let deg_st = g[x].len();
        
        // Do-while loop from C++
        loop {
            let mut nxt = None;
            for &y in &g[x] {
                if !vis[y] && (nxt.is_none() || g[y].len() < g[nxt.unwrap()].len()) {
                    nxt = Some(y);
                }
            }
            
            // Must have a next node (per original algorithm)
            let next = nxt.unwrap();
            x = next;
            row.push(x as i32);
            vis[x] = true;
            
            // Exit condition
            if g[x].len() <= deg_st {
                break;
            }
        }

        // Create grid layout
        let rows_count = n / row.len();
        let mut ans = vec![vec![]; rows_count];
        ans[0] = row;
        
        // Construct remaining rows
        for i in 1..rows_count {
            for &x in &ans[i - 1] {
                let x = x as usize;
                for &y in &g[x] {
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
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse n and edgesSize
    let first_line = lines.next().unwrap().unwrap();
    let inputs: Vec<i32> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = inputs[0];
    let edges_size = inputs[1] as usize;
    
    // Read edges
    let mut edges = vec![vec![0; 2]; edges_size];
    for i in 0..edges_size {
        let line = lines.next().unwrap().unwrap();
        let edge: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        edges[i][0] = edge[0];
        edges[i][1] = edge[1];
    }
    
    // Solve and output
    let res = Solution::construct_grid_layout(n, edges);
    
    for row in res {
        for (i, &x) in row.iter().enumerate() {
            print!("{}", x);
            if i < row.len() - 1 {
                print!(" ");
            }
        }
        println!();
    }
}