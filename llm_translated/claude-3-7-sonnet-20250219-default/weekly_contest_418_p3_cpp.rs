use std::io::{self, BufRead};

struct Solution {}

impl Solution {
    fn construct_grid_layout(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        // Build adjacency list representation of the graph
        let mut g: Vec<Vec<i32>> = vec![vec![]; n];
        for e in &edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            g[x].push(y as i32);
            g[y].push(x as i32);
        }

        // Find the node with minimum degree
        let mut x = 0;
        for i in 0..g.len() {
            if g[i].len() < g[x].len() {
                x = i;
            }
        }

        // Construct the first row
        let mut row = vec![x as i32];
        let mut vis = vec![false; n];
        vis[x] = true;
        let deg_st = g[x].len();
        
        loop {
            let mut nxt = -1;
            for &y in &g[x] {
                let y = y as usize;
                if !vis[y] && (nxt < 0 || g[y].len() < g[nxt as usize].len()) {
                    nxt = y as i32;
                }
            }
            if nxt < 0 {
                break;
            }
            x = nxt as usize;
            row.push(x as i32);
            vis[x] = true;
            if g[x].len() <= deg_st {
                break;
            }
        }

        // Construct the remaining rows
        let row_len = row.len();
        let mut ans = vec![vec![]; n / row_len];
        ans[0] = row;
        
        for i in 1..ans.len() {
            let prev_row = ans[i - 1].clone();
            for &x in &prev_row {
                let x = x as usize;
                for &y in &g[x] {
                    let y = y as usize;
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
    
    // Read n and edgesSize
    let first_line = lines.next().unwrap()?;
    let mut parts = first_line.split_whitespace();
    let n: i32 = parts.next().unwrap().parse().unwrap();
    let edges_size: usize = parts.next().unwrap().parse().unwrap();
    
    // Read edges
    let mut edges = vec![vec![0; 2]; edges_size];
    for i in 0..edges_size {
        let line = lines.next().unwrap()?;
        let mut parts = line.split_whitespace();
        edges[i][0] = parts.next().unwrap().parse().unwrap();
        edges[i][1] = parts.next().unwrap().parse().unwrap();
    }
    
    // Solve the problem
    let sol = Solution {};
    let res = Solution::construct_grid_layout(n, edges);
    
    // Print the result
    for row in res {
        for (i, &x) in row.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", x);
        }
        println!();
    }
    
    Ok(())
}