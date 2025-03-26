use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and edgesSize from the first line
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let edges_size: usize = parts.next().unwrap().parse().unwrap();

    // Read edges from the subsequent lines
    let mut edges = Vec::with_capacity(edges_size);
    for _ in 0..edges_size {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let x: usize = parts.next().unwrap().parse().unwrap();
        let y: usize = parts.next().unwrap().parse().unwrap();
        edges.push(vec![x, y]);
    }

    // Construct the grid layout
    let res = construct_grid_layout(n, &edges);

    // Print the result
    for row in res {
        for x in row {
            print!("{} ", x);
        }
        println!();
    }
}

fn construct_grid_layout(n: usize, edges: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    // Build the adjacency list
    let mut g = vec![Vec::new(); n];
    for e in edges {
        let x = e[0];
        let y = e[1];
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

    // Start building the first row
    let mut row = vec![x];
    let mut vis = vec![false; n];
    vis[x] = true;
    let deg_st = g[x].len();

    // Continue building the row until the degree condition is met
    loop {
        let mut nxt = None;
        for &y in &g[x] {
            if !vis[y] && (nxt.is_none() || g[y].len() < g[nxt.unwrap()].len()) {
                nxt = Some(y);
            }
        }
        if let Some(y) = nxt {
            x = y;
            row.push(x);
            vis[x] = true;
            if g[x].len() <= deg_st {
                break;
            }
        } else {
            break;
        }
    }

    // Build the rest of the grid
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