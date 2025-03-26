use std::io::{self, BufRead};

fn construct_grid_layout(n: usize, edges: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut g = vec![Vec::new(); n];
    for e in edges {
        let x = e[0];
        let y = e[1];
        g[x].push(y);
        g[y].push(x);
    }

    let mut x = 0;
    for i in 0..g.len() {
        if g[i].len() < g[x].len() {
            x = i;
        }
    }

    let mut row = vec![x];
    let mut vis = vec![false; n];
    vis[x] = true;
    let deg_st = g[x].len();
    loop {
        let mut nxt = usize::MAX;
        for &y in &g[x] {
            if !vis[y] && (nxt == usize::MAX || g[y].len() < g[nxt].len()) {
                nxt = y;
            }
        }
        if nxt == usize::MAX {
            break;
        }
        x = nxt;
        row.push(x);
        vis[x] = true;
        if g[x].len() <= deg_st {
            break;
        }
    }

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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line: Vec<usize> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = first_line[0];
    let edges_size = first_line[1];

    let mut edges = Vec::new();
    for _ in 0..edges_size {
        let edge: Vec<usize> = lines.next().unwrap()?.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges.push(edge);
    }

    let res = construct_grid_layout(n, &edges);

    for row in res {
        for x in row {
            print!("{} ", x);
        }
        println!();
    }

    Ok(())
}