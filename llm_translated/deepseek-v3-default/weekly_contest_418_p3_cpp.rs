use std::collections::VecDeque;
use std::io;

fn main() {
    // Read input values
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid input");
    let edges_size: usize = iter.next().unwrap().parse().expect("Invalid input");

    // Read edges
    let mut edges: Vec<Vec<usize>> = Vec::with_capacity(edges_size);
    for _ in 0..edges_size {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        let x: usize = iter.next().unwrap().parse().expect("Invalid input");
        let y: usize = iter.next().unwrap().parse().expect("Invalid input");
        edges.push(vec![x, y]);
    }

    // Construct grid layout
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
    // Build adjacency list
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
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

    // Continue building the row
    loop {
        let mut nxt = None;
        for &y in &g[x] {
            if !vis[y] && (nxt.is_none() || g[y].len() < g[nxt.unwrap()].len()) {
                nxt = Some(y);
            }
        }
        if let Some(next_x) = nxt {
            x = next_x;
            row.push(x);
            vis[x] = true;
        } else {
            break;
        }
        if g[x].len() <= deg_st {
            break;
        }
    }

    // Build the rest of the grid
    let mut ans: Vec<Vec<usize>> = vec![Vec::new(); n / row.len()];
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