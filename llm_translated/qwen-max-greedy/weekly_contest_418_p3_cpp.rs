use std::io::{self, BufRead, Write};

fn construct_grid_layout(n: usize, edges: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut g = vec![vec![]; n];
    for e in edges.iter() {
        let x = e[0];
        let y = e[1];
        g[x].push(y);
        g[y].push(x);
    }

    let mut x = 0;
    for (i, neighbors) in g.iter().enumerate() {
        if neighbors.len() < g[x].len() {
            x = i;
        }
    }

    let mut row = vec![x];
    let mut vis = vec![false; n];
    vis[x] = true;
    let deg_st = g[x].len();
    while let Some(next) = g[x].iter().filter(|&&y| !vis[y]).min_by_key(|&y| g[*y].len()) {
        x = *next;
        row.push(x);
        vis[x] = true;
        if g[x].len() <= deg_st {
            break;
        }
    }

    let mut ans = vec![vec![]; n / row.len()];
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

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the number of nodes and edges
    stdin.read_line(&mut buffer).unwrap();
    let input: Vec<usize> = buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = input[0];
    let edges_size = input[1];

    // Read the edges
    let mut edges = vec![vec![0; 2]; edges_size];
    for i in 0..edges_size {
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        edges[i] = buffer
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
    }

    // Construct the grid layout
    let res = construct_grid_layout(n, &edges);

    // Print the result
    for row in res {
        for x in row {
            write!(stdout, "{} ", x).unwrap();
        }
        writeln!(stdout).unwrap();
    }
}